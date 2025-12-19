use axum::{
    extract::{Query, State, Path},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use rust_decimal::Decimal;
use chrono::NaiveDateTime;
use uuid::Uuid;

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

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ProductSearchResult {
    pub id_producto_detalle: i32,
    pub nombre: String,
    pub sku: String,
    pub marca: String,
    pub stock_actual: i32,
    pub imagen_principal: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EntradaStockRequest {
    pub id_producto_detalle: i32,
    pub cantidad: i32,
    pub ubicacion_fisica: Option<String>,
    pub lote: Option<String>,
    pub fecha_vencimiento: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AjusteInventarioRequest {
    pub cantidad_disponible: i32,
    pub cantidad_minima: i32,
    pub cantidad_maxima: Option<i32>,
    pub ubicacion_fisica: Option<String>,
    pub motivo: String,
    pub imagen_principal: Option<String>, // Base64 image string
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

pub async fn search_products(
    State(pool): State<PgPool>,
    Query(query): Query<InventarioQuery>,
) -> Result<Json<Vec<ProductSearchResult>>, StatusCode> {
    let search_term = query.search.unwrap_or_default();
    
    let sql = r#"
        SELECT 
            pd.id_producto_detalle,
            pd.nombre,
            pd.sku,
            m.nombre as marca,
            COALESCE(i.cantidad_disponible, 0) as stock_actual,
            pd.imagen_principal
        FROM producto_detalle pd
        JOIN marca m ON pd.id_marca = m.id_marca
        LEFT JOIN inventario i ON pd.id_producto_detalle = i.id_producto_detalle
        WHERE pd.nombre ILIKE $1 OR pd.sku ILIKE $1
        LIMIT 10
    "#;

    let search_pattern = format!("%{}%", search_term);

    match sqlx::query_as::<_, ProductSearchResult>(sql)
        .bind(&search_pattern)
        .fetch_all(&pool)
        .await
    {
        Ok(results) => Ok(Json(results)),
        Err(e) => {
            eprintln!("Error searching products: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn add_stock_entry(
    State(pool): State<PgPool>,
    Json(payload): Json<EntradaStockRequest>,
) -> Result<StatusCode, StatusCode> {
    use chrono::NaiveDate;

    // Parse fecha_vencimiento if provided
    let fecha_venc = if let Some(fecha_str) = &payload.fecha_vencimiento {
        NaiveDate::parse_from_str(fecha_str, "%Y-%m-%d").ok()
    } else {
        None
    };

    // Start a transaction
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("Error starting transaction: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Get current stock and id_inventario
    let current_stock: (i32, i32) = match sqlx::query_as(
        "SELECT cantidad_disponible, id_inventario FROM inventario WHERE id_producto_detalle = $1"
    )
    .bind(payload.id_producto_detalle)
    .fetch_one(&mut *tx)
    .await
    {
        Ok(stock) => stock,
        Err(e) => {
            eprintln!("Error fetching current stock: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let cantidad_anterior = current_stock.0;
    let id_inventario = current_stock.1;
    let cantidad_nueva = cantidad_anterior + payload.cantidad;

    // Update inventory
    let update_query = r#"
        UPDATE inventario
        SET 
            cantidad_disponible = cantidad_disponible + $1,
            ubicacion_fisica = COALESCE($2, ubicacion_fisica),
            lote = COALESCE($3, lote),
            fecha_vencimiento = COALESCE($4, fecha_vencimiento),
            fecha_ultima_entrada = CURRENT_DATE,
            fecha_actualizacion = CURRENT_TIMESTAMP
        WHERE id_producto_detalle = $5
    "#;

    if let Err(e) = sqlx::query(update_query)
        .bind(payload.cantidad)
        .bind(&payload.ubicacion_fisica)
        .bind(&payload.lote)
        .bind(fecha_venc)
        .bind(payload.id_producto_detalle)
        .execute(&mut *tx)
        .await
    {
        eprintln!("Error updating inventory: {:?}", e);
        let _ = tx.rollback().await;
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    // Create movement record
    let insert_movement = r#"
        INSERT INTO movimiento_inventario (
            id_inventario,
            id_producto_detalle,
            tipo_movimiento,
            cantidad,
            cantidad_anterior,
            cantidad_nueva,
            motivo,
            id_usuario,
            fecha_movimiento,
            notas,
            documento_referencia
        ) VALUES ($1, $2, $3::tipo_movimiento_inventario, $4, $5, $6, $7, $8, CURRENT_TIMESTAMP, $9, $10)
    "#;

    if let Err(e) = sqlx::query(insert_movement)
        .bind(id_inventario)
        .bind(payload.id_producto_detalle)
        .bind("entrada")
        .bind(payload.cantidad)
        .bind(cantidad_anterior)
        .bind(cantidad_nueva)
        .bind("Entrada de stock manual")
        .bind(1) // TODO: Get actual user ID from session
        .bind(Option::<String>::None) // notas
        .bind(Option::<String>::None) // documento_referencia
        .execute(&mut *tx)
        .await
    {
        eprintln!("Error creating movement record: {:?}", e);
        let _ = tx.rollback().await;
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    // Commit transaction
    match tx.commit().await {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => {
            eprintln!("Error committing transaction: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct MovimientoHistorial {
    pub id_movimiento: i32,
    pub tipo_movimiento: String,
    pub cantidad: i32,
    pub cantidad_anterior: i32,
    pub cantidad_nueva: i32,
    pub motivo: Option<String>,
    pub usuario: Option<String>,
    pub fecha_movimiento: Option<NaiveDateTime>,
    pub notas: Option<String>,
    pub documento_referencia: Option<String>,
}

pub async fn get_historial_inventario(
    State(pool): State<PgPool>,
    axum::extract::Path(id_producto_detalle): axum::extract::Path<i32>,
) -> Result<Json<Vec<MovimientoHistorial>>, StatusCode> {
    let rows = sqlx::query!(
        r#"
        SELECT 
            m.id_movimiento,
            m.tipo_movimiento::text as tipo_movimiento,
            m.cantidad,
            m.cantidad_anterior,
            m.cantidad_nueva,
            m.motivo,
            u.nombre as "usuario?",
            m.fecha_movimiento,
            m.notas,
            m.documento_referencia
        FROM movimiento_inventario m
        LEFT JOIN usuario u ON m.id_usuario = u.id_usuario
        WHERE m.id_producto_detalle = $1
        ORDER BY m.fecha_movimiento DESC
        LIMIT 100
        "#,
        id_producto_detalle
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        eprintln!("Error fetching inventory history: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let movimientos: Vec<MovimientoHistorial> = rows
        .into_iter()
        .map(|row| MovimientoHistorial {
            id_movimiento: row.id_movimiento,
            tipo_movimiento: row.tipo_movimiento.unwrap_or_default(),
            cantidad: row.cantidad,
            cantidad_anterior: row.cantidad_anterior,
            cantidad_nueva: row.cantidad_nueva,
            motivo: row.motivo,
            usuario: row.usuario,
            fecha_movimiento: row.fecha_movimiento.map(|dt| {
                chrono::NaiveDateTime::new(
                    chrono::NaiveDate::from_ymd_opt(dt.year(), dt.month() as u32, dt.day() as u32).unwrap(),
                    chrono::NaiveTime::from_hms_opt(dt.hour() as u32, dt.minute() as u32, dt.second() as u32).unwrap()
                )
            }),
            notas: row.notas,
            documento_referencia: row.documento_referencia,
        })
        .collect();

    Ok(Json(movimientos))
}

pub async fn update_inventario(
    State(pool): State<PgPool>,
    Path(id_producto_detalle): Path<i32>,
    Json(payload): Json<AjusteInventarioRequest>,
) -> Result<StatusCode, StatusCode> {
    // Start a transaction
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("Error starting transaction: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Get current stock and id_inventario
    let current_stock: (i32, i32) = match sqlx::query_as(
        "SELECT cantidad_disponible, id_inventario FROM inventario WHERE id_producto_detalle = $1"
    )
    .bind(id_producto_detalle)
    .fetch_one(&mut *tx)
    .await
    {
        Ok(stock) => stock,
        Err(e) => {
            eprintln!("Error fetching current stock: {:?}", e);
            return Err(StatusCode::NOT_FOUND);
        }
    };

    let cantidad_anterior = current_stock.0;
    let id_inventario = current_stock.1;
    let cantidad_nueva = payload.cantidad_disponible;
    let diferencia = cantidad_nueva - cantidad_anterior;

    // Update inventory
    let update_query = r#"
        UPDATE inventario
        SET 
            cantidad_disponible = $1,
            cantidad_minima = $2,
            cantidad_maxima = $3,
            ubicacion_fisica = $4,
            fecha_actualizacion = CURRENT_TIMESTAMP
        WHERE id_producto_detalle = $5
    "#;

    if let Err(e) = sqlx::query(update_query)
        .bind(payload.cantidad_disponible)
        .bind(payload.cantidad_minima)
        .bind(&payload.cantidad_maxima)
        .bind(&payload.ubicacion_fisica)
        .bind(id_producto_detalle)
        .execute(&mut *tx)
        .await
    {
        eprintln!("Error updating inventory: {:?}", e);
        let _ = tx.rollback().await;
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    // Create movement record for the adjustment only if quantity changed
    if diferencia != 0 {
        let insert_movement = r#"
            INSERT INTO movimiento_inventario (
                id_inventario,
                id_producto_detalle,
                tipo_movimiento,
                cantidad,
                cantidad_anterior,
                cantidad_nueva,
                motivo,
                id_usuario,
                fecha_movimiento
            ) VALUES ($1, $2, $3::tipo_movimiento_inventario, $4, $5, $6, $7, $8, CURRENT_TIMESTAMP)
        "#;

        if let Err(e) = sqlx::query(insert_movement)
            .bind(id_inventario)
            .bind(id_producto_detalle)
            .bind("ajuste")
            .bind(diferencia)
            .bind(cantidad_anterior)
            .bind(cantidad_nueva)
            .bind(&payload.motivo)
            .bind(1) // TODO: Get actual user ID from session
            .execute(&mut *tx)
            .await
        {
            eprintln!("Error creating movement record: {:?}", e);
            let _ = tx.rollback().await;
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    // Update product image if provided
    if let Some(imagen) = &payload.imagen_principal {
        let update_image_query = "UPDATE producto_detalle SET imagen_principal = $1 WHERE id_producto_detalle = $2";
        
        if let Err(e) = sqlx::query(update_image_query)
            .bind(imagen)
            .bind(id_producto_detalle)
            .execute(&mut *tx)
            .await
        {
            eprintln!("Error updating product image: {:?}", e);
            let _ = tx.rollback().await;
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    // Commit transaction
    match tx.commit().await {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => {
            eprintln!("Error committing transaction: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete_inventario(
    State(pool): State<PgPool>,
    Path(id_producto_detalle): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    // Start a transaction
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("Error starting transaction: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Check if inventory exists
    let exists: (bool,) = match sqlx::query_as(
        "SELECT EXISTS(SELECT 1 FROM inventario WHERE id_producto_detalle = $1)"
    )
    .bind(id_producto_detalle)
    .fetch_one(&mut *tx)
    .await
    {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Error checking inventory existence: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    if !exists.0 {
        return Err(StatusCode::NOT_FOUND);
    }

    // Delete associated movement records first (foreign key constraint)
    if let Err(e) = sqlx::query(
        "DELETE FROM movimiento_inventario WHERE id_producto_detalle = $1"
    )
    .bind(id_producto_detalle)
    .execute(&mut *tx)
    .await
    {
        eprintln!("Error deleting movement records: {:?}", e);
        let _ = tx.rollback().await;
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    // Delete inventory record
    if let Err(e) = sqlx::query(
        "DELETE FROM inventario WHERE id_producto_detalle = $1"
    )
    .bind(id_producto_detalle)
    .execute(&mut *tx)
    .await
    {
        eprintln!("Error deleting inventory: {:?}", e);
        let _ = tx.rollback().await;
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    // Commit transaction
    match tx.commit().await {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => {
            eprintln!("Error committing transaction: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ReporteInventarioItem {
    pub sku: String,
    pub nombre: String,
    pub categoria: String,
    pub marca: String,
    pub stock_actual: i32,
    pub precio_venta: Decimal,
    pub valor_total: Decimal,
    pub ultimo_movimiento: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReporteInventarioResponse {
    pub items: Vec<ReporteInventarioItem>,
    pub total_productos: i64,
    pub total_stock: i64,
    pub valor_total_inventario: Decimal,
}

pub async fn get_reporte_general(
    State(pool): State<PgPool>,
) -> Result<Json<ReporteInventarioResponse>, StatusCode> {
    let query = r#"
        SELECT 
            pd.sku,
            pd.nombre,
            c.nombre as categoria,
            m.nombre as marca,
            COALESCE(i.cantidad_disponible, 0) as stock_actual,
            pd.precio_venta,
            COALESCE(i.cantidad_disponible, 0) * pd.precio_venta as valor_total,
            i.fecha_actualizacion as ultimo_movimiento
        FROM producto_detalle pd
        JOIN producto p ON pd.id_producto = p.id_producto
        JOIN categoria c ON p.id_categoria = c.id_categoria
        JOIN marca m ON pd.id_marca = m.id_marca
        LEFT JOIN inventario i ON pd.id_producto_detalle = i.id_producto_detalle
        ORDER BY valor_total DESC
    "#;

    let items = match sqlx::query_as::<_, ReporteInventarioItem>(query)
        .fetch_all(&pool)
        .await
    {
        Ok(items) => items,
        Err(e) => {
            eprintln!("Error fetching report: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let total_productos = items.len() as i64;
    let total_stock: i64 = items.iter().map(|i| i.stock_actual as i64).sum();
    let valor_total_inventario: Decimal = items.iter().map(|i| i.valor_total).sum();

    Ok(Json(ReporteInventarioResponse {
        items,
        total_productos,
        total_stock,
        valor_total_inventario,
    }))
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ReporteValorizacionCategoria {
    pub categoria: String,
    pub total_productos: i64,
    pub total_stock: i64,
    pub valor_total: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReporteValorizacionResponse {
    pub por_categoria: Vec<ReporteValorizacionCategoria>,
    pub valor_total_general: Decimal,
}

pub async fn get_reporte_valorizacion(
    State(pool): State<PgPool>,
) -> Result<Json<ReporteValorizacionResponse>, StatusCode> {
    let query = r#"
        SELECT 
            c.nombre as categoria,
            COUNT(DISTINCT pd.id_producto_detalle) as total_productos,
            COALESCE(SUM(i.cantidad_disponible), 0) as total_stock,
            COALESCE(SUM(i.cantidad_disponible * pd.precio_venta), 0) as valor_total
        FROM categoria c
        LEFT JOIN producto p ON c.id_categoria = p.id_categoria
        LEFT JOIN producto_detalle pd ON p.id_producto = pd.id_producto
        LEFT JOIN inventario i ON pd.id_producto_detalle = i.id_producto_detalle
        GROUP BY c.id_categoria, c.nombre
        ORDER BY valor_total DESC
    "#;

    let por_categoria = match sqlx::query_as::<_, ReporteValorizacionCategoria>(query)
        .fetch_all(&pool)
        .await
    {
        Ok(items) => items,
        Err(e) => {
            eprintln!("Error fetching valuation report: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let valor_total_general: Decimal = por_categoria.iter().map(|c| c.valor_total).sum();

    Ok(Json(ReporteValorizacionResponse {
        por_categoria,
        valor_total_general,
    }))
}
