use axum::{routing::{get, put}, Router};
use sqlx::PgPool;

use crate::handlers::venta;

pub fn venta_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/ventas", get(venta::get_ventas))
        .route("/ventas/{id}", get(venta::get_venta_by_id))
        .route("/ventas/{id}/estado", put(venta::update_venta_estado))
        .route("/ventas/{id}/tracking", put(venta::update_venta_tracking))
        .route("/ventas/{id}/notas", get(venta::get_notas_admin))
        .route("/ventas/{id}/notas", put(venta::update_notas_admin))
        .route("/reportes/ventas", get(venta::get_reporte_ventas))
        .with_state(pool)
}
