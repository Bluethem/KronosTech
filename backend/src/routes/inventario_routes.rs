use axum::{routing::{get, post}, Router};
use sqlx::PgPool;

use crate::handlers::inventario_handler;

pub fn inventario_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/inventario", get(inventario_handler::get_inventario))
        .route("/inventario/stats", get(inventario_handler::get_inventario_stats))
        .route("/inventario/search", get(inventario_handler::search_products))
        .route("/inventario/entrada", post(inventario_handler::add_stock_entry))
        .with_state(pool)
}
