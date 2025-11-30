use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MetodoPagoCliente {
    pub id_metodo_pago_cliente: i32,
    pub id_usuario: i32,
    pub id_metodo_pago: i32,
    pub tipo: String,
    pub token_pago: Option<String>,
    pub ultimos_4_digitos: Option<String>,
    pub marca: Option<String>,
    pub fecha_expiracion: Option<String>,
    pub nombre_titular: Option<String>,
    pub es_predeterminado: Option<bool>,
    pub activo: Option<bool>,
    pub fecha_creacion: Option<NaiveDateTime>,
    pub fecha_actualizacion: Option<NaiveDateTime>,
}
