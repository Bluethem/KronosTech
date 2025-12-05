use axum::{
    extract::{Query, State, Path},
    http::{StatusCode, HeaderMap},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use chrono::{NaiveDateTime, Datelike};
use rust_decimal::Decimal;
use crate::services::AuthService;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ReembolsoWithDetails {
    pub id_reembolso: i32,
    pub numero_pedido: String,
    pub id_venta: i32,
    pub nombre_cliente: Option<String>,
    pub email_cliente: Option<String>,
    pub tipo_reembolso: String,
    pub monto_reembolsado: Decimal,
    pub estado: Option<String>,
    pub fecha_solicitado: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct ReembolsoQuery {
    pub estado: Option<String>,
    pub tipo: Option<String>,
    pub search: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

// ==================== HELPER FUNCTIONS ====================

#[derive(Debug, serde::Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub message: String,
}

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

pub async fn get_reembolsos(
    State(pool): State<PgPool>,
    Query(query): Query<ReembolsoQuery>,
) -> Result<Json<Vec<ReembolsoWithDetails>>, StatusCode> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(50);
    let offset = (page - 1) * limit;

    let mut sql = String::from(
        r#"
        SELECT 
            r.id_reembolso,
            v.numero_pedido,
            r.id_venta,
            u.nombre as nombre_cliente,
            u.email as email_cliente,
            r.tipo_reembolso,
            r.monto_reembolsado,
            r.estado::text,
            r.fecha_solicitado
        FROM reembolso r
        JOIN venta v ON r.id_venta = v.id_venta
        JOIN usuario u ON v.id_usuario = u.id_usuario
        WHERE 1=1
        "#,
    );

    // Add filters
    if let Some(estado) = &query.estado {
        if estado != "Todos" {
            sql.push_str(&format!(" AND r.estado = '{}'", estado.to_lowercase()));
        }
    }

    if let Some(tipo) = &query.tipo {
        if tipo != "Todos" {
            sql.push_str(&format!(" AND r.tipo_reembolso = '{}'", tipo.to_lowercase()));
        }
    }

    if let Some(search) = &query.search {
        if !search.is_empty() {
            sql.push_str(&format!(
                " AND (v.numero_pedido ILIKE '%{}%' OR u.nombre ILIKE '%{}%' OR u.email ILIKE '%{}%')",
                search, search, search
            ));
        }
    }

    sql.push_str(" ORDER BY r.fecha_solicitado DESC NULLS LAST");
    sql.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));

    match sqlx::query_as::<_, ReembolsoWithDetails>(&sql)
        .fetch_all(&pool)
        .await
    {
        Ok(reembolsos) => Ok(Json(reembolsos)),
        Err(e) => {
            eprintln!("Error fetching reembolsos: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReembolsoStats {
    pub pendientes: i64,
    pub completados_mes: i64,
    pub monto_total_mes: Decimal,
    pub cambio_pendientes: i64,
    pub cambio_completados: f64,
    pub cambio_monto: f64,
}

pub async fn get_reembolso_stats(
    State(pool): State<PgPool>,
) -> Result<Json<ReembolsoStats>, StatusCode> {
    // Get current month start and end
    let now = chrono::Local::now().naive_local();
    let mes_actual_inicio = chrono::NaiveDate::from_ymd_opt(now.year(), now.month0() + 1, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    let mes_actual_fin = now;

    // Get previous month start and end
    let mes_anterior = if now.month0() == 0 {
        chrono::NaiveDate::from_ymd_opt(now.year() - 1, 12, 1)
    } else {
        chrono::NaiveDate::from_ymd_opt(now.year(), now.month0(), 1)
    }
    .unwrap()
    .and_hms_opt(0, 0, 0)
    .unwrap();

    // Count pending refunds
    let pendientes: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) 
        FROM reembolso 
        WHERE estado = 'solicitado'
        "#,
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        eprintln!("Error fetching pendientes: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Count completed this month
    let completados_mes: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) 
        FROM reembolso 
        WHERE estado = 'completado' 
        AND fecha_completado BETWEEN $1 AND $2
        "#,
    )
    .bind(mes_actual_inicio)
    .bind(mes_actual_fin)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        eprintln!("Error fetching completados_mes: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Total amount refunded this month
    let monto_total_mes: (Option<Decimal>,) = sqlx::query_as(
        r#"
        SELECT COALESCE(SUM(monto_reembolsado), 0) 
        FROM reembolso 
        WHERE estado = 'completado' 
        AND fecha_completado BETWEEN $1 AND $2
        "#,
    )
    .bind(mes_actual_inicio)
    .bind(mes_actual_fin)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        eprintln!("Error fetching monto_total_mes: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Previous month stats for comparison
    let completados_mes_anterior: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) 
        FROM reembolso 
        WHERE estado = 'completado' 
        AND fecha_completado >= $1 
        AND fecha_completado < $2
        "#,
    )
    .bind(mes_anterior)
    .bind(mes_actual_inicio)
    .fetch_one(&pool)
    .await
    .unwrap_or((0,));

    let monto_mes_anterior: (Option<Decimal>,) = sqlx::query_as(
        r#"
        SELECT COALESCE(SUM(monto_reembolsado), 0) 
        FROM reembolso 
        WHERE estado = 'completado' 
        AND fecha_completado >= $1 
        AND fecha_completado < $2
        "#,
    )
    .bind(mes_anterior)
    .bind(mes_actual_inicio)
    .fetch_one(&pool)
    .await
    .unwrap_or((Some(Decimal::ZERO),));

    // Calculate percentage changes
    let cambio_completados = if completados_mes_anterior.0 > 0 {
        ((completados_mes.0 as f64 - completados_mes_anterior.0 as f64) / completados_mes_anterior.0 as f64) * 100.0
    } else {
        0.0
    };

    let monto_anterior_f64 = monto_mes_anterior.0.unwrap_or(Decimal::ZERO).to_string().parse::<f64>().unwrap_or(0.0);
    let monto_actual_f64 = monto_total_mes.0.unwrap_or(Decimal::ZERO).to_string().parse::<f64>().unwrap_or(0.0);
    
    let cambio_monto = if monto_anterior_f64 > 0.0 {
        ((monto_actual_f64 - monto_anterior_f64) / monto_anterior_f64) * 100.0
    } else {
        0.0
    };

    Ok(Json(ReembolsoStats {
        pendientes: pendientes.0,
        completados_mes: completados_mes.0,
        monto_total_mes: monto_total_mes.0.unwrap_or(Decimal::ZERO),
        cambio_pendientes: 0, // Could calculate week-over-week if needed
        cambio_completados,
        cambio_monto,
    }))
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ReembolsoDetalle {
    pub id_reembolso: i32,
    pub numero_pedido: String,
    pub id_venta: i32,
    pub nombre_cliente: Option<String>,
    pub email_cliente: Option<String>,
    pub telefono_cliente: Option<String>,
    pub tipo_reembolso: String,
    pub monto_reembolsado: Decimal,
    pub motivo: String,
    pub estado: Option<String>,
    pub fecha_solicitado: Option<NaiveDateTime>,
    pub fecha_aprobado: Option<NaiveDateTime>,
    pub fecha_completado: Option<NaiveDateTime>,
    pub notas_admin: Option<String>,
    pub nombre_aprobador: Option<String>,
    pub total_venta: Decimal,
    pub fecha_pedido: Option<NaiveDateTime>,
    pub metodo_pago: Option<String>,
}

pub async fn get_reembolso_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<ReembolsoDetalle>, StatusCode> {
    let reembolso = sqlx::query_as::<_, ReembolsoDetalle>(
        r#"
        SELECT 
            r.id_reembolso,
            v.numero_pedido,
            r.id_venta,
            u.nombre as nombre_cliente,
            u.email as email_cliente,
            u.telefono as telefono_cliente,
            r.tipo_reembolso,
            r.monto_reembolsado,
            r.motivo,
            r.estado::text,
            r.fecha_solicitado,
            r.fecha_aprobado,
            r.fecha_completado,
            r.notas_admin,
            ua.nombre as nombre_aprobador,
            v.total as total_venta,
            v.fecha_pedido,
            mp.nombre as metodo_pago
        FROM reembolso r
        JOIN venta v ON r.id_venta = v.id_venta
        JOIN usuario u ON v.id_usuario = u.id_usuario
        LEFT JOIN usuario ua ON r.id_usuario_aprobador = ua.id_usuario
        LEFT JOIN pago p ON r.id_pago = p.id_pago
        LEFT JOIN metodo_pago mp ON p.id_metodo_pago = mp.id_metodo_pago
        WHERE r.id_reembolso = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        eprintln!("Error fetching reembolso: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(reembolso))
}

#[derive(Debug, Deserialize)]
pub struct ProcesarReembolsoRequest {
    pub decision: String, // "aprobar" o "rechazar"
    pub monto_final: Option<f64>,
    pub notas_admin: Option<String>,
}

pub async fn procesar_reembolso(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<ProcesarReembolsoRequest>,
) -> Result<StatusCode, StatusCode> {
    let nuevo_estado = match payload.decision.as_str() {
        "aprobar" => "completado",
        "rechazar" => "rechazado",
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    let fecha_campo = if nuevo_estado == "completado" {
        "fecha_completado"
    } else {
        "fecha_aprobado"
    };

    let mut query = format!(
        "UPDATE reembolso SET estado = $1::estado_reembolso, {} = CURRENT_TIMESTAMP",
        fecha_campo
    );

    if let Some(notas) = &payload.notas_admin {
        query.push_str(", notas_admin = $2 WHERE id_reembolso = $3");
        
        sqlx::query(&query)
            .bind(nuevo_estado)
            .bind(notas)
            .bind(id)
            .execute(&pool)
            .await
            .map_err(|e| {
                eprintln!("Error updating reembolso: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
    } else {
        query.push_str(" WHERE id_reembolso = $2");
        
        sqlx::query(&query)
            .bind(nuevo_estado)
            .bind(id)
            .execute(&pool)
            .await
            .map_err(|e| {
                eprintln!("Error updating reembolso: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
    }

    Ok(StatusCode::OK)
}

// ==================== CLIENTE: SOLICITAR REEMBOLSO ====================

#[derive(Debug, Deserialize)]
pub struct SolicitarReembolsoRequest {
    pub id_venta: i32,
    pub tipo_reembolso: String, // "total" o "parcial"
    pub monto_reembolsado: Decimal,
    pub motivo: String,
}

#[derive(Debug, Serialize)]
pub struct SolicitarReembolsoResponse {
    pub success: bool,
    pub message: String,
    pub id_reembolso: Option<i32>,
}

pub async fn solicitar_reembolso(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Json(payload): Json<SolicitarReembolsoRequest>,
) -> Result<Json<SolicitarReembolsoResponse>, (StatusCode, Json<SolicitarReembolsoResponse>)> {
    // Extraer ID de usuario del token JWT
    let id_usuario_autenticado = match extract_user_id(&headers) {
        Ok(id) => id,
        Err((status, Json(err))) => {
            return Err((
                status,
                Json(SolicitarReembolsoResponse {
                    success: false,
                    message: err.message,
                    id_reembolso: None,
                }),
            ));
        }
    };

    // Validar tipo de reembolso
    if payload.tipo_reembolso != "total" && payload.tipo_reembolso != "parcial" {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(SolicitarReembolsoResponse {
                success: false,
                message: "Tipo de reembolso inválido. Debe ser 'total' o 'parcial'".to_string(),
                id_reembolso: None,
            }),
        ));
    }

    // Validar que el monto sea positivo
    if payload.monto_reembolsado <= Decimal::ZERO {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(SolicitarReembolsoResponse {
                success: false,
                message: "El monto debe ser mayor a 0".to_string(),
                id_reembolso: None,
            }),
        ));
    }

    // Obtener información de la venta y verificar que existe
    let venta: Result<(i32, Decimal, i32), _> = sqlx::query_as(
        r#"
        SELECT v.id_usuario, v.total, p.id_pago
        FROM venta v
        LEFT JOIN pago p ON p.id_venta = v.id_venta
        WHERE v.id_venta = $1
        LIMIT 1
        "#,
    )
    .bind(payload.id_venta)
    .fetch_one(&pool)
    .await;

    let (id_usuario_venta, total_venta, id_pago) = match venta {
        Ok(data) => data,
        Err(_) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(SolicitarReembolsoResponse {
                    success: false,
                    message: "Pedido no encontrado".to_string(),
                    id_reembolso: None,
                }),
            ));
        }
    };

    // Validar que el pedido pertenezca al usuario autenticado
    if id_usuario_venta != id_usuario_autenticado {
        return Err((
            StatusCode::FORBIDDEN,
            Json(SolicitarReembolsoResponse {
                success: false,
                message: "No tienes permiso para solicitar reembolso de este pedido".to_string(),
                id_reembolso: None,
            }),
        ));
    }

    // Validar que el monto no exceda el total de la venta
    if payload.monto_reembolsado > total_venta {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(SolicitarReembolsoResponse {
                success: false,
                message: format!(
                    "El monto solicitado (S/. {}) excede el total del pedido (S/. {})",
                    payload.monto_reembolsado, total_venta
                ),
                id_reembolso: None,
            }),
        ));
    }

    // Verificar que no exista ya un reembolso solicitado para esta venta
    let existing: Result<(i64,), _> = sqlx::query_as(
        r#"
        SELECT COUNT(*)
        FROM reembolso
        WHERE id_venta = $1 AND estado IN ('solicitado', 'procesando', 'completado')
        "#,
    )
    .bind(payload.id_venta)
    .fetch_one(&pool)
    .await;

    if let Ok((count,)) = existing {
        if count > 0 {
            return Err((
                StatusCode::CONFLICT,
                Json(SolicitarReembolsoResponse {
                    success: false,
                    message: "Ya existe una solicitud de reembolso activa para este pedido".to_string(),
                    id_reembolso: None,
                }),
            ));
        }
    }

    // Crear la solicitud de reembolso
    let result: Result<(i32,), _> = sqlx::query_as(
        r#"
        INSERT INTO reembolso (
            id_pago,
            id_venta,
            tipo_reembolso,
            monto_reembolsado,
            motivo,
            estado,
            id_usuario_solicitante,
            fecha_solicitado
        ) VALUES ($1, $2, $3, $4, $5, 'solicitado', $6, CURRENT_TIMESTAMP)
        RETURNING id_reembolso
        "#,
    )
    .bind(id_pago)
    .bind(payload.id_venta)
    .bind(&payload.tipo_reembolso)
    .bind(payload.monto_reembolsado)
    .bind(&payload.motivo)
    .bind(id_usuario_autenticado)
    .fetch_one(&pool)
    .await;

    match result {
        Ok((id_reembolso,)) => Ok(Json(SolicitarReembolsoResponse {
            success: true,
            message: "Solicitud de reembolso creada exitosamente".to_string(),
            id_reembolso: Some(id_reembolso),
        })),
        Err(e) => {
            eprintln!("Error creating reembolso: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(SolicitarReembolsoResponse {
                    success: false,
                    message: "Error al crear la solicitud de reembolso".to_string(),
                    id_reembolso: None,
                }),
            ))
        }
    }
}

// ==================== CLIENTE: LISTAR MIS REEMBOLSOS ====================

pub async fn get_mis_reembolsos(
    State(pool): State<PgPool>,
    headers: HeaderMap,
) -> Result<Json<Vec<ReembolsoWithDetails>>, (StatusCode, Json<ErrorResponse>)> {
    // Extraer id_usuario del JWT token
    let id_usuario = extract_user_id(&headers)?;

    let reembolsos = sqlx::query_as::<_, ReembolsoWithDetails>(
        r#"
        SELECT
            r.id_reembolso,
            v.numero_pedido,
            r.id_venta,
            u.nombre as nombre_cliente,
            u.email as email_cliente,
            r.tipo_reembolso,
            r.monto_reembolsado,
            r.estado::text,
            r.fecha_solicitado
        FROM reembolso r
        JOIN venta v ON r.id_venta = v.id_venta
        JOIN usuario u ON v.id_usuario = u.id_usuario
        WHERE v.id_usuario = $1
        ORDER BY r.fecha_solicitado DESC
        "#,
    )
    .bind(id_usuario)
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        eprintln!("Error fetching reembolsos: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                success: false,
                message: "Error al obtener reembolsos".to_string(),
            }),
        )
    })?;

    Ok(Json(reembolsos))
}
