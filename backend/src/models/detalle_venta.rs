use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DetalleVenta {
    pub id_detalle_venta: i32,
    pub id_venta: i32,
    pub id_producto_detalle: i32,
    pub id_producto: i32,
    pub cantidad: i32,
    pub precio_unitario: Decimal,
    pub descuento_unitario: Option<Decimal>,
    pub precio_final: Decimal,
    pub subtotal: Decimal,
}
