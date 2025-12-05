use axum::Router;
use sqlx::PgPool;

use crate::handlers::reembolso_handler::{
    get_reembolsos, get_reembolso_stats, get_reembolso_by_id, procesar_reembolso,
    solicitar_reembolso, get_mis_reembolsos,
};

pub fn reembolso_routes(pool: PgPool) -> Router {
    Router::new()
        // Admin endpoints
        .route("/reembolsos", axum::routing::get(get_reembolsos))
        .route("/reembolsos/stats", axum::routing::get(get_reembolso_stats))
        .route("/reembolsos/{id}", axum::routing::get(get_reembolso_by_id))
        .route("/reembolsos/{id}/procesar", axum::routing::put(procesar_reembolso))
        // Cliente endpoints
        .route("/mis-reembolsos", axum::routing::get(get_mis_reembolsos))
        .route("/mis-reembolsos/solicitar", axum::routing::post(solicitar_reembolso))
        .with_state(pool)
}
