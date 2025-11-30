use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

// ==================== ENUMS ====================

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "tipo_metodo_pago", rename_all = "snake_case")]
pub enum TipoMetodoPago {
    #[sqlx(rename = "tarjeta_credito")]
    TarjetaCredito,
    #[sqlx(rename = "tarjeta_debito")]
    TarjetaDebito,
    #[sqlx(rename = "billetera_digital")]
    BilleteraDigital,
    #[sqlx(rename = "transferencia")]
    Transferencia,
    #[sqlx(rename = "efectivo")]
    Efectivo,
    #[sqlx(rename = "contrareembolso")]
    Contrareembolso,
}

// ==================== ENTITIES ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetodoPago {
    pub id_metodo_pago: i32,
    pub nombre: String,
    pub tipo: TipoMetodoPago,
    pub proveedor: Option<String>,
    pub descripcion: Option<String>,
    pub icono: Option<String>,
    pub comision_porcentaje: Decimal,
    pub comision_fija: Decimal,
    pub requiere_verificacion: bool,
    pub tiempo_procesamiento: Option<String>,
    pub instrucciones: Option<String>,
    pub orden: i32,
    pub activo: bool,
    pub fecha_creacion: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pago {
    pub id_pago: i32,
    pub id_venta: i32,
    pub id_metodo_pago: i32,
    pub numero_transaccion: Option<String>,
    pub estado: String, // De la tabla usa estado_pago enum
    pub monto: Decimal,
    pub moneda: String,
    pub comision: Decimal,
    pub monto_neto: Decimal,
    // Datos del procesador (simulado por ahora)
    pub proveedor_pago: Option<String>,
    pub id_transaccion_proveedor: Option<String>,
    pub ip_cliente: Option<String>,
    pub user_agent: Option<String>,
    pub fecha_pago: Option<DateTime<Utc>>,
    pub fecha_creacion: DateTime<Utc>,
    pub nota_error: Option<String>,
    pub intentos_fallidos: i32,
}

// ==================== DTOs ====================

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
    pub tiempo_procesamiento: Option<String>,
    pub instrucciones: Option<String>,
}

impl From<MetodoPago> for MetodoPagoResponse {
    fn from(mp: MetodoPago) -> Self {
        Self {
            id_metodo_pago: mp.id_metodo_pago,
            nombre: mp.nombre,
            tipo: match mp.tipo {
                TipoMetodoPago::TarjetaCredito => "tarjeta_credito".to_string(),
                TipoMetodoPago::TarjetaDebito => "tarjeta_debito".to_string(),
                TipoMetodoPago::BilleteraDigital => "billetera_digital".to_string(),
                TipoMetodoPago::Transferencia => "transferencia".to_string(),
                TipoMetodoPago::Efectivo => "efectivo".to_string(),
                TipoMetodoPago::Contrareembolso => "contrareembolso".to_string(),
            },
            proveedor: mp.proveedor,
            descripcion: mp.descripcion,
            icono: mp.icono,
            comision_porcentaje: mp.comision_porcentaje,
            comision_fija: mp.comision_fija,
            tiempo_procesamiento: mp.tiempo_procesamiento,
            instrucciones: mp.instrucciones,
        }
    }
}
