use axum::{
    extract::State,
    http::{StatusCode, HeaderMap},
    Json,
    response::IntoResponse,
};
use serde::Serialize;
use sqlx::PgPool;

use crate::models::{LoginRequest, RegisterRequest};
use crate::services::AuthService;

// ==================== RESPONSES ====================

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub message: String,
}

// ==================== HANDLERS ====================

// POST /api/auth/register
pub async fn register_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    match AuthService::register(&pool, payload).await {
        Ok(response) => Ok((
            StatusCode::CREATED,
            Json(ApiResponse {
                success: true,
                data: Some(response),
                message: None,
            }),
        )),
        Err(err) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                message: err,
            }),
        )),
    }
}

// POST /api/auth/login
pub async fn login_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    match AuthService::login(&pool, payload).await {
        Ok(response) => Ok((
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: Some(response),
                message: None,
            }),
        )),
        Err(err) => Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                success: false,
                message: err,
            }),
        )),
    }
}

// GET /api/auth/me
pub async fn get_current_user_handler(
    State(pool): State<PgPool>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    // Extraer token del header Authorization
    let token = headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| {
            if value.starts_with("Bearer ") {
                Some(&value[7..])
            } else {
                None
            }
        })
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    success: false,
                    message: "Token no proporcionado".to_string(),
                }),
            )
        })?;

    match AuthService::get_current_user(&pool, token).await {
        Ok(usuario) => Ok((
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: Some(usuario),
                message: None,
            }),
        )),
        Err(err) => Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                success: false,
                message: err,
            }),
        )),
    }
}

// POST /api/auth/logout
// El logout se maneja en el frontend eliminando el token
// Este endpoint es opcional, podría usarse para invalidar tokens en el futuro
pub async fn logout_handler() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(ApiResponse::<()> {
            success: true,
            data: None,
            message: Some("Sesión cerrada exitosamente".to_string()),
        }),
    )
}

// ==================== PERFIL ====================

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ActualizarPerfilRequest {
    pub nombre: Option<String>,
    pub apellido: Option<String>,
    pub telefono: Option<String>,
    pub dni: Option<String>,
}

// PUT /api/auth/perfil
pub async fn actualizar_perfil_handler(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Json(payload): Json<ActualizarPerfilRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    // Extraer token del header Authorization
    let token = headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| {
            if value.starts_with("Bearer ") {
                Some(&value[7..])
            } else {
                None
            }
        })
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    success: false,
                    message: "Token no proporcionado".to_string(),
                }),
            )
        })?;

    match AuthService::actualizar_perfil(&pool, token, payload).await {
        Ok(usuario) => Ok((
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: Some(usuario),
                message: Some("Perfil actualizado exitosamente".to_string()),
            }),
        )),
        Err(err) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                message: err,
            }),
        )),
    }
}

#[derive(Debug, Deserialize)]
pub struct CambiarPasswordRequest {
    pub password_actual: String,
    pub password_nuevo: String,
}

// PUT /api/auth/cambiar-password
pub async fn cambiar_password_handler(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Json(payload): Json<CambiarPasswordRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    // Extraer token del header Authorization
    let token = headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| {
            if value.starts_with("Bearer ") {
                Some(&value[7..])
            } else {
                None
            }
        })
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    success: false,
                    message: "Token no proporcionado".to_string(),
                }),
            )
        })?;

    match AuthService::cambiar_password(&pool, token, payload).await {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(ApiResponse::<()> {
                success: true,
                data: None,
                message: Some("Contraseña actualizada exitosamente".to_string()),
            }),
        )),
        Err(err) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                message: err,
            }),
        )),
    }
}
