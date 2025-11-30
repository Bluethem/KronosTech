use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
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
    #[sqlx(rename = "devuelto")]
    Devuelto,
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
    #[sqlx(rename = "rechazado")]
    Rechazado,
    #[sqlx(rename = "cancelado")]
    Cancelado,
    #[sqlx(rename = "reembolsado")]
    Reembolsado,
    #[sqlx(rename = "parcialmente_reembolsado")]
    ParcialmenteReembolsado,
}

// ==================== ENTITIES ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Venta {
    pub id_venta: i32,
    pub numero_pedido: String,
    pub id_usuario: i32,
    pub id_carrito: Option<i32>,
    // Montos
    pub subtotal: Decimal,
    pub descuento_total: Decimal,
    pub costo_envio: Decimal,
    pub total: Decimal,
    pub moneda: String,
    // Estados
    pub estado: EstadoPedido,
    pub estado_pago: EstadoPago,
    // Snapshot de dirección de envío
    pub direccion_envio: Option<String>,
    pub ciudad: Option<String>,
    pub departamento: Option<String>,
    pub codigo_postal: Option<String>,
    pub telefono_contacto: Option<String>,
    pub metodo_envio: Option<String>,
    pub numero_tracking: Option<String>,
    // Fechas
    pub fecha_pedido: DateTime<Utc>,
    pub fecha_pago: Option<DateTime<Utc>>,
    pub fecha_confirmacion: Option<DateTime<Utc>>,
    pub fecha_envio: Option<DateTime<Utc>>,
    pub fecha_entrega_estimada: Option<chrono::NaiveDate>,
    pub fecha_entrega: Option<DateTime<Utc>>,
    pub fecha_cancelacion: Option<DateTime<Utc>>,
    // Notas
    pub notas_cliente: Option<String>,
    pub notas_admin: Option<String>,
    // Tracking
    pub ip_cliente: Option<String>,
    pub user_agent: Option<String>,
    pub fecha_actualizacion: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetalleVenta {
    pub id_detalle_venta: i32,
    pub id_venta: i32,
    pub id_producto_detalle: i32,
    pub id_producto: i32,
    pub cantidad: i32,
    pub precio_unitario: Decimal,
    pub descuento_unitario: Decimal,
    pub precio_final: Decimal,
    pub subtotal: Decimal,
}

// ==================== DTOs ====================

#[derive(Debug, Deserialize)]
pub struct ProcesarCheckoutRequest {
    pub id_direccion: i32,
    pub id_metodo_pago: i32,
    pub notas_cliente: Option<String>,
    // FUTURO: Para cupones
    // pub codigo_cupon: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct VentaResponse {
    pub id_venta: i32,
    pub numero_pedido: String,
    pub id_usuario: i32,
    pub subtotal: Decimal,
    pub descuento_total: Decimal,
    pub costo_envio: Decimal,
    pub total: Decimal,
    pub moneda: String,
    pub estado: String,
    pub estado_pago: String,
    // Dirección
    pub direccion_envio: Option<String>,
    pub ciudad: Option<String>,
    pub departamento: Option<String>,
    pub codigo_postal: Option<String>,
    pub telefono_contacto: Option<String>,
    pub metodo_envio: Option<String>,
    pub numero_tracking: Option<String>,
    // Fechas
    pub fecha_pedido: DateTime<Utc>,
    pub fecha_pago: Option<DateTime<Utc>>,
    pub fecha_entrega_estimada: Option<chrono::NaiveDate>,
    pub notas_cliente: Option<String>,
    // Items del pedido
    pub items: Vec<DetalleVentaResponse>,
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

#[derive(Debug, Serialize)]
pub struct CalcularTotalResponse {
    pub subtotal: Decimal,
    pub descuento_total: Decimal,
    pub costo_envio: Decimal,
    pub total: Decimal,
    // FUTURO: Información del cupón aplicado
    // pub cupon_aplicado: Option<CuponInfo>,
}

impl From<Venta> for VentaResponse {
    fn from(v: Venta) -> Self {
        Self {
            id_venta: v.id_venta,
            numero_pedido: v.numero_pedido,
            id_usuario: v.id_usuario,
            subtotal: v.subtotal,
            descuento_total: v.descuento_total,
            costo_envio: v.costo_envio,
            total: v.total,
            moneda: v.moneda,
            estado: format!("{:?}", v.estado).to_lowercase(),
            estado_pago: format!("{:?}", v.estado_pago).to_lowercase(),
            direccion_envio: v.direccion_envio,
            ciudad: v.ciudad,
            departamento: v.departamento,
            codigo_postal: v.codigo_postal,
            telefono_contacto: v.telefono_contacto,
            metodo_envio: v.metodo_envio,
            numero_tracking: v.numero_tracking,
            fecha_pedido: v.fecha_pedido,
            fecha_pago: v.fecha_pago,
            fecha_entrega_estimada: v.fecha_entrega_estimada,
            notas_cliente: v.notas_cliente,
            items: vec![], // Se llena por el servicio
        }
    }
}
