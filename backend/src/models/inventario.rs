use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{NaiveDate, NaiveDateTime};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Inventario {
    pub id_inventario: i32,
    pub id_producto_detalle: i32,
    pub cantidad_disponible: i32,
    pub cantidad_minima: i32,
    pub cantidad_maxima: Option<i32>,
    pub ubicacion_fisica: Option<String>,
    pub lote: Option<String>,
    pub fecha_vencimiento: Option<NaiveDate>,
    pub fecha_ultima_entrada: Option<NaiveDate>,
    pub fecha_ultima_salida: Option<NaiveDate>,
    pub fecha_actualizacion: Option<NaiveDateTime>,
}
