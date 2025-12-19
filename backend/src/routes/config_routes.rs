use axum::{
    routing::get,
    Router,
};
use sqlx::PgPool;

use crate::handlers::config_handler::{
    get_all_config_handler,
    get_config_handler,
    update_config_handler,
    update_config_batch_handler,
    get_session_timeout_handler,
};

pub fn config_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(get_all_config_handler).put(update_config_batch_handler))
        .route("/session-timeout", get(get_session_timeout_handler))
        .route("/{clave}", get(get_config_handler).put(update_config_handler))
        .with_state(pool)
}

