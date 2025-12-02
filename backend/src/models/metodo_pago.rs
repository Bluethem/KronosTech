use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MetodoPago {
    pub id_metodo_pago: i32,
    pub nombre: String,
    pub tipo: String,
    pub proveedor: Option<String>,
    pub descripcion: Option<String>,
    pub icono: Option<String>,
    pub comision_porcentaje: Option<Decimal>,
    pub comision_fija: Option<Decimal>,
    pub requiere_verificacion: Option<bool>,
    pub tiempo_procesamiento: Option<String>,
    pub instrucciones: Option<String>,
    pub orden: Option<i32>,
    pub activo: Option<bool>,
    pub fecha_creacion: Option<NaiveDateTime>,
}
