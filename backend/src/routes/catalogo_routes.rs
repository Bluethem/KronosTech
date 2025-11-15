use axum::{
    routing::get,
    Router,
};
use sqlx::PgPool;

use crate::handlers::catalogo_handler::*;

/// Rutas públicas del catálogo (sin autenticación)
pub fn catalogo_routes(pool: PgPool) -> Router {
    Router::new()
        // Familias, Categorías, Subcategorías, Marcas
        .route("/familias", get(get_familias))
        .route("/categorias", get(get_categorias))
        .route("/subcategorias", get(get_subcategorias))
        .route("/marcas", get(get_marcas))
        
        // Productos
        .route("/productos", get(get_productos))
        .route("/productos/slug/{slug}", get(get_producto_by_slug))
        .route("/productos/{id}", get(get_producto_by_id))
        .route("/productos/{id}/valoraciones", get(get_valoraciones))
        
        .with_state(pool)
}
