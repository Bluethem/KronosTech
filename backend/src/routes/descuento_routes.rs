use axum::{routing::get, Router};
use sqlx::PgPool;

use crate::handlers::descuento_handler;

pub fn descuento_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/descuentos", get(descuento_handler::get_descuentos))
        .route("/descuentos/stats", get(descuento_handler::get_descuento_stats))
        .route("/descuentos/{id}", get(descuento_handler::get_descuento_detalle))
        .with_state(pool)
}
