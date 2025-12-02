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
        "SELECT * FROM descuento WHERE id_descuento = $1"
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
