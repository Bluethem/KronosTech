use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// ==================== MODELO PRINCIPAL ====================

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Carrito {
    pub id_carrito: i32,
    pub id_usuario: Option<i32>,
    pub id_sesion: Option<String>,
    pub estado: String, // 'activo', 'convertido', 'abandonado', 'expirado'
    pub fecha_expiracion: Option<DateTime<Utc>>,
    pub fecha_creacion: DateTime<Utc>,
    pub fecha_actualizacion: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CarritoDetalle {
    pub id_carrito_detalle: i32,
    pub id_carrito: i32,
    pub id_producto_detalle: i32,
    pub cantidad: i32,
    pub precio_unitario: rust_decimal::Decimal,
    pub fecha_agregado: DateTime<Utc>,
    pub fecha_actualizacion: DateTime<Utc>,
}

// ==================== DTOs DE RESPUESTA ====================

#[derive(Debug, Serialize)]
pub struct CarritoResponse {
    pub id_carrito: i32,
    pub id_usuario: Option<i32>,
    pub items: Vec<CarritoItemResponse>,
    pub total_items: i32,
    pub subtotal: rust_decimal::Decimal,
    pub fecha_actualizacion: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CarritoItemResponse {
    pub id_carrito_detalle: i32,
    pub id_producto_detalle: i32,
    pub id_producto: i32,
    pub nombre: String,
    pub sku: String,
    pub imagen_principal: Option<String>,
    pub precio_unitario: rust_decimal::Decimal,
    pub cantidad: i32,
    pub subtotal: rust_decimal::Decimal,
    pub stock_disponible: i32,
}

// ==================== DTOs DE REQUEST ====================

#[derive(Debug, Deserialize)]
pub struct AgregarAlCarritoRequest {
    pub id_producto_detalle: i32,
    pub cantidad: i32,
}

#[derive(Debug, Deserialize)]
pub struct ActualizarCantidadRequest {
    pub cantidad: i32,
}
