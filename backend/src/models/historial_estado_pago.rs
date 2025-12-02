use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct HistorialEstadoPago {
    pub id_historial: i32,
    pub id_pago: i32,
    pub estado_anterior: Option<String>,
    pub estado_nuevo: String,
    pub razon: Option<String>,
    pub id_usuario: Option<i32>,
    pub metadatos: Option<serde_json::Value>,
    pub fecha_cambio: Option<NaiveDateTime>,
}
