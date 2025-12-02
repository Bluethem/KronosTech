use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{NaiveDate, NaiveDateTime};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Venta {
    pub id_venta: i32,
    pub numero_pedido: String,
    pub id_usuario: i32,
    pub id_carrito: Option<i32>,
    pub subtotal: Decimal,
    pub descuento_total: Option<Decimal>,
    pub costo_envio: Option<Decimal>,
    pub total: Decimal,
    pub moneda: Option<String>,
    pub estado: Option<String>,
    pub estado_pago: Option<String>,
    pub direccion_envio: Option<String>,
    pub ciudad: Option<String>,
    pub departamento: Option<String>,
    pub codigo_postal: Option<String>,
    pub telefono_contacto: Option<String>,
    pub metodo_envio: Option<String>,
    pub numero_tracking: Option<String>,
    pub fecha_pedido: Option<NaiveDateTime>,
    pub fecha_pago: Option<NaiveDateTime>,
    pub fecha_confirmacion: Option<NaiveDateTime>,
    pub fecha_envio: Option<NaiveDateTime>,
    pub fecha_entrega_estimada: Option<NaiveDate>,
    pub fecha_entrega: Option<NaiveDateTime>,
    pub fecha_cancelacion: Option<NaiveDateTime>,
    pub notas_cliente: Option<String>,
    pub notas_admin: Option<String>,
    pub ip_cliente: Option<String>,
    pub user_agent: Option<String>,
    pub fecha_actualizacion: Option<NaiveDateTime>,
}

// ==================== DTOs para handlers (H3nr7) ====================

#[derive(Debug, Deserialize)]
pub struct ProcesarCheckoutRequest {
    pub id_direccion: i32,
    pub id_metodo_pago: i32,
    pub notas_cliente: Option<String>,
    // FUTURO: Para cupones
    // pub codigo_cupon: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CalcularTotalResponse {
    pub subtotal: Decimal,
    pub descuento_total: Decimal,
    pub costo_envio: Decimal,
    pub total: Decimal,
    pub items_count: i32,
}

#[derive(Debug, Serialize)]
pub struct VentaResponse {
    pub id_venta: i32,
    pub numero_pedido: String,
    pub subtotal: Decimal,
    pub descuento_total: Decimal,
    pub costo_envio: Decimal,
    pub total: Decimal,
    pub estado: String,
    pub estado_pago: String,
    pub fecha_pedido: NaiveDateTime,
    pub direccion_envio: Option<String>,
    pub ciudad: Option<String>,
    pub departamento: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DetalleVentaResponse {
    pub id_producto_detalle: i32,
    pub nombre: String,
    pub sku: String,
    pub cantidad: i32,
    pub precio_unitario: Decimal,
    pub subtotal: Decimal,
}
