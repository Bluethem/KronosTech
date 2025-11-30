use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ImagenProducto {
    pub id_imagen: i32,
    pub id_producto_detalle: i32,
    pub url_imagen: String,
    pub es_principal: Option<bool>,
    pub orden: Option<i32>,
    pub alt_text: Option<String>,
    pub titulo: Option<String>,
    pub fecha_subida: Option<NaiveDateTime>,
}
