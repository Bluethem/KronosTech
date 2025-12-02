use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MovimientoInventario {
    pub id_movimiento: i32,
    pub id_inventario: i32,
    pub id_producto_detalle: i32,
    pub tipo_movimiento: String,
    pub cantidad: i32,
    pub cantidad_anterior: i32,
    pub cantidad_nueva: i32,
    pub motivo: Option<String>,
    pub id_usuario: Option<i32>,
    pub id_venta: Option<i32>,
    pub id_proveedor: Option<i32>,
    pub costo_unitario: Option<Decimal>,
    pub documento_referencia: Option<String>,
    pub fecha_movimiento: Option<NaiveDateTime>,
    pub notas: Option<String>,
}
