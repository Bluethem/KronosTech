use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Familia {
    pub id_familia: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub icono: Option<String>,
    pub slug: Option<String>,
    pub orden: i32,
    pub estado: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FamiliaResponse {
    pub id_familia: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub icono: Option<String>,
    pub slug: Option<String>,
    pub total_productos: Option<i64>,
}
