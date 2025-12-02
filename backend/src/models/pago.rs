use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Pago {
    pub id_pago: i32,
    pub id_venta: i32,
    pub id_metodo_pago: i32,
    pub id_metodo_pago_cliente: Option<i32>,
    pub numero_transaccion: Option<String>,
    pub estado: Option<String>,
    pub monto: Decimal,
    pub moneda: Option<String>,
    pub comision: Option<Decimal>,
    pub monto_neto: Decimal,
    pub proveedor_pago: Option<String>,
    pub id_transaccion_proveedor: Option<String>,
    pub token_pago: Option<String>,
    pub respuesta_proveedor: Option<serde_json::Value>,
    pub ultimos_4_digitos: Option<String>,
    pub marca_tarjeta: Option<String>,
    pub ip_cliente: Option<String>,
    pub user_agent: Option<String>,
    pub fecha_pago: Option<NaiveDateTime>,
    pub fecha_creacion: Option<NaiveDateTime>,
    pub fecha_actualizacion: Option<NaiveDateTime>,
    pub nota_error: Option<String>,
    pub intentos_fallidos: Option<i32>,
}
