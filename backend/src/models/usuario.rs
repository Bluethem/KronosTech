use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// ==================== MODELO PRINCIPAL ====================

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Usuario {
    pub id_usuario: i32,
    pub nombre: String,
    pub apellido: String,
    pub email: String,
    #[serde(skip_serializing)] // Nunca enviar contraseña en respuestas
    pub contrasena: String,
    pub telefono: Option<String>,
    pub dni: Option<String>,
    pub rol: String, // 'cliente', 'administrador', 'super_admin'
    pub email_verificado: bool,
    pub token_verificacion: Option<String>,
    pub activo: bool,
    pub fecha_registro: DateTime<Utc>,
    pub ultima_conexion: Option<DateTime<Utc>>,
    pub fecha_actualizacion: DateTime<Utc>,
}

// ==================== DTOs ====================

#[derive(Debug, Serialize)]
pub struct UsuarioResponse {
    pub id_usuario: i32,
    pub nombre: String,
    pub apellido: String,
    pub email: String,
    pub telefono: Option<String>,
    pub dni: Option<String>,
    pub rol: String,
    pub email_verificado: bool,
    pub activo: bool,
    pub fecha_registro: DateTime<Utc>,
}

impl From<Usuario> for UsuarioResponse {
    fn from(usuario: Usuario) -> Self {
        UsuarioResponse {
            id_usuario: usuario.id_usuario,
            nombre: usuario.nombre,
            apellido: usuario.apellido,
            email: usuario.email,
            telefono: usuario.telefono,
            dni: usuario.dni,
            rol: usuario.rol,
            email_verificado: usuario.email_verificado,
            activo: usuario.activo,
            fecha_registro: usuario.fecha_registro,
        }
    }
}

// ==================== REQUEST DTOs ====================

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    pub remember_me: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub nombre: String,
    pub apellido: String,
    pub email: String,
    pub telefono: Option<String>,
    pub dni: Option<String>,
    pub password: String,
}

// ==================== RESPONSE DTOs ====================

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub usuario: UsuarioResponse,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub message: String,
    pub usuario: UsuarioResponse,
}
