use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Notificacion {
    pub id_notificacion: i32,
    pub id_usuario: i32,
    pub tipo: String,
    pub titulo: String,
    pub mensaje: String,
    pub url: Option<String>,
    pub leida: Option<bool>,
    pub fecha_creacion: Option<NaiveDateTime>,
    pub fecha_leida: Option<NaiveDateTime>,
}
