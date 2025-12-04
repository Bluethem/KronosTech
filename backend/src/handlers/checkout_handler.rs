use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::models::ProcesarCheckoutRequest;
use crate::services::{AuthService, CheckoutService};

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

// ==================== QUERY PARAMS ====================

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CalcularTotalQuery {
    pub id_direccion: Option<i32>,
    pub codigo_cupon: Option<String>,
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

fn extract_ip_and_user_agent(headers: &HeaderMap) -> (Option<String>, Option<String>) {
    let ip = headers
        .get("X-Forwarded-For")
        .or_else(|| headers.get("X-Real-IP"))
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let user_agent = headers
        .get("User-Agent")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    (ip, user_agent)
}

// ==================== HANDLERS ====================

/// GET /api/metodos-pago - Obtener métodos de pago disponibles
pub async fn get_metodos_pago_handler(
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    match CheckoutService::get_metodos_pago(&pool).await {
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

/// GET /api/checkout/calcular-total - Calcular total del checkout
pub async fn calcular_total_handler(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Query(params): Query<CalcularTotalQuery>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let id_usuario = extract_user_id(&headers)?;

    match CheckoutService::calcular_total(&pool, id_usuario, params.id_direccion, params.codigo_cupon).await {
        Ok(totales) => Ok((
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: Some(totales),
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

/// POST /api/checkout/procesar - Procesar checkout y crear pedido
pub async fn procesar_checkout_handler(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Json(payload): Json<ProcesarCheckoutRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let id_usuario = extract_user_id(&headers)?;
    let (ip_cliente, user_agent) = extract_ip_and_user_agent(&headers);

    match CheckoutService::procesar_checkout(&pool, id_usuario, payload, ip_cliente, user_agent).await {
        Ok(venta) => Ok((
            StatusCode::CREATED,
            Json(ApiResponse {
                success: true,
                data: Some(venta),
                message: Some("Pedido creado exitosamente".to_string()),
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

/// GET /api/pedidos - Listar pedidos del usuario
pub async fn get_pedidos_handler(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Query(params): Query<PaginationQuery>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let id_usuario = extract_user_id(&headers)?;

    match CheckoutService::get_pedidos_usuario(&pool, id_usuario, params.limit, params.offset).await {
        Ok(pedidos) => Ok((
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: Some(pedidos),
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

/// GET /api/pedidos/{id} - Obtener detalle de un pedido
pub async fn get_pedido_handler(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Path(id_venta): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let id_usuario = extract_user_id(&headers)?;

    match CheckoutService::get_pedido(&pool, id_venta, id_usuario).await {
        Ok(pedido) => Ok((
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: Some(pedido),
                message: None,
            }),
        )),
        Err(err) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                success: false,
                message: err,
            }),
        )),
    }
}
