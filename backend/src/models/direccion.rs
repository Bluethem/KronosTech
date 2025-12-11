use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;
use std::str::FromStr;

// ==================== ENUMS ====================

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "tipo_direccion", rename_all = "lowercase")]
pub enum TipoDireccion {
    #[sqlx(rename = "envio")]
    Envio,
    #[sqlx(rename = "facturacion")]
    Facturacion,
    #[sqlx(rename = "ambos")]
    Ambos,
}

impl FromStr for TipoDireccion {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "envio" => Ok(TipoDireccion::Envio),
            "facturacion" => Ok(TipoDireccion::Facturacion),
            "ambos" => Ok(TipoDireccion::Ambos),
            _ => Err(format!("Tipo de dirección inválido: {}", s)),
        }
    }
}

// ==================== MODELO PRINCIPAL ====================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Direccion {
    pub id_direccion: i32,
    pub id_usuario: i32,
    pub tipo: Option<String>,
    pub nombre_completo: Option<String>,
    pub direccion_linea1: String,
    pub direccion_linea2: Option<String>,
    pub ciudad: String,
    pub departamento: String,
    pub codigo_postal: Option<String>,
    pub pais: Option<String>,
    pub telefono_contacto: Option<String>,
    pub referencia: Option<String>,
    pub es_predeterminada: Option<bool>,
    pub activo: Option<bool>,
    pub fecha_creacion: Option<NaiveDateTime>,
}

// ==================== DTOs DE RESPUESTA ====================

#[derive(Debug, Serialize)]
pub struct DireccionResponse {
    pub id_direccion: i32,
    pub tipo: String,
    pub nombre_completo: Option<String>,
    pub direccion_linea1: String,
    pub direccion_linea2: Option<String>,
    pub ciudad: String,
    pub departamento: String,
    pub codigo_postal: Option<String>,
    pub pais: Option<String>,
    pub telefono_contacto: Option<String>,
    pub referencia: Option<String>,
    pub es_predeterminada: bool,
}

impl From<Direccion> for DireccionResponse {
    fn from(d: Direccion) -> Self {
        DireccionResponse {
            id_direccion: d.id_direccion,
            tipo: d.tipo.unwrap_or_else(|| "envio".to_string()),
            nombre_completo: d.nombre_completo,
            direccion_linea1: d.direccion_linea1,
            direccion_linea2: d.direccion_linea2,
            ciudad: d.ciudad,
            departamento: d.departamento,
            codigo_postal: d.codigo_postal,
            pais: d.pais,
            telefono_contacto: d.telefono_contacto,
            referencia: d.referencia,
            es_predeterminada: d.es_predeterminada.unwrap_or(false),
        }
    }
}

// ==================== DTOs DE REQUEST ====================

#[derive(Debug, Deserialize)]
pub struct CrearDireccionRequest {
    pub tipo: Option<String>, // "envio", "facturacion", "ambos"
    pub nombre_completo: Option<String>,
    pub direccion_linea1: String,
    pub direccion_linea2: Option<String>,
    pub ciudad: String,
    pub departamento: String,
    pub codigo_postal: Option<String>,
    pub pais: Option<String>, // Default: "Perú"
    pub telefono_contacto: Option<String>,
    pub referencia: Option<String>,
    pub es_predeterminada: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ActualizarDireccionRequest {
    pub tipo: Option<String>,
    pub nombre_completo: Option<String>,
    pub direccion_linea1: Option<String>,
    pub direccion_linea2: Option<String>,
    pub ciudad: Option<String>,
    pub departamento: Option<String>,
    pub codigo_postal: Option<String>,
    pub pais: Option<String>,
    pub telefono_contacto: Option<String>,
    pub referencia: Option<String>,
    pub es_predeterminada: Option<bool>,
}
