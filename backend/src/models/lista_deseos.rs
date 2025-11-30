use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ListaDeseos {
    pub id_lista_deseos: i32,
    pub id_usuario: i32,
    pub id_producto_detalle: i32,
    pub fecha_agregado: Option<NaiveDateTime>,
}
