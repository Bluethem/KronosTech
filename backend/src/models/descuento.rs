use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Descuento {
    pub id_descuento: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub tipo_descuento: String,
    pub valor: Decimal,
    pub aplica_a: String,
    pub id_referencia: Option<i32>,
    pub compra_minima: Option<Decimal>,
    pub cantidad_minima: Option<i32>,
    pub usos_maximos: Option<i32>,
    pub usos_actuales: Option<i32>,
    pub fecha_inicio: NaiveDateTime,
    pub fecha_fin: NaiveDateTime,
    pub activo: Option<bool>,
    pub fecha_creacion: Option<NaiveDateTime>,
    pub fecha_actualizacion: Option<NaiveDateTime>,
}
