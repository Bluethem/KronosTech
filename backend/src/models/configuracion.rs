use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;
use std::collections::HashMap;

/// Modelo de configuración del sistema
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ConfiguracionSistema {
    pub id_config: i32,
    pub clave: String,
    pub valor: String,
    pub tipo: String,
    pub descripcion: Option<String>,
    pub categoria: String,
    pub fecha_actualizacion: NaiveDateTime,
    pub actualizado_por: Option<i32>,
}

/// Request para actualizar configuración
#[derive(Debug, Deserialize)]
pub struct ActualizarConfigRequest {
    pub clave: String,
    pub valor: String,
}

/// Request para actualizar múltiples configuraciones
#[derive(Debug, Deserialize)]
pub struct ActualizarConfigBatchRequest {
    pub configuraciones: Vec<ActualizarConfigRequest>,
}

/// Response de configuración como mapa clave-valor
#[derive(Debug, Serialize)]
pub struct ConfigMapResponse {
    pub success: bool,
    pub data: HashMap<String, ConfigValue>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ConfigValue {
    pub valor: String,
    pub tipo: String,
    pub categoria: String,
}

impl ConfiguracionSistema {
    /// Convierte el valor según su tipo
    pub fn get_typed_value(&self) -> serde_json::Value {
        match self.tipo.as_str() {
            "number" => {
                if let Ok(n) = self.valor.parse::<f64>() {
                    serde_json::json!(n)
                } else {
                    serde_json::json!(self.valor)
                }
            }
            "boolean" => {
                let b = self.valor.to_lowercase() == "true" || self.valor == "1";
                serde_json::json!(b)
            }
            "json" => {
                serde_json::from_str(&self.valor).unwrap_or(serde_json::json!(self.valor))
            }
            _ => serde_json::json!(self.valor)
        }
    }
}

