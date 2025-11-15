use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Subcategoria {
    pub id_subcategoria: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub slug: Option<String>,
    pub id_categoria: i32,
    pub orden: i32,
    #[sqlx(try_from = "String")]
    pub estado: String,
    #[serde(skip)]
    pub fecha_creacion: Option<sqlx::types::time::PrimitiveDateTime>,
    #[serde(skip)]
    pub fecha_actualizacion: Option<sqlx::types::time::PrimitiveDateTime>,
}
