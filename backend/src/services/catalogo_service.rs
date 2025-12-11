use crate::repositories::{CatalogoRepository, ProductoFilters};
use crate::models::{familia, categoria, marca, producto_detalle, Subcategoria, Valoracion, valoracion::CrearValoracionRequest};
use sqlx::PgPool;

pub struct CatalogoService;

impl CatalogoService {
    pub async fn get_familias(pool: &PgPool) -> Result<Vec<familia::FamiliaResponse>, sqlx::Error> {
        CatalogoRepository::get_familias(pool).await
    }

    pub async fn get_categorias(
        pool: &PgPool,
        id_familia: Option<i32>,
    ) -> Result<Vec<categoria::CategoriaResponse>, sqlx::Error> {
        CatalogoRepository::get_categorias(pool, id_familia).await
    }

    pub async fn get_subcategorias(
        pool: &PgPool,
        id_categoria: Option<i32>,
    ) -> Result<Vec<Subcategoria>, sqlx::Error> {
        CatalogoRepository::get_subcategorias(pool, id_categoria).await
    }

    pub async fn get_marcas(pool: &PgPool) -> Result<Vec<marca::MarcaResponse>, sqlx::Error> {
        CatalogoRepository::get_marcas(pool).await
    }

    pub async fn get_productos(
        pool: &PgPool,
        filters: ProductoFilters,
    ) -> Result<(Vec<producto_detalle::ProductoListItem>, i64), sqlx::Error> {
        let productos = CatalogoRepository::get_productos_list(pool, &filters).await?;
        let total = CatalogoRepository::count_productos(pool, &filters).await?;
        Ok((productos, total))
    }

    pub async fn get_producto_by_id(
        pool: &PgPool,
        id: i32,
    ) -> Result<Option<producto_detalle::ProductoDetalleResponse>, sqlx::Error> {
        // Incrementar vistas
        let _ = CatalogoRepository::increment_views(pool, id).await;
        CatalogoRepository::get_producto_by_id(pool, id).await
    }

    pub async fn get_producto_by_slug(
        pool: &PgPool,
        slug: &str,
    ) -> Result<Option<producto_detalle::ProductoDetalleResponse>, sqlx::Error> {
        let producto = CatalogoRepository::get_producto_by_slug(pool, slug).await?;
        
        // Incrementar vistas si se encontró el producto
        if let Some(ref prod) = producto {
            let _ = CatalogoRepository::increment_views(pool, prod.id_producto_detalle).await;
        }
        
        Ok(producto)
    }

    pub async fn get_valoraciones(
        pool: &PgPool,
        id_producto_detalle: i32,
    ) -> Result<Vec<Valoracion>, sqlx::Error> {
        CatalogoRepository::get_valoraciones(pool, id_producto_detalle).await
    }

    pub async fn crear_valoracion(
        pool: &PgPool,
        id_producto_detalle: i32,
        id_usuario: i32,
        payload: CrearValoracionRequest,
    ) -> Result<Valoracion, String> {
        if payload.calificacion < 1 || payload.calificacion > 5 {
            return Err("La calificación debe estar entre 1 y 5".to_string());
        }

        crate::repositories::catalogo_repository::crear_valoracion(
            pool,
            id_producto_detalle,
            id_usuario,
            payload.calificacion,
            payload.titulo,
            payload.comentario,
        )
        .await
        .map_err(|e| format!("Error al crear valoración: {}", e))
    }
}
