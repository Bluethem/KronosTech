use axum::{routing::get, Router};
use sqlx::PgPool;

use crate::handlers::venta;

pub fn venta_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/ventas", get(venta::get_ventas))
        .route("/ventas/{id}", get(venta::get_venta_by_id))
        .route("/reportes/ventas", get(venta::get_reporte_ventas))
        .with_state(pool)
}
