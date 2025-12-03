use axum::{routing::{get, post, delete, put}, Router};
use sqlx::PgPool;

use crate::handlers::descuento_handler;

pub fn descuento_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/descuentos", get(descuento_handler::get_descuentos).post(descuento_handler::create_descuento))
        .route("/descuentos/stats", get(descuento_handler::get_descuento_stats))
        .route("/descuentos/{id}", get(descuento_handler::get_descuento_detalle).put(descuento_handler::update_descuento).delete(descuento_handler::delete_descuento))
        .route("/productos/dropdown", get(descuento_handler::get_productos_dropdown))
        .with_state(pool)
}
