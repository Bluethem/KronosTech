use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Carrito {
    pub id_carrito: i32,
    pub id_usuario: Option<i32>,
    pub id_sesion: Option<String>,
    pub estado: Option<String>,
    pub fecha_expiracion: Option<NaiveDateTime>,
    pub fecha_creacion: Option<NaiveDateTime>,
    pub fecha_actualizacion: Option<NaiveDateTime>,
}
