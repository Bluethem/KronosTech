use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Cupon {
    pub id_cupon: i32,
    pub codigo: String,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub tipo_cupon: String,
    pub valor: Decimal,
    pub aplica_cupon: String,
    pub id_referencia: Option<i32>,
    pub compra_minima: Option<Decimal>,
    pub usos_maximos_totales: Option<i32>,
    pub usos_maximos_por_usuario: Option<i32>,
    pub usos_actuales: Option<i32>,
    pub solo_nuevos_usuarios: Option<bool>,
    pub solo_primera_compra: Option<bool>,
    pub fecha_inicio: NaiveDateTime,
    pub fecha_fin: NaiveDateTime,
    pub activo: Option<bool>,
    pub fecha_creacion: Option<NaiveDateTime>,
    pub fecha_actualizacion: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CuponListItem {
    pub id_cupon: i32,
    pub codigo: String,
    pub nombre: String,
    pub tipo_cupon: String,
    pub valor: Decimal,
    pub fecha_inicio: NaiveDateTime,
    pub fecha_fin: NaiveDateTime,
    pub usos_actuales: Option<i32>,
    pub usos_maximos_totales: Option<i32>,
    pub activo: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CuponStats {
    pub total_activos: i64,
    pub usos_hoy: i64,
    pub descuento_mes: Option<Decimal>,
}
