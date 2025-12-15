use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Valoracion {
    pub id_valoracion: i32,
    pub id_producto: i32,
    pub id_usuario: i32,
    pub id_producto_detalle: Option<i32>,
    pub calificacion: i32,
    pub titulo: Option<String>,
    pub comentario: Option<String>,
    pub compra_verificada: bool,
    pub votos_util: i32,
    pub votos_no_util: i32,
    pub aprobado: bool,
    pub fecha_creacion: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usuario_nombre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usuario_apellido: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imagenes: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct CrearValoracionRequest {
    pub calificacion: i32,
    pub titulo: Option<String>,
    pub comentario: Option<String>,
}
