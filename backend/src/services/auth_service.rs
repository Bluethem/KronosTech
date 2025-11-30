use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::env;

use crate::models::{LoginRequest, LoginResponse, RegisterRequest, RegisterResponse, Usuario, UsuarioResponse};
use crate::repositories::AuthRepository;

// ==================== JWT CLAIMS ====================

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,         // user id
    pub email: String,
    pub rol: String,
    pub exp: usize,       // expiration time
    pub iat: usize,       // issued at
}

// ==================== SERVICE ====================

pub struct AuthService;

impl AuthService {
    // Registrar nuevo usuario
    pub async fn register(
        pool: &PgPool,
        request: RegisterRequest,
    ) -> Result<RegisterResponse, String> {
        // Validar email
        if !Self::is_valid_email(&request.email) {
            return Err("Email inválido".to_string());
        }

        // Validar contraseña
        if request.password.len() < 6 {
            return Err("La contraseña debe tener al menos 6 caracteres".to_string());
        }

        // Verificar si el email ya existe
        let email_exists = AuthRepository::email_exists(pool, &request.email)
            .await
            .map_err(|e| format!("Error al verificar email: {}", e))?;

        if email_exists {
            return Err("El email ya está registrado".to_string());
        }

        // Hash de la contraseña
        let password_hash = hash(&request.password, DEFAULT_COST)
            .map_err(|e| format!("Error al encriptar contraseña: {}", e))?;

        // Crear usuario
        let usuario = AuthRepository::create_user(
            pool,
            &request.nombre,
            &request.apellido,
            &request.email,
            &password_hash,
            request.telefono.as_deref(),
            request.dni.as_deref(),
        )
        .await
        .map_err(|e| format!("Error al crear usuario: {}", e))?;

        Ok(RegisterResponse {
            message: "Usuario registrado exitosamente".to_string(),
            usuario: UsuarioResponse::from(usuario),
        })
    }

    // Login de usuario
    pub async fn login(
        pool: &PgPool,
        request: LoginRequest,
    ) -> Result<LoginResponse, String> {
        // Buscar usuario por email
        let usuario = AuthRepository::find_by_email(pool, &request.email)
            .await
            .map_err(|e| format!("Error al buscar usuario: {}", e))?
            .ok_or("Credenciales inválidas".to_string())?;

        // Verificar que el usuario esté activo
        if !usuario.activo {
            return Err("La cuenta está desactivada".to_string());
        }

        // Verificar contraseña
        let password_match = verify(&request.password, &usuario.contrasena)
            .map_err(|e| format!("Error al verificar contraseña: {}", e))?;

        if !password_match {
            return Err("Credenciales inválidas".to_string());
        }

        // Actualizar última conexión
        let _ = AuthRepository::update_last_login(pool, usuario.id_usuario).await;

        // Generar token JWT
        let token = Self::generate_token(&usuario, request.remember_me.unwrap_or(false))?;

        Ok(LoginResponse {
            token,
            usuario: UsuarioResponse::from(usuario),
        })
    }

    // Generar token JWT
    fn generate_token(usuario: &Usuario, remember_me: bool) -> Result<String, String> {
        let jwt_secret = env::var("JWT_SECRET")
            .unwrap_or_else(|_| "default_secret_change_in_production".to_string());

        // Duración del token: 24 horas (o 30 días si remember_me)
        let expiration = if remember_me {
            chrono::Utc::now()
                .checked_add_signed(chrono::Duration::days(30))
                .expect("valid timestamp")
                .timestamp() as usize
        } else {
            chrono::Utc::now()
                .checked_add_signed(chrono::Duration::hours(24))
                .expect("valid timestamp")
                .timestamp() as usize
        };

        let claims = Claims {
            sub: usuario.id_usuario,
            email: usuario.email.clone(),
            rol: usuario.rol.clone(),
            exp: expiration,
            iat: chrono::Utc::now().timestamp() as usize,
        };

        encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(jwt_secret.as_bytes()),
        )
        .map_err(|e| format!("Error al generar token: {}", e))
    }

    // Verificar token JWT
    pub fn verify_token(token: &str) -> Result<Claims, String> {
        let jwt_secret = env::var("JWT_SECRET")
            .unwrap_or_else(|_| "default_secret_change_in_production".to_string());

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|e| format!("Token inválido: {}", e))?;

        Ok(token_data.claims)
    }

    // Obtener usuario actual por token
    pub async fn get_current_user(
        pool: &PgPool,
        token: &str,
    ) -> Result<UsuarioResponse, String> {
        let claims = Self::verify_token(token)?;

        let usuario = AuthRepository::find_by_id(pool, claims.sub)
            .await
            .map_err(|e| format!("Error al buscar usuario: {}", e))?
            .ok_or("Usuario no encontrado".to_string())?;

        if !usuario.activo {
            return Err("La cuenta está desactivada".to_string());
        }

        Ok(UsuarioResponse::from(usuario))
    }

    // Validar email
    fn is_valid_email(email: &str) -> bool {
        email.contains('@') && email.contains('.') && email.len() > 5
    }
}
