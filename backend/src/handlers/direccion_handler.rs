use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use sqlx::PgPool;

use crate::models::{CrearDireccionRequest, ActualizarDireccionRequest};
use crate::services::{AuthService, DireccionService};

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

// ==================== HELPER FUNCTIONS ====================

fn extract_user_id(headers: &HeaderMap) -> Result<i32, (StatusCode, Json<ErrorResponse>)> {
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

    let claims = AuthService::verify_token(token).map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                success: false,
                message: "Token inválido o expirado".to_string(),
            }),
        )
    })?;

    Ok(claims.sub)
}

// ==================== HANDLERS ====================

/// GET /api/direcciones - Obtener direcciones del usuario
pub async fn get_direcciones_handler(
    State(pool): State<PgPool>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let id_usuario = extract_user_id(&headers)?;

    match DireccionService::get_direcciones(&pool, id_usuario).await {
        Ok(direcciones) => Ok((
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: Some(direcciones),
                message: None,
            }),
        )),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                success: false,
                message: err,
            }),
        )),
    }
}

/// POST /api/direcciones - Crear nueva dirección
pub async fn crear_direccion_handler(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Json(payload): Json<CrearDireccionRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let id_usuario = extract_user_id(&headers)?;

    match DireccionService::crear_direccion(&pool, id_usuario, payload).await {
        Ok(direccion) => Ok((
            StatusCode::CREATED,
            Json(ApiResponse {
                success: true,
                data: Some(direccion),
                message: Some("Dirección creada exitosamente".to_string()),
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

/// PUT /api/direcciones/{id} - Actualizar dirección
pub async fn actualizar_direccion_handler(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Path(id_direccion): Path<i32>,
    Json(payload): Json<ActualizarDireccionRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let id_usuario = extract_user_id(&headers)?;

    match DireccionService::actualizar_direccion(&pool, id_usuario, id_direccion, payload).await {
        Ok(direccion) => Ok((
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: Some(direccion),
                message: Some("Dirección actualizada exitosamente".to_string()),
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

/// DELETE /api/direcciones/{id} - Eliminar dirección
pub async fn eliminar_direccion_handler(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Path(id_direccion): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let id_usuario = extract_user_id(&headers)?;

    match DireccionService::eliminar_direccion(&pool, id_usuario, id_direccion).await {
        Ok(()) => Ok((
            StatusCode::OK,
            Json(ApiResponse::<()> {
                success: true,
                data: None,
                message: Some("Dirección eliminada exitosamente".to_string()),
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
