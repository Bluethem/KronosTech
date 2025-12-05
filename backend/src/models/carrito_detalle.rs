use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::PrimitiveDateTime;
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CarritoDetalle {
    pub id_carrito_detalle: i32,
    pub id_carrito: i32,
    pub id_producto_detalle: i32,
    pub cantidad: i32,
    pub precio_unitario: Decimal,
    pub fecha_agregado: PrimitiveDateTime,
    pub fecha_actualizacion: PrimitiveDateTime,
}
