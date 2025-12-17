use axum::{
    extract::{State, Query},
    http::{StatusCode, HeaderMap},
    Json,
    response::IntoResponse,
};
use serde::Serialize;
use sqlx::PgPool;

use crate::models::log_auditoria::{LogAuditoria, CrearLogRequest, FiltrarLogsQuery, LogResponse};
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

/// Verificar si el usuario es admin o super_admin
fn verify_admin(token: &str) -> Result<(i32, String, String), (StatusCode, Json<ErrorResponse>)> {
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
                message: "Acceso denegado. Solo administradores pueden acceder a este recurso".to_string(),
            }),
        ));
    }

    Ok((claims.sub, claims.email, claims.rol))
}

/// Extraer token del header
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

/// Extraer IP del cliente desde headers
fn extract_client_ip(headers: &HeaderMap) -> Option<String> {
    headers
        .get("X-Forwarded-For")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.split(',').next().unwrap_or(s).trim().to_string())
        .or_else(|| {
            headers
                .get("X-Real-IP")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string())
        })
}

/// Extraer User-Agent del cliente
fn extract_user_agent(headers: &HeaderMap) -> Option<String> {
    headers
        .get("User-Agent")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
}

// ==================== HANDLERS ====================

/// GET /api/logs
/// Listar logs con filtros (solo admin/super_admin)
pub async fn listar_logs_handler(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Query(query): Query<FiltrarLogsQuery>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let token = extract_token(&headers)?;
    verify_admin(token)?;

    let limit = query.limit.unwrap_or(100).min(500);
    let offset = query.offset.unwrap_or(0);

    // Construir query dinámicamente
    let mut sql = String::from(
        "SELECT id_log, nivel::TEXT as nivel, accion, detalles, modulo, 
         id_usuario, email_usuario, ip_cliente, user_agent, fecha_creacion
         FROM log_auditoria WHERE 1=1"
    );
    
    let mut param_count = 0;
    
    if query.nivel.is_some() {
        param_count += 1;
        sql.push_str(&format!(" AND nivel = ${}::nivel_log", param_count));
    }
    
    if query.modulo.is_some() {
        param_count += 1;
        sql.push_str(&format!(" AND modulo = ${}", param_count));
    }
    
    if query.fecha_inicio.is_some() {
        param_count += 1;
        sql.push_str(&format!(" AND fecha_creacion >= ${}::timestamp", param_count));
    }
    
    if query.fecha_fin.is_some() {
        param_count += 1;
        sql.push_str(&format!(" AND fecha_creacion <= ${}::timestamp", param_count));
    }
    
    sql.push_str(&format!(" ORDER BY fecha_creacion DESC LIMIT ${} OFFSET ${}", param_count + 1, param_count + 2));

    // Ejecutar query con parámetros dinámicos
    let mut query_builder = sqlx::query_as::<_, LogAuditoria>(&sql);
    
    if let Some(nivel) = &query.nivel {
        query_builder = query_builder.bind(nivel);
    }
    if let Some(modulo) = &query.modulo {
        query_builder = query_builder.bind(modulo);
    }
    if let Some(fecha_inicio) = &query.fecha_inicio {
        query_builder = query_builder.bind(fecha_inicio);
    }
    if let Some(fecha_fin) = &query.fecha_fin {
        query_builder = query_builder.bind(fecha_fin);
    }
    
    query_builder = query_builder.bind(limit).bind(offset);

    match query_builder.fetch_all(&pool).await {
        Ok(logs) => {
            let logs_response: Vec<LogResponse> = logs.into_iter().map(LogResponse::from).collect();
            Ok((
                StatusCode::OK,
                Json(ApiResponse {
                    success: true,
                    data: Some(logs_response),
                    message: None,
                }),
            ))
        }
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                success: false,
                message: format!("Error al listar logs: {}", err),
            }),
        )),
    }
}

/// POST /api/logs
/// Crear nuevo log (autenticado - admin/super_admin o sistema)
pub async fn crear_log_handler(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Json(payload): Json<CrearLogRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    // Intentar obtener usuario del token (opcional para logs del sistema)
    let (id_usuario, email_usuario) = if let Ok(token) = extract_token(&headers) {
        if let Ok((id, email, _)) = verify_admin(token) {
            (Some(id), Some(email))
        } else {
            (None, payload.email_usuario.clone())
        }
    } else {
        (None, payload.email_usuario.clone())
    };

    let ip_cliente = payload.ip_cliente.clone().or_else(|| extract_client_ip(&headers));
    let user_agent = payload.user_agent.clone().or_else(|| extract_user_agent(&headers));

    // Validar nivel
    let niveles_validos = ["info", "warning", "error", "success", "security"];
    if !niveles_validos.contains(&payload.nivel.as_str()) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                message: format!("Nivel inválido. Debe ser uno de: {:?}", niveles_validos),
            }),
        ));
    }

    match sqlx::query_as::<_, LogAuditoria>(
        "INSERT INTO log_auditoria (nivel, accion, detalles, modulo, id_usuario, email_usuario, ip_cliente, user_agent)
         VALUES ($1::nivel_log, $2, $3, $4, $5, $6, $7, $8)
         RETURNING id_log, nivel::TEXT as nivel, accion, detalles, modulo, id_usuario, email_usuario, ip_cliente, user_agent, fecha_creacion"
    )
    .bind(&payload.nivel)
    .bind(&payload.accion)
    .bind(&payload.detalles)
    .bind(&payload.modulo)
    .bind(id_usuario)
    .bind(&email_usuario)
    .bind(&ip_cliente)
    .bind(&user_agent)
    .fetch_one(&pool)
    .await
    {
        Ok(log) => Ok((
            StatusCode::CREATED,
            Json(ApiResponse {
                success: true,
                data: Some(LogResponse::from(log)),
                message: Some("Log creado exitosamente".to_string()),
            }),
        )),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                success: false,
                message: format!("Error al crear log: {}", err),
            }),
        )),
    }
}

/// DELETE /api/logs
/// Limpiar todos los logs (solo super_admin)
pub async fn limpiar_logs_handler(
    State(pool): State<PgPool>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let token = extract_token(&headers)?;
    let (_, email, rol) = verify_admin(token)?;

    // Solo super_admin puede limpiar logs
    if rol != "super_admin" {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse {
                success: false,
                message: "Solo super_admin puede eliminar logs".to_string(),
            }),
        ));
    }

    // Eliminar todos los logs
    match sqlx::query("DELETE FROM log_auditoria")
        .execute(&pool)
        .await
    {
        Ok(result) => {
            let deleted_count = result.rows_affected();
            
            // Crear log de la acción de limpieza
            let _ = sqlx::query(
                "INSERT INTO log_auditoria (nivel, accion, detalles, modulo, email_usuario)
                 VALUES ('warning'::nivel_log, 'Logs eliminados', $1, 'Sistema', $2)"
            )
            .bind(format!("Se eliminaron {} logs del sistema", deleted_count))
            .bind(&email)
            .execute(&pool)
            .await;

            Ok((
                StatusCode::OK,
                Json(ApiResponse::<()> {
                    success: true,
                    data: None,
                    message: Some(format!("Se eliminaron {} logs", deleted_count)),
                }),
            ))
        }
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                success: false,
                message: format!("Error al eliminar logs: {}", err),
            }),
        )),
    }
}


