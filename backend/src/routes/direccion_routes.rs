use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;

use crate::handlers::{
    get_direcciones_handler,
    crear_direccion_handler,
    actualizar_direccion_handler,
    eliminar_direccion_handler,
};

pub fn direccion_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/direcciones", get(get_direcciones_handler))
        .route("/direcciones", post(crear_direccion_handler))
        .route("/direcciones/{id}", put(actualizar_direccion_handler))
        .route("/direcciones/{id}", delete(eliminar_direccion_handler))
        .with_state(pool)
}
