use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Marca {
    pub id_marca: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub logo: Option<String>,
    pub slug: Option<String>,
    pub pais_origen: Option<String>,
    pub sitio_web: Option<String>,
    pub estado: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarcaResponse {
    pub id_marca: i32,
    pub nombre: String,
    pub logo: Option<String>,
    pub slug: Option<String>,
    pub total_productos: Option<i64>,
}
