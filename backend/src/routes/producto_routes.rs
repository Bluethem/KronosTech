use axum::{routing::{get, post}, Router};
use sqlx::PgPool;

use crate::handlers::producto_handler;

pub fn producto_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/productos", post(producto_handler::create_producto))
        .route("/productos/check-sku", post(producto_handler::check_sku_availability))
        .with_state(pool)
}
