use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use rust_decimal::Decimal;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct InventarioItem {
    pub id_inventario: i32,
    pub id_producto_detalle: i32,
    pub nombre: String,
    pub sku: String,
    pub marca: String,
    pub categoria: String,
    pub cantidad_disponible: i32,
    pub cantidad_minima: i32,
    pub cantidad_maxima: Option<i32>,
    pub ubicacion_fisica: Option<String>,
    pub precio_venta: Decimal,
    pub imagen_principal: Option<String>,
    pub fecha_actualizacion: Option<NaiveDateTime>,
    pub stock_estado: String, // "ok", "bajo", "agotado"
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct InventarioStats {
    pub total_productos: i64,
    pub stock_bajo: i64,
    pub sin_stock: i64,
    pub valor_total: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventarioResponse {
    pub items: Vec<InventarioItem>,
    pub total_count: i64,
}

#[derive(Debug, Deserialize)]
pub struct InventarioQuery {
    pub search: Option<String>,
    pub stock_estado: Option<String>, // "ok", "bajo", "agotado", "todos"
    pub marca: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

pub async fn get_inventario(
    State(pool): State<PgPool>,
    Query(query): Query<InventarioQuery>,
) -> Result<Json<InventarioResponse>, StatusCode> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    let offset = (page - 1) * limit;

    let mut sql = String::from(
        r#"
        SELECT 
            i.id_inventario,
            i.id_producto_detalle,
            pd.nombre,
            pd.sku,
            m.nombre as marca,
            c.nombre as categoria,
            i.cantidad_disponible,
            i.cantidad_minima,
            i.cantidad_maxima,
            i.ubicacion_fisica,
            pd.precio_venta,
            pd.imagen_principal,
            i.fecha_actualizacion,
            CASE 
                WHEN i.cantidad_disponible = 0 THEN 'agotado'
                WHEN i.cantidad_disponible < 10 THEN 'bajo'
                ELSE 'ok'
            END as stock_estado
        FROM inventario i
        JOIN producto_detalle pd ON i.id_producto_detalle = pd.id_producto_detalle
        JOIN producto p ON pd.id_producto = p.id_producto
        JOIN categoria c ON p.id_categoria = c.id_categoria
        JOIN marca m ON pd.id_marca = m.id_marca
        WHERE 1=1
        "#,
    );

    let mut count_sql = String::from(
        r#"
        SELECT COUNT(*) as count
        FROM inventario i
        JOIN producto_detalle pd ON i.id_producto_detalle = pd.id_producto_detalle
        JOIN producto p ON pd.id_producto = p.id_producto
        JOIN categoria c ON p.id_categoria = c.id_categoria
        JOIN marca m ON pd.id_marca = m.id_marca
        WHERE 1=1
        "#,
    );

    // Search filter
    if let Some(search) = &query.search {
        if !search.is_empty() {
            let filter = format!(
                " AND (pd.nombre ILIKE '%{}%' OR pd.sku ILIKE '%{}%')",
                search, search
            );
            sql.push_str(&filter);
            count_sql.push_str(&filter);
        }
    }

    // Stock status filter
    if let Some(estado) = &query.stock_estado {
        let filter = match estado.as_str() {
            "bajo" => " AND i.cantidad_disponible < 10 AND i.cantidad_disponible > 0",
            "agotado" => " AND i.cantidad_disponible = 0",
            "ok" => " AND i.cantidad_disponible >= 10",
            _ => "" // "todos" or any other value - no filter
        };
        sql.push_str(filter);
        count_sql.push_str(filter);
    }

    // Brand filter
    if let Some(marca) = &query.marca {
        if marca != "Todas" && !marca.is_empty() {
            let filter = format!(" AND m.nombre = '{}'", marca);
            sql.push_str(&filter);
            count_sql.push_str(&filter);
        }
    }

    sql.push_str(" ORDER BY i.fecha_actualizacion DESC NULLS LAST");
    sql.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));

    // Get total count
    let total_count: (i64,) = match sqlx::query_as(&count_sql)
        .fetch_one(&pool)
        .await
    {
        Ok(count) => count,
        Err(e) => {
            eprintln!("Error fetching count: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Get items
    let items = match sqlx::query_as::<_, InventarioItem>(&sql)
        .fetch_all(&pool)
        .await
    {
        Ok(items) => items,
        Err(e) => {
            eprintln!("Error fetching inventario: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(Json(InventarioResponse {
        items,
        total_count: total_count.0,
    }))
}

pub async fn get_inventario_stats(
    State(pool): State<PgPool>,
) -> Result<Json<InventarioStats>, StatusCode> {
    let stats_query = r#"
        SELECT 
            COUNT(*) as total_productos,
            COUNT(*) FILTER (WHERE i.cantidad_disponible < 10 AND i.cantidad_disponible > 0) as stock_bajo,
            COUNT(*) FILTER (WHERE i.cantidad_disponible = 0) as sin_stock,
            COALESCE(SUM(i.cantidad_disponible * pd.precio_venta), 0) as valor_total
        FROM inventario i
        JOIN producto_detalle pd ON i.id_producto_detalle = pd.id_producto_detalle
    "#;

    match sqlx::query_as::<_, InventarioStats>(stats_query)
        .fetch_one(&pool)
        .await
    {
        Ok(stats) => Ok(Json(stats)),
        Err(e) => {
            eprintln!("Error fetching inventario stats: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
