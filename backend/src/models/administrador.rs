use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Administrador {
    pub id_administrador: i32,
    pub id_usuario: i32,
    pub es_super_admin: Option<bool>,
    pub permisos: Option<serde_json::Value>,
    pub fecha_creacion: Option<NaiveDateTime>,
}
