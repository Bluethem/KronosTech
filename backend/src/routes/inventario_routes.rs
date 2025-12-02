use axum::{routing::{get, post, put, delete}, Router};
use sqlx::PgPool;

use crate::handlers::inventario_handler;

pub fn inventario_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/inventario", get(inventario_handler::get_inventario))
        .route("/inventario/stats", get(inventario_handler::get_inventario_stats))
        .route("/inventario/search", get(inventario_handler::search_products))
        .route("/inventario/entrada", post(inventario_handler::add_stock_entry))
        .route("/inventario/{id}/historial", get(inventario_handler::get_historial_inventario))
        .route("/inventario/{id}", put(inventario_handler::update_inventario))
        .route("/inventario/{id}", delete(inventario_handler::delete_inventario))
        .route("/inventario/reportes/general", get(inventario_handler::get_reporte_general))
        .route("/inventario/reportes/valorizacion", get(inventario_handler::get_reporte_valorizacion))
        .with_state(pool)
}
