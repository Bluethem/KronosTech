use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ImagenValoracion {
    pub id_imagen_valoracion: i32,
    pub id_valoracion: i32,
    pub url_imagen: String,
    pub orden: Option<i32>,
    pub fecha_subida: Option<NaiveDateTime>,
}
