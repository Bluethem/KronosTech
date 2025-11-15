use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Producto {
    pub id_producto: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub slug: Option<String>,
    pub id_categoria: i32,
    pub id_subcategoria: Option<i32>,
    pub especificaciones_base: Option<serde_json::Value>,
    pub imagen_referencia: Option<String>,
    pub valoracion_promedio: Option<rust_decimal::Decimal>,
    pub total_valoraciones: i32,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub keywords: Option<String>,
    pub estado: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductoResponse {
    pub id_producto: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub slug: Option<String>,
    pub categoria: String,
    pub subcategoria: Option<String>,
    pub especificaciones_base: Option<serde_json::Value>,
    pub valoracion_promedio: Option<f64>,
    pub total_valoraciones: i32,
    pub precio_desde: Option<f64>,
    pub precio_hasta: Option<f64>,
    pub total_variantes: i64,
}
