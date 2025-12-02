use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EspecificacionProducto {
    pub id_especificacion: i32,
    pub id_producto_detalle: i32,
    pub nombre_atributo: String,
    pub valor_atributo: String,
    pub unidad_medida: Option<String>,
    pub orden: Option<i32>,
}
