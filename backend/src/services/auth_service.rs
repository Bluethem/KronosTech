use anyhow::{anyhow, Result};
use bcrypt::{hash, verify};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Serialize;

use crate::models::usuario::{AuthResponse, LoginRequest, RegisterRequest, UsuarioPublic};
use crate::repositories::UsuarioRepository;

#[derive(Serialize)]
struct Claims {
    sub: i32,
    email: String,
    rol: String,
    exp: i64,
}

pub struct AuthService<'a> {
    repo: UsuarioRepository<'a>,
    jwt_key: EncodingKey,
}

impl<'a> AuthService<'a> {
    pub fn new(repo: UsuarioRepository<'a>, secret: &str) -> Self {
        Self {
            repo,
            jwt_key: EncodingKey::from_secret(secret.as_bytes()),
        }
    }

    pub async fn register(&self, mut req: RegisterRequest) -> Result<AuthResponse> {
        req.email = req.email.trim().to_lowercase();

        if self.repo.find_by_email(&req.email).await?.is_some() {
            return Err(anyhow!("email_ya_registrado"));
        }

        let hash = hash(&req.password, 12)?;
        let usuario = self.repo.create(&req, &hash).await?;
        let token = self.issue_token(usuario.id_usuario, &usuario.email, &usuario.rol)?;

        Ok(AuthResponse {
            token,
            usuario: UsuarioPublic::from(usuario),
        })
    }

    pub async fn login(&self, mut req: LoginRequest) -> Result<AuthResponse> {
        req.email = req.email.trim().to_lowercase();

        let usuario = self
            .repo
            .find_by_email(&req.email)
            .await?
            .ok_or_else(|| anyhow!("credenciales_invalidas"))?;

        if !usuario.activo {
            return Err(anyhow!("usuario_inactivo"));
        }

        if !verify(&req.password, &usuario.contrasena)? {
            return Err(anyhow!("credenciales_invalidas"));
        }

        let _ = self.repo.actualizar_ultima_conexion(usuario.id_usuario).await;
        let token = self.issue_token(usuario.id_usuario, &usuario.email, &usuario.rol)?;

        Ok(AuthResponse {
            token,
            usuario: UsuarioPublic::from(usuario),
        })
    }

    fn issue_token(&self, id_usuario: i32, email: &str, rol: &str) -> Result<String> {
        let exp = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .ok_or_else(|| anyhow!("no_se_puede_generar_token"))?
            .timestamp();

        let claims = Claims {
            sub: id_usuario,
            email: email.to_string(),
            rol: rol.to_string(),
            exp,
        };

        Ok(encode(&Header::default(), &claims, &self.jwt_key)?)
    }
}
