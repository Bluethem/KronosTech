use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

use crate::handlers::{
    register_handler,
    login_handler,
    get_current_user_handler,
    logout_handler,
};

pub fn auth_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .route("/me", get(get_current_user_handler))
        .route("/logout", post(logout_handler))
        .with_state(pool)
}
