use axum::{
    extract::{Query, State, Path},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use rust_decimal::Decimal;
use chrono::NaiveDateTime;

use crate::models::descuento::Descuento;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct DescuentoListItem {
    pub id_descuento: i32,
    pub nombre: String,
    pub tipo_descuento: String,
    pub valor: Decimal,
    pub aplica_a: String,
    pub referencia_nombre: Option<String>,
    pub fecha_inicio: NaiveDateTime,
    pub fecha_fin: NaiveDateTime,
    pub usos_maximos: Option<i32>,
    pub usos_actuales: Option<i32>,
    pub activo: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct DescuentoStats {
    pub total_activos: i64,
    pub vigentes_hoy: i64,
    pub usos_mes_actual: i64,
    pub ahorro_generado_mes: Decimal,
}

#[derive(Debug, Deserialize)]
pub struct DescuentoQuery {
    pub search: Option<String>,
    pub tipo: Option<String>,
    pub estado: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateDescuentoRequest {
    pub nombre: String,
    pub descripcion: Option<String>,
    pub tipo_descuento: String,
    pub valor: Decimal,
    pub aplica_a: String,
    pub id_referencia: Option<i32>,
    pub compra_minima: Option<Decimal>,
    pub cantidad_minima: Option<i32>,
    pub usos_maximos: Option<i32>,
    pub fecha_inicio: String,
    pub fecha_fin: String,
    pub activo: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ProductoDropdown {
    pub id_producto_detalle: i32,
    pub nombre: String,
}

pub async fn get_descuentos(
    State(pool): State<PgPool>,
    Query(query): Query<DescuentoQuery>,
) -> Result<Json<Vec<DescuentoListItem>>, StatusCode> {
    let mut sql = String::from(
        r#"
        SELECT 
            d.id_descuento,
            d.nombre,
            d.tipo_descuento::TEXT as tipo_descuento,
            d.valor,
            d.aplica_a::TEXT as aplica_a,
            CASE 
                WHEN d.aplica_a = 'categoria' THEN c.nombre
                WHEN d.aplica_a = 'marca' THEN m.nombre
                WHEN d.aplica_a = 'producto' THEN pd.nombre
                ELSE 'Global'
            END as referencia_nombre,
            d.fecha_inicio,
            d.fecha_fin,
            d.usos_maximos,
            d.usos_actuales,
            d.activo
        FROM descuento d
        LEFT JOIN categoria c ON d.aplica_a = 'categoria' AND d.id_referencia = c.id_categoria
        LEFT JOIN marca m ON d.aplica_a = 'marca' AND d.id_referencia = m.id_marca
        LEFT JOIN producto_detalle pd ON d.aplica_a = 'producto' AND d.id_referencia = pd.id_producto_detalle
        WHERE 1=1
        "#,
    );

    if let Some(search) = &query.search {
        if !search.is_empty() {
            sql.push_str(&format!(" AND d.nombre ILIKE '%{}%'", search));
        }
    }

    if let Some(tipo) = &query.tipo {
        if !tipo.is_empty() && tipo != "todos" {
            sql.push_str(&format!(" AND d.tipo_descuento::TEXT = '{}'", tipo));
        }
    }

    if let Some(estado) = &query.estado {
        match estado.as_str() {
            "activo" => sql.push_str(" AND d.activo = true"),
            "inactivo" => sql.push_str(" AND d.activo = false"),
            "vigente" => sql.push_str(" AND d.activo = true AND CURRENT_DATE BETWEEN d.fecha_inicio AND d.fecha_fin"),
            "vencido" => sql.push_str(" AND CURRENT_DATE > d.fecha_fin"),
            "proximo" => sql.push_str(" AND CURRENT_DATE < d.fecha_inicio"),
            _ => {}
        }
    }

    sql.push_str(" ORDER BY d.fecha_creacion DESC");

    match sqlx::query_as::<_, DescuentoListItem>(&sql)
        .fetch_all(&pool)
        .await
    {
        Ok(descuentos) => Ok(Json(descuentos)),
        Err(e) => {
            eprintln!("Error fetching descuentos: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_descuento_stats(
    State(pool): State<PgPool>,
) -> Result<Json<DescuentoStats>, StatusCode> {
    let stats_query = r#"
        SELECT 
            COUNT(*) FILTER (WHERE activo = true) as total_activos,
            COUNT(*) FILTER (WHERE activo = true AND CURRENT_DATE BETWEEN fecha_inicio AND fecha_fin) as vigentes_hoy,
            COALESCE(SUM(usos_actuales) FILTER (WHERE DATE_TRUNC('month', fecha_creacion) = DATE_TRUNC('month', CURRENT_DATE)), 0) as usos_mes_actual,
            0::DECIMAL as ahorro_generado_mes
        FROM descuento
    "#;

    match sqlx::query_as::<_, DescuentoStats>(stats_query)
        .fetch_one(&pool)
        .await
    {
        Ok(stats) => Ok(Json(stats)),
        Err(e) => {
            eprintln!("Error fetching descuento stats: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_descuento_detalle(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Descuento>, StatusCode> {
    match sqlx::query_as::<_, Descuento>(
        r#"
        SELECT 
            id_descuento,
            nombre,
            descripcion,
            tipo_descuento::TEXT as tipo_descuento,
            valor,
            aplica_a::TEXT as aplica_a,
            id_referencia,
            compra_minima,
            cantidad_minima,
            usos_maximos,
            usos_actuales,
            fecha_inicio,
            fecha_fin,
            activo,
            fecha_creacion,
            fecha_actualizacion
        FROM descuento 
        WHERE id_descuento = $1
        "#
    )
    .bind(id)
    .fetch_one(&pool)
    .await
    {
        Ok(descuento) => Ok(Json(descuento)),
        Err(e) => {
            eprintln!("Error fetching descuento detalle: {:?}", e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

pub async fn create_descuento(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateDescuentoRequest>,
) -> Result<Json<Descuento>, StatusCode> {
    // Validate required fields
    if payload.nombre.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    if payload.valor <= Decimal::ZERO {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Validate tipo_descuento
    if !["porcentaje", "monto_fijo", "envio_gratis"].contains(&payload.tipo_descuento.as_str()) {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Validate aplica_a
    if !["global", "producto", "categoria", "marca", "familia"].contains(&payload.aplica_a.as_str()) {
        return Err(StatusCode::BAD_REQUEST);
    }

    // If aplica_a is producto, id_referencia is required
    if payload.aplica_a == "producto" && payload.id_referencia.is_none() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Parse dates from string
    let fecha_inicio = NaiveDateTime::parse_from_str(&payload.fecha_inicio, "%Y-%m-%d %H:%M:%S")
        .map_err(|e| {
            eprintln!("Error parsing fecha_inicio: {:?}", e);
            StatusCode::BAD_REQUEST
        })?;
    
    let fecha_fin = NaiveDateTime::parse_from_str(&payload.fecha_fin, "%Y-%m-%d %H:%M:%S")
        .map_err(|e| {
            eprintln!("Error parsing fecha_fin: {:?}", e);
            StatusCode::BAD_REQUEST
        })?;

    // Validate dates
    if fecha_fin <= fecha_inicio {
        return Err(StatusCode::BAD_REQUEST);
    }

    match sqlx::query_as::<_, Descuento>(
        r#"
        INSERT INTO descuento (
            nombre, descripcion, tipo_descuento, valor, aplica_a, id_referencia,
            compra_minima, cantidad_minima, usos_maximos, usos_actuales,
            fecha_inicio, fecha_fin, activo
        )
        VALUES ($1, $2, CAST($3 AS tipo_descuento), $4, CAST($5 AS aplica_descuento), $6, $7, $8, $9, 0, $10, $11, $12)
        RETURNING 
            id_descuento,
            nombre,
            descripcion,
            tipo_descuento::TEXT as tipo_descuento,
            valor,
            aplica_a::TEXT as aplica_a,
            id_referencia,
            compra_minima,
            cantidad_minima,
            usos_maximos,
            usos_actuales,
            fecha_inicio,
            fecha_fin,
            activo,
            fecha_creacion,
            fecha_actualizacion
        "#
    )
    .bind(&payload.nombre)
    .bind(&payload.descripcion)
    .bind(&payload.tipo_descuento)
    .bind(&payload.valor)
    .bind(&payload.aplica_a)
    .bind(&payload.id_referencia)
    .bind(&payload.compra_minima)
    .bind(&payload.cantidad_minima)
    .bind(&payload.usos_maximos)
    .bind(&fecha_inicio)
    .bind(&fecha_fin)
    .bind(payload.activo.unwrap_or(true))
    .fetch_one(&pool)
    .await
    {
        Ok(descuento) => Ok(Json(descuento)),
        Err(e) => {
            eprintln!("Error creating descuento: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_productos_dropdown(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<ProductoDropdown>>, StatusCode> {
    match sqlx::query_as::<_, ProductoDropdown>(
        r#"
        SELECT id_producto_detalle, nombre
        FROM producto_detalle
        ORDER BY nombre ASC
        "#
    )
    .fetch_all(&pool)
    .await
    {
        Ok(productos) => Ok(Json(productos)),
        Err(e) => {
            eprintln!("Error fetching productos dropdown: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update_descuento(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<CreateDescuentoRequest>,
) -> Result<Json<Descuento>, StatusCode> {
    // Validate required fields (same as create)
    if payload.nombre.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    if payload.valor <= Decimal::ZERO {
        return Err(StatusCode::BAD_REQUEST);
    }

    if !["porcentaje", "monto_fijo", "envio_gratis"].contains(&payload.tipo_descuento.as_str()) {
        return Err(StatusCode::BAD_REQUEST);
    }

    if !["global", "producto", "categoria", "marca", "familia"].contains(&payload.aplica_a.as_str()) {
        return Err(StatusCode::BAD_REQUEST);
    }

    if payload.aplica_a == "producto" && payload.id_referencia.is_none() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Parse dates
    let fecha_inicio = NaiveDateTime::parse_from_str(&payload.fecha_inicio, "%Y-%m-%d %H:%M:%S")
        .map_err(|e| {
            eprintln!("Error parsing fecha_inicio: {:?}", e);
            StatusCode::BAD_REQUEST
        })?;
    
    let fecha_fin = NaiveDateTime::parse_from_str(&payload.fecha_fin, "%Y-%m-%d %H:%M:%S")
        .map_err(|e| {
            eprintln!("Error parsing fecha_fin: {:?}", e);
            StatusCode::BAD_REQUEST
        })?;

    if fecha_fin <= fecha_inicio {
        return Err(StatusCode::BAD_REQUEST);
    }

    match sqlx::query_as::<_, Descuento>(
        r#"
        UPDATE descuento SET
            nombre = $1,
            descripcion = $2,
            tipo_descuento = CAST($3 AS tipo_descuento),
            valor = $4,
            aplica_a = CAST($5 AS aplica_descuento),
            id_referencia = $6,
            compra_minima = $7,
            cantidad_minima = $8,
            usos_maximos = $9,
            fecha_inicio = $10,
            fecha_fin = $11,
            activo = $12,
            fecha_actualizacion = CURRENT_TIMESTAMP
        WHERE id_descuento = $13
        RETURNING 
            id_descuento,
            nombre,
            descripcion,
            tipo_descuento::TEXT as tipo_descuento,
            valor,
            aplica_a::TEXT as aplica_a,
            id_referencia,
            compra_minima,
            cantidad_minima,
            usos_maximos,
            usos_actuales,
            fecha_inicio,
            fecha_fin,
            activo,
            fecha_creacion,
            fecha_actualizacion
        "#
    )
    .bind(&payload.nombre)
    .bind(&payload.descripcion)
    .bind(&payload.tipo_descuento)
    .bind(&payload.valor)
    .bind(&payload.aplica_a)
    .bind(&payload.id_referencia)
    .bind(&payload.compra_minima)
    .bind(&payload.cantidad_minima)
    .bind(&payload.usos_maximos)
    .bind(&fecha_inicio)
    .bind(&fecha_fin)
    .bind(payload.activo.unwrap_or(true))
    .bind(id)
    .fetch_one(&pool)
    .await
    {
        Ok(descuento) => Ok(Json(descuento)),
        Err(e) => {
            eprintln!("Error updating descuento: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete_descuento(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    match sqlx::query("DELETE FROM descuento WHERE id_descuento = $1")
        .bind(id)
        .execute(&pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                Ok(StatusCode::NO_CONTENT)
            } else {
                Err(StatusCode::NOT_FOUND)
            }
        }
        Err(e) => {
            eprintln!("Error deleting descuento: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
