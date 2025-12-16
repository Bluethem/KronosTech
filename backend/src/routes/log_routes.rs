use axum::{
    routing::{get, delete},
    Router,
};
use sqlx::PgPool;

use crate::handlers::log_handler::{
    listar_logs_handler,
    crear_log_handler,
    limpiar_logs_handler,
};

pub fn log_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(listar_logs_handler).post(crear_log_handler))
        .route("/limpiar", delete(limpiar_logs_handler))
        .with_state(pool)
}
