use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Categoria {
    pub id_categoria: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub icono: Option<String>,
    pub slug: Option<String>,
    pub id_familia: i32,
    pub orden: i32,
    pub estado: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoriaResponse {
    pub id_categoria: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub icono: Option<String>,
    pub slug: Option<String>,
    pub id_familia: i32,
    pub familia_nombre: Option<String>,
    pub total_productos: Option<i64>,
}
