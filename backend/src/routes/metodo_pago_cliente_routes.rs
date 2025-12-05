use axum::{
    routing::{get, post, put, delete},
    Router,
};
use sqlx::PgPool;

use crate::handlers::metodo_pago_cliente_handler::{
    get_metodos_pago_handler,
    crear_metodo_pago_handler,
    actualizar_metodo_pago_handler,
    eliminar_metodo_pago_handler,
};

pub fn metodo_pago_cliente_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(get_metodos_pago_handler))
        .route("/", post(crear_metodo_pago_handler))
        .route("/{id}", put(actualizar_metodo_pago_handler))
        .route("/{id}", delete(eliminar_metodo_pago_handler))
        .with_state(pool)
}
