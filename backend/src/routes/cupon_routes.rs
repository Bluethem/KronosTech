use axum::{routing::{get, post, delete, put}, Router};
use sqlx::PgPool;

use crate::handlers::cupon_handler;

pub fn cupon_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/cupones", get(cupon_handler::get_cupones).post(cupon_handler::create_cupon))
        .route("/cupones/stats", get(cupon_handler::get_cupon_stats))
        .route("/cupones/{id}", get(cupon_handler::get_cupon_detalle).put(cupon_handler::update_cupon).delete(cupon_handler::delete_cupon))
        .route("/cupones/{id}/usuarios", get(cupon_handler::get_assigned_users))
        .route("/cupones/{id_cupon}/usuarios/{id_usuario}", delete(cupon_handler::unassign_cupon_from_user))
        .route("/cupones/assign", post(cupon_handler::assign_cupon_to_users))
        .route("/usuarios", get(cupon_handler::get_usuarios))
        .with_state(pool)
}
