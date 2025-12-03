use axum::Router;
use sqlx::PgPool;

use crate::handlers::reembolso_handler::{
    get_reembolsos, get_reembolso_stats, get_reembolso_by_id, procesar_reembolso,
};

pub fn reembolso_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/reembolsos", axum::routing::get(get_reembolsos))
        .route("/reembolsos/stats", axum::routing::get(get_reembolso_stats))
        .route("/reembolsos/{id}", axum::routing::get(get_reembolso_by_id))
        .route("/reembolsos/{id}/procesar", axum::routing::put(procesar_reembolso))
        .with_state(pool)
}
