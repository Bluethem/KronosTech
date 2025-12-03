use axum::{
    extract::{Query, State, Path},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use chrono::{NaiveDateTime, Datelike};
use rust_decimal::Decimal;

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
