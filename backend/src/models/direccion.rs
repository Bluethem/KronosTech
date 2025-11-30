use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// ==================== ENUMS ====================

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
#[serde(rename_all = "lowercase")]
pub enum TipoDireccion {
    #[serde(rename = "envio")]
    Envio,
    #[serde(rename = "facturacion")]
    Facturacion,
    #[serde(rename = "ambos")]
    Ambos,
}

impl std::str::FromStr for TipoDireccion {
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

impl std::fmt::Display for TipoDireccion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TipoDireccion::Envio => write!(f, "envio"),
            TipoDireccion::Facturacion => write!(f, "facturacion"),
            TipoDireccion::Ambos => write!(f, "ambos"),
        }
    }
}

// ==================== ENTITIES ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Direccion {
    pub id_direccion: i32,
    pub id_usuario: i32,
    pub tipo: TipoDireccion,
    pub nombre_completo: Option<String>,
    pub direccion_linea1: String,
    pub direccion_linea2: Option<String>,
    pub ciudad: String,
    pub departamento: String,
    pub codigo_postal: Option<String>,
    pub pais: String,
    pub telefono_contacto: Option<String>,
    pub referencia: Option<String>,
    pub es_predeterminada: bool,
    pub activo: bool,
    pub fecha_creacion: DateTime<Utc>,
}

// ==================== DTOs ====================

#[derive(Debug, Deserialize)]
pub struct CrearDireccionRequest {
    pub tipo: String, // "envio", "facturacion", "ambos"
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
    pub activo: Option<bool>,
}

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
    pub pais: String,
    pub telefono_contacto: Option<String>,
    pub referencia: Option<String>,
    pub es_predeterminada: bool,
    pub activo: bool,
    pub fecha_creacion: DateTime<Utc>,
}

impl From<Direccion> for DireccionResponse {
    fn from(d: Direccion) -> Self {
        Self {
            id_direccion: d.id_direccion,
            tipo: match d.tipo {
                TipoDireccion::Envio => "envio".to_string(),
                TipoDireccion::Facturacion => "facturacion".to_string(),
                TipoDireccion::Ambos => "ambos".to_string(),
            },
            nombre_completo: d.nombre_completo,
            direccion_linea1: d.direccion_linea1,
            direccion_linea2: d.direccion_linea2,
            ciudad: d.ciudad,
            departamento: d.departamento,
            codigo_postal: d.codigo_postal,
            pais: d.pais,
            telefono_contacto: d.telefono_contacto,
            referencia: d.referencia,
            es_predeterminada: d.es_predeterminada,
            activo: d.activo,
            fecha_creacion: d.fecha_creacion,
        }
    }
}
