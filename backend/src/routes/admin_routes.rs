use axum::{
    routing::{get, post, put},
    Router,
};
use sqlx::PgPool;

use crate::handlers::admin_handler::{
    listar_usuarios_handler,
    actualizar_usuario_admin_handler,
    crear_administrador_handler,
};
use crate::handlers::dashboard_handler::get_dashboard_stats;

pub fn admin_routes(pool: PgPool) -> Router {
    Router::new()
        // Dashboard
        .route("/dashboard/stats", get(get_dashboard_stats))
        // Usuarios
        .route("/usuarios", get(listar_usuarios_handler))
        .route("/usuarios/{id}", put(actualizar_usuario_admin_handler))
        .route("/administradores", post(crear_administrador_handler))
        .with_state(pool)
}
