use axum::{
    extract::{State, Path},
    http::{StatusCode, HeaderMap},
    Json,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::models::MetodoPagoCliente;
use crate::services::{AuthService, MetodoPagoClienteService};

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

// ==================== REQUESTS ====================

#[derive(Debug, Deserialize)]
pub struct CrearMetodoPagoRequest {
    pub id_metodo_pago: i32,
    pub tipo: String,
    pub token_pago: Option<String>,
    pub ultimos_4_digitos: Option<String>,
    pub marca: Option<String>,
    pub fecha_expiracion: Option<String>,
    pub nombre_titular: Option<String>,
    pub es_predeterminado: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ActualizarMetodoPagoRequest {
    pub ultimos_4_digitos: Option<String>,
    pub marca: Option<String>,
    pub fecha_expiracion: Option<String>,
    pub nombre_titular: Option<String>,
    pub es_predeterminado: Option<bool>,
}

// ==================== HANDLERS ====================

/// GET /api/metodos-pago-cliente
/// Obtener todos los métodos de pago del usuario
pub async fn get_metodos_pago_handler(
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

    // Verificar token y obtener claims
    let claims = AuthService::verify_token(token).map_err(|e| {
        (
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                success: false,
                message: e,
            }),
        )
    })?;

    match MetodoPagoClienteService::get_user_payment_methods(&pool, claims.sub).await {
        Ok(metodos) => Ok((
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: Some(metodos),
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

/// POST /api/metodos-pago-cliente
/// Crear un nuevo método de pago
pub async fn crear_metodo_pago_handler(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Json(payload): Json<CrearMetodoPagoRequest>,
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

    // Verificar token y obtener claims
    let claims = AuthService::verify_token(token).map_err(|e| {
        (
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                success: false,
                message: e,
            }),
        )
    })?;

    match MetodoPagoClienteService::create_payment_method(
        &pool,
        claims.sub,
        payload.id_metodo_pago,
        &payload.tipo,
        payload.token_pago.as_deref(),
        payload.ultimos_4_digitos.as_deref(),
        payload.marca.as_deref(),
        payload.fecha_expiracion.as_deref(),
        payload.nombre_titular.as_deref(),
        payload.es_predeterminado.unwrap_or(false),
    )
    .await
    {
        Ok(metodo) => Ok((
            StatusCode::CREATED,
            Json(ApiResponse {
                success: true,
                data: Some(metodo),
                message: Some("Método de pago creado exitosamente".to_string()),
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

/// PUT /api/metodos-pago-cliente/:id
/// Actualizar un método de pago
pub async fn actualizar_metodo_pago_handler(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Path(id): Path<i32>,
    Json(payload): Json<ActualizarMetodoPagoRequest>,
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

    // Verificar token y obtener claims
    let claims = AuthService::verify_token(token).map_err(|e| {
        (
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                success: false,
                message: e,
            }),
        )
    })?;

    match MetodoPagoClienteService::update_payment_method(
        &pool,
        id,
        claims.sub,
        payload.ultimos_4_digitos.as_deref(),
        payload.marca.as_deref(),
        payload.fecha_expiracion.as_deref(),
        payload.nombre_titular.as_deref(),
        payload.es_predeterminado,
    )
    .await
    {
        Ok(metodo) => Ok((
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: Some(metodo),
                message: Some("Método de pago actualizado exitosamente".to_string()),
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

/// DELETE /api/metodos-pago-cliente/:id
/// Eliminar un método de pago
pub async fn eliminar_metodo_pago_handler(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Path(id): Path<i32>,
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

    // Verificar token y obtener claims
    let claims = AuthService::verify_token(token).map_err(|e| {
        (
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                success: false,
                message: e,
            }),
        )
    })?;

    match MetodoPagoClienteService::delete_payment_method(&pool, id, claims.sub).await {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(ApiResponse::<()> {
                success: true,
                data: None,
                message: Some("Método de pago eliminado exitosamente".to_string()),
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
