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
