use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

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

// ==================== DTOs para handlers (H3nr7) ====================

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
