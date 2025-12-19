use axum::{
    extract::{State, Path},
    http::{StatusCode, HeaderMap},
    Json,
    response::IntoResponse,
};
use serde::Serialize;
use sqlx::PgPool;
use std::collections::HashMap;

use crate::models::configuracion::{ConfiguracionSistema, ActualizarConfigRequest, ActualizarConfigBatchRequest, ConfigValue};
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

// ==================== HELPER FUNCTIONS ====================

fn extract_token(headers: &HeaderMap) -> Result<&str, (StatusCode, Json<ErrorResponse>)> {
    headers
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
        })
}

fn verify_admin(token: &str) -> Result<(i32, String), (StatusCode, Json<ErrorResponse>)> {
    let claims = AuthService::verify_token(token).map_err(|e| {
        (
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                success: false,
                message: e,
            }),
        )
    })?;

    if claims.rol != "super_admin" && claims.rol != "administrador" {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse {
                success: false,
                message: "Acceso denegado. Solo administradores pueden acceder".to_string(),
            }),
        ));
    }

    Ok((claims.sub, claims.rol))
}

// ==================== HANDLERS ====================

/// GET /api/config
/// Obtener toda la configuración (público - algunas claves, admin - todas)
pub async fn get_all_config_handler(
    State(pool): State<PgPool>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    // Intentar verificar si es admin para mostrar todas las configs
    let is_admin = if let Ok(token) = extract_token(&headers) {
        verify_admin(token).is_ok()
    } else {
        false
    };

    let configs = if is_admin {
        // Admin ve toda la configuración
        sqlx::query_as::<_, ConfiguracionSistema>(
            "SELECT * FROM configuracion_sistema ORDER BY categoria, clave"
        )
        .fetch_all(&pool)
        .await
    } else {
        // Público solo ve configuración pública (no email ni seguridad interna)
        sqlx::query_as::<_, ConfiguracionSistema>(
            "SELECT * FROM configuracion_sistema 
             WHERE categoria NOT IN ('email', 'seguridad')
             ORDER BY categoria, clave"
        )
        .fetch_all(&pool)
        .await
    };

    match configs {
        Ok(configs) => {
            let mut config_map: HashMap<String, ConfigValue> = HashMap::new();
            for config in configs {
                config_map.insert(config.clave.clone(), ConfigValue {
                    valor: config.valor,
                    tipo: config.tipo,
                    categoria: config.categoria,
                });
            }
            
            Ok((
                StatusCode::OK,
                Json(ApiResponse {
                    success: true,
                    data: Some(config_map),
                    message: None,
                }),
            ))
        }
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                success: false,
                message: format!("Error al obtener configuración: {}", err),
            }),
        )),
    }
}

/// GET /api/config/:clave
/// Obtener una configuración específica
pub async fn get_config_handler(
    State(pool): State<PgPool>,
    Path(clave): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    match sqlx::query_as::<_, ConfiguracionSistema>(
        "SELECT * FROM configuracion_sistema WHERE clave = $1"
    )
    .bind(&clave)
    .fetch_optional(&pool)
    .await
    {
        Ok(Some(config)) => Ok((
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: Some(config),
                message: None,
            }),
        )),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                success: false,
                message: format!("Configuración '{}' no encontrada", clave),
            }),
        )),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                success: false,
                message: format!("Error: {}", err),
            }),
        )),
    }
}

/// PUT /api/config/:clave
/// Actualizar una configuración (solo admin)
pub async fn update_config_handler(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Path(clave): Path<String>,
    Json(payload): Json<ActualizarConfigRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let token = extract_token(&headers)?;
    let (user_id, _) = verify_admin(token)?;

    // Validar que la clave existe
    let existing = sqlx::query_as::<_, ConfiguracionSistema>(
        "SELECT * FROM configuracion_sistema WHERE clave = $1"
    )
    .bind(&clave)
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                success: false,
                message: format!("Error: {}", e),
            }),
        )
    })?;

    if existing.is_none() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                success: false,
                message: format!("Configuración '{}' no encontrada", clave),
            }),
        ));
    }

    // Actualizar
    match sqlx::query_as::<_, ConfiguracionSistema>(
        "UPDATE configuracion_sistema 
         SET valor = $1, fecha_actualizacion = CURRENT_TIMESTAMP, actualizado_por = $2
         WHERE clave = $3
         RETURNING *"
    )
    .bind(&payload.valor)
    .bind(user_id)
    .bind(&clave)
    .fetch_one(&pool)
    .await
    {
        Ok(config) => Ok((
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: Some(config),
                message: Some("Configuración actualizada".to_string()),
            }),
        )),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                success: false,
                message: format!("Error al actualizar: {}", err),
            }),
        )),
    }
}

/// PUT /api/config
/// Actualizar múltiples configuraciones (solo admin)
pub async fn update_config_batch_handler(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Json(payload): Json<ActualizarConfigBatchRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let token = extract_token(&headers)?;
    let (user_id, _) = verify_admin(token)?;

    let mut updated_count = 0;
    let mut errors: Vec<String> = Vec::new();

    for config in payload.configuraciones {
        match sqlx::query(
            "UPDATE configuracion_sistema 
             SET valor = $1, fecha_actualizacion = CURRENT_TIMESTAMP, actualizado_por = $2
             WHERE clave = $3"
        )
        .bind(&config.valor)
        .bind(user_id)
        .bind(&config.clave)
        .execute(&pool)
        .await
        {
            Ok(result) => {
                if result.rows_affected() > 0 {
                    updated_count += 1;
                } else {
                    errors.push(format!("Clave '{}' no encontrada", config.clave));
                }
            }
            Err(e) => {
                errors.push(format!("Error en '{}': {}", config.clave, e));
            }
        }
    }

    if errors.is_empty() {
        Ok((
            StatusCode::OK,
            Json(ApiResponse::<()> {
                success: true,
                data: None,
                message: Some(format!("{} configuraciones actualizadas", updated_count)),
            }),
        ))
    } else {
        Ok((
            StatusCode::OK,
            Json(ApiResponse::<()> {
                success: true,
                data: None,
                message: Some(format!("{} actualizadas. Errores: {}", updated_count, errors.join(", "))),
            }),
        ))
    }
}

/// GET /api/config/session-timeout
/// Obtener el timeout de sesión (usado por el servicio de auth)
pub async fn get_session_timeout_handler(
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    match sqlx::query_scalar::<_, String>(
        "SELECT valor FROM configuracion_sistema WHERE clave = 'session_timeout'"
    )
    .fetch_optional(&pool)
    .await
    {
        Ok(Some(valor)) => {
            let hours: i64 = valor.parse().unwrap_or(24);
            Ok((
                StatusCode::OK,
                Json(ApiResponse {
                    success: true,
                    data: Some(hours),
                    message: None,
                }),
            ))
        }
        Ok(None) => Ok((
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: Some(24i64), // Default 24 horas
                message: None,
            }),
        )),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                success: false,
                message: format!("Error: {}", err),
            }),
        )),
    }
}

