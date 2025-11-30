use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use sqlx::PgPool;

use crate::models::{AgregarAlCarritoRequest, ActualizarCantidadRequest};
use crate::services::{AuthService, CarritoService};

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

// Extraer ID de usuario del token JWT
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

// GET /api/carrito
pub async fn get_carrito_handler(
    State(pool): State<PgPool>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let id_usuario = extract_user_id(&headers)?;

    match CarritoService::get_carrito(&pool, id_usuario).await {
        Ok(carrito) => Ok((
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: Some(carrito),
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

// POST /api/carrito/items
pub async fn agregar_item_handler(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Json(payload): Json<AgregarAlCarritoRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let id_usuario = extract_user_id(&headers)?;

    match CarritoService::agregar_producto(&pool, id_usuario, payload).await {
        Ok(carrito) => Ok((
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: Some(carrito),
                message: Some("Producto agregado al carrito".to_string()),
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

// PATCH /api/carrito/items/:id
pub async fn actualizar_cantidad_handler(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Path(id_carrito_detalle): Path<i32>,
    Json(payload): Json<ActualizarCantidadRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let id_usuario = extract_user_id(&headers)?;

    match CarritoService::actualizar_cantidad(&pool, id_usuario, id_carrito_detalle, payload).await
    {
        Ok(carrito) => Ok((
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: Some(carrito),
                message: Some("Cantidad actualizada".to_string()),
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

// DELETE /api/carrito/items/:id
pub async fn eliminar_item_handler(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Path(id_carrito_detalle): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let id_usuario = extract_user_id(&headers)?;

    match CarritoService::eliminar_item(&pool, id_usuario, id_carrito_detalle).await {
        Ok(carrito) => Ok((
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: Some(carrito),
                message: Some("Item eliminado del carrito".to_string()),
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

// DELETE /api/carrito
pub async fn limpiar_carrito_handler(
    State(pool): State<PgPool>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let id_usuario = extract_user_id(&headers)?;

    match CarritoService::limpiar_carrito(&pool, id_usuario).await {
        Ok(carrito) => Ok((
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: Some(carrito),
                message: Some("Carrito limpiado".to_string()),
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
