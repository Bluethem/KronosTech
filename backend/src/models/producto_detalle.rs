use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProductoDetalle {
    pub id_producto_detalle: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub modelo: Option<String>,
    pub sku: String,
    pub slug: Option<String>,
    pub codigo_barras: Option<String>,
    pub id_producto: i32,
    pub id_marca: i32,
    pub precio_base: rust_decimal::Decimal,
    pub precio_venta: rust_decimal::Decimal,
    pub costo: Option<rust_decimal::Decimal>,
    pub descuento_adicional_porcentaje: Option<rust_decimal::Decimal>,
    pub descuento_adicional_activo: bool,
    pub imagen_principal: Option<String>,
    pub imagenes: Option<serde_json::Value>,
    pub peso: Option<rust_decimal::Decimal>,
    pub dimensiones: Option<String>,
    pub garantia_meses: i32,
    pub total_vendidos: i32,
    pub vistas: i32,
    pub es_destacado: bool,
    pub es_nuevo: bool,
    pub es_oferta: bool,
    pub estado: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductoDetalleResponse {
    pub id_producto_detalle: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub modelo: Option<String>,
    pub sku: String,
    pub slug: Option<String>,
    pub marca: String,
    pub marca_logo: Option<String>,
    pub precio_base: Option<f64>,
    pub precio_venta: Option<f64>,
    pub descuento_porcentaje: Option<f64>,
    pub imagen_principal: Option<String>,
    pub imagenes: Option<serde_json::Value>,
    pub peso: Option<f64>,
    pub garantia_meses: Option<i32>,
    pub total_vendidos: Option<i32>,
    pub es_destacado: Option<bool>,
    pub es_nuevo: Option<bool>,
    pub es_oferta: Option<bool>,
    pub stock_disponible: Option<i32>,
    pub stock_estado: Option<String>, // "disponible", "bajo", "agotado"
    // Datos del producto padre
    pub producto_nombre: String,
    pub producto_slug: Option<String>,
    pub categoria_nombre: String,
    pub especificaciones_base: Option<serde_json::Value>,
    pub valoracion_promedio: Option<f64>,
    pub total_valoraciones: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ProductoListItem {
    pub id_producto_detalle: i32,
    pub nombre: String,
    pub sku: String,
    pub slug: Option<String>,
    pub marca: String,
    pub precio_venta: f64,
    pub precio_base: f64,
    pub descuento_porcentaje: Option<f64>,
    pub imagen_principal: Option<String>,
    pub es_destacado: bool,
    pub es_nuevo: bool,
    pub es_oferta: bool,
    pub stock_disponible: i32,
    pub valoracion_promedio: Option<f64>,
    pub total_valoraciones: i32,
    pub categoria: String,
}
