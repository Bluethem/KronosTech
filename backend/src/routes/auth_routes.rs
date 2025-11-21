use axum::{routing::post, Router};

use crate::{
    config::AppState,
    handlers::auth_handler::{login, register},
};

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
}
