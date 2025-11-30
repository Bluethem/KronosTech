use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AsignacionCupon {
    pub id_asignacion: i32,
    pub id_cupon: i32,
    pub id_usuario: i32,
    pub usado: Option<bool>,
    pub fecha_asignacion: Option<NaiveDateTime>,
    pub fecha_uso: Option<NaiveDateTime>,
}
