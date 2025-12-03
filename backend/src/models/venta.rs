use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{NaiveDate, NaiveDateTime};
use rust_decimal::Decimal;

// ==================== ENUMS ====================

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "estado_pedido", rename_all = "lowercase")]
pub enum EstadoPedido {
    #[sqlx(rename = "pendiente")]
    Pendiente,
    #[sqlx(rename = "confirmado")]
    Confirmado,
    #[sqlx(rename = "procesando")]
    Procesando,
    #[sqlx(rename = "enviado")]
    Enviado,
    #[sqlx(rename = "entregado")]
    Entregado,
    #[sqlx(rename = "cancelado")]
    Cancelado,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "estado_pago", rename_all = "lowercase")]
pub enum EstadoPago {
    #[sqlx(rename = "pendiente")]
    Pendiente,
    #[sqlx(rename = "procesando")]
    Procesando,
    #[sqlx(rename = "completado")]
    Completado,
    #[sqlx(rename = "fallido")]
    Fallido,
    #[sqlx(rename = "reembolsado")]
    Reembolsado,
    #[sqlx(rename = "parcialmente_reembolsado")]
    ParcialmenteReembolsado,
}

// ==================== MODELO PRINCIPAL ====================

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
    pub items: Vec<DetalleVentaResponse>,
}

impl From<Venta> for VentaResponse {
    fn from(v: Venta) -> Self {
        VentaResponse {
            id_venta: v.id_venta,
            numero_pedido: v.numero_pedido,
            subtotal: v.subtotal,
            descuento_total: v.descuento_total.unwrap_or(Decimal::ZERO),
            costo_envio: v.costo_envio.unwrap_or(Decimal::ZERO),
            total: v.total,
            estado: v.estado.unwrap_or_else(|| "pendiente".to_string()),
            estado_pago: v.estado_pago.unwrap_or_else(|| "pendiente".to_string()),
            fecha_pedido: v.fecha_pedido.unwrap_or_else(|| chrono::Utc::now().naive_utc()),
            direccion_envio: v.direccion_envio,
            ciudad: v.ciudad,
            departamento: v.departamento,
            items: Vec::new(), // Se llenará después
        }
    }
}

#[derive(Debug, Serialize)]
pub struct DetalleVentaResponse {
    pub id_detalle_venta: i32,
    pub id_producto_detalle: i32,
    pub nombre_producto: String,
    pub sku: String,
    pub imagen: Option<String>,
    pub cantidad: i32,
    pub precio_unitario: Decimal,
    pub descuento_unitario: Decimal,
    pub precio_final: Decimal,
    pub subtotal: Decimal,
}
