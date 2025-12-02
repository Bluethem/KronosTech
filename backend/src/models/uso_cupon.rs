use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UsoCupon {
    pub id_uso_cupon: i32,
    pub id_cupon: i32,
    pub id_venta: i32,
    pub id_usuario: i32,
    pub descuento_aplicado: Decimal,
    pub fecha_uso: Option<NaiveDateTime>,
}
