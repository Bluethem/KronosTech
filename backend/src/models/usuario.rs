use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Usuario {
    pub id_usuario: i32,
    pub nombre: String,
    pub apellido: String,
    pub email: String,
    pub contrasena: String,
    pub telefono: Option<String>,
    pub dni: Option<String>,
    pub rol: Option<String>,
    pub email_verificado: Option<bool>,
    pub token_verificacion: Option<String>,
    pub activo: Option<bool>,
    pub fecha_registro: Option<NaiveDateTime>,
    pub ultima_conexion: Option<NaiveDateTime>,
    pub fecha_actualizacion: Option<NaiveDateTime>,
}
