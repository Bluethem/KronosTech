use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde::Serialize;
use sqlx::PgPool;
use chrono::{Local, NaiveDateTime, Datelike};

#[derive(Debug, Serialize)]
pub struct DashboardStats {
    pub ventas_hoy: i64,
    pub pedidos_pendientes: i64,
    pub productos_bajo_stock: i64,
    pub usuarios_activos: i64,
}

/// GET /api/admin/dashboard/stats
/// Obtiene estadísticas generales para el panel de administración
pub async fn get_dashboard_stats(
    State(pool): State<PgPool>,
) -> Result<Json<DashboardStats>, StatusCode> {
    // Obtener fecha de hoy (inicio y fin del día)
    let now = Local::now().naive_local();
    let hoy_inicio = chrono::NaiveDate::from_ymd_opt(now.year(), now.month(), now.day())
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    let hoy_fin = chrono::NaiveDate::from_ymd_opt(now.year(), now.month(), now.day())
        .unwrap()
        .and_hms_opt(23, 59, 59)
        .unwrap();

    // 1. Contar ventas de hoy (pedidos creados hoy con pago completado)
    let ventas_hoy: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) 
        FROM venta 
        WHERE fecha_pedido BETWEEN $1 AND $2
        AND estado_pago = 'completado'
        "#,
    )
    .bind(hoy_inicio)
    .bind(hoy_fin)
    .fetch_one(&pool)
    .await
    .unwrap_or((0,));

    // 2. Contar pedidos pendientes (estado pendiente o confirmado que necesitan atención)
    let pedidos_pendientes: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) 
        FROM venta 
        WHERE estado IN ('pendiente', 'confirmado', 'procesando')
        AND estado != 'cancelado'
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap_or((0,));

    // 3. Contar productos con bajo stock (cantidad_disponible <= stock_minimo o <= 10 si no tiene mínimo)
    let productos_bajo_stock: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(DISTINCT pd.id_producto_detalle)
        FROM producto_detalle pd
        LEFT JOIN inventario i ON pd.id_producto_detalle = i.id_producto_detalle
        WHERE pd.activo = TRUE
        AND (
            i.cantidad_disponible <= COALESCE(i.stock_minimo, 10)
            OR i.cantidad_disponible IS NULL
        )
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap_or((0,));

    // 4. Contar usuarios activos (todos los usuarios con activo = true)
    let usuarios_activos: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) 
        FROM usuario 
        WHERE activo = TRUE
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap_or((0,));

    Ok(Json(DashboardStats {
        ventas_hoy: ventas_hoy.0,
        pedidos_pendientes: pedidos_pendientes.0,
        productos_bajo_stock: productos_bajo_stock.0,
        usuarios_activos: usuarios_activos.0,
    }))
}

