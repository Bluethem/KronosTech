use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Reembolso {
    pub id_reembolso: i32,
    pub id_pago: i32,
    pub id_venta: i32,
    pub tipo_reembolso: String,
    pub monto_reembolsado: Decimal,
    pub motivo: String,
    pub estado: Option<String>,
    pub id_reembolso_proveedor: Option<String>,
    pub respuesta_proveedor: Option<serde_json::Value>,
    pub id_usuario_solicitante: Option<i32>,
    pub id_usuario_aprobador: Option<i32>,
    pub fecha_solicitado: Option<NaiveDateTime>,
    pub fecha_aprobado: Option<NaiveDateTime>,
    pub fecha_completado: Option<NaiveDateTime>,
    pub notas_admin: Option<String>,
}
