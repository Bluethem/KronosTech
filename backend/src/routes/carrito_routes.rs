use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use sqlx::PgPool;

use crate::handlers::{
    agregar_item_handler, actualizar_cantidad_handler, eliminar_item_handler,
    get_carrito_handler, limpiar_carrito_handler,
};

pub fn carrito_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/carrito", get(get_carrito_handler))
        .route("/carrito", delete(limpiar_carrito_handler))
        .route("/carrito/items", post(agregar_item_handler))
        .route("/carrito/items/{id}", patch(actualizar_cantidad_handler))
        .route("/carrito/items/{id}", delete(eliminar_item_handler))
        .with_state(pool)
}
