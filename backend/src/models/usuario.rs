use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Usuario {
    pub id_usuario: i32,
    pub nombre: String,
    pub apellido: String,
    pub email: String,
    pub contrasena: String,
    pub telefono: Option<String>,
    pub dni: Option<String>,
    pub rol: String,
    pub email_verificado: bool,
    pub activo: bool,
    pub fecha_registro: NaiveDateTime,
    pub ultima_conexion: Option<NaiveDateTime>,
    pub fecha_actualizacion: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub nombre: String,
    pub apellido: String,
    pub email: String,
    pub telefono: Option<String>,
    pub dni: Option<String>,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct UsuarioPublic {
    pub id_usuario: i32,
    pub nombre: String,
    pub apellido: String,
    pub email: String,
    pub rol: String,
}

impl From<Usuario> for UsuarioPublic {
    fn from(u: Usuario) -> Self {
        Self {
            id_usuario: u.id_usuario,
            nombre: u.nombre,
            apellido: u.apellido,
            email: u.email,
            rol: u.rol,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct AuthResponse {
    pub token: String,
    pub usuario: UsuarioPublic,
}
