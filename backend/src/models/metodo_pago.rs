use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;
use rust_decimal::Decimal;

// ==================== MODELO PRINCIPAL ====================

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

// ==================== DTOs DE RESPUESTA ====================

#[derive(Debug, Serialize)]
pub struct MetodoPagoResponse {
    pub id_metodo_pago: i32,
    pub nombre: String,
    pub tipo: String,
    pub proveedor: Option<String>,
    pub descripcion: Option<String>,
    pub icono: Option<String>,
    pub comision_porcentaje: Decimal,
    pub comision_fija: Decimal,
    pub requiere_verificacion: bool,
    pub tiempo_procesamiento: Option<String>,
    pub instrucciones: Option<String>,
}

impl From<MetodoPago> for MetodoPagoResponse {
    fn from(mp: MetodoPago) -> Self {
        MetodoPagoResponse {
            id_metodo_pago: mp.id_metodo_pago,
            nombre: mp.nombre,
            tipo: mp.tipo,
            proveedor: mp.proveedor,
            descripcion: mp.descripcion,
            icono: mp.icono,
            comision_porcentaje: mp.comision_porcentaje.unwrap_or(Decimal::ZERO),
            comision_fija: mp.comision_fija.unwrap_or(Decimal::ZERO),
            requiere_verificacion: mp.requiere_verificacion.unwrap_or(false),
            tiempo_procesamiento: mp.tiempo_procesamiento,
            instrucciones: mp.instrucciones,
        }
    }
}
