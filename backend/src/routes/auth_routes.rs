use axum::{
    routing::{get, post, put},
    Router,
};
use sqlx::PgPool;

use crate::handlers::auth_handler::{
    register_handler,
    login_handler,
    get_current_user_handler,
    logout_handler,
    actualizar_perfil_handler,
    cambiar_password_handler,
};

pub fn auth_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .route("/me", get(get_current_user_handler))
        .route("/logout", post(logout_handler))
        .route("/perfil", put(actualizar_perfil_handler))
        .route("/cambiar-password", put(cambiar_password_handler))
        .with_state(pool)
}
