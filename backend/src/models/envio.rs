use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{NaiveDate, NaiveDateTime};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Envio {
    pub id_envio: i32,
    pub id_venta: i32,
    pub empresa_envio: Option<String>,
    pub metodo_envio: Option<String>,
    pub numero_tracking: Option<String>,
    pub costo: Option<Decimal>,
    pub estado: Option<String>,
    pub direccion_completa: Option<String>,
    pub ciudad: Option<String>,
    pub departamento: Option<String>,
    pub codigo_postal: Option<String>,
    pub telefono_contacto: Option<String>,
    pub fecha_estimada: Option<NaiveDate>,
    pub fecha_envio: Option<NaiveDateTime>,
    pub fecha_entrega: Option<NaiveDateTime>,
    pub fecha_creacion: Option<NaiveDateTime>,
    pub fecha_actualizacion: Option<NaiveDateTime>,
    pub historial_tracking: Option<serde_json::Value>,
    pub notas: Option<String>,
}
