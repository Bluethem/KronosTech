use axum::{routing::get, Router};
use sqlx::PgPool;

use crate::handlers::inventario_handler;

pub fn inventario_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/inventario", get(inventario_handler::get_inventario))
        .route("/inventario/stats", get(inventario_handler::get_inventario_stats))
        .with_state(pool)
}
