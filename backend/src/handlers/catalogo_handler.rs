use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::repositories::ProductoFilters;
use crate::services::CatalogoService;

// ==================== RESPONSES ====================
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub success: bool,
    pub data: Vec<T>,
    pub pagination: Pagination,
}

#[derive(Debug, Serialize)]
pub struct Pagination {
    pub total: i64,
    pub limit: i32,
    pub offset: i32,
    pub total_pages: i64,
    pub current_page: i64,
}

// ==================== QUERY PARAMS ====================
#[derive(Debug, Deserialize)]
pub struct ProductoQuery {
    pub search: Option<String>,
    pub categoria: Option<i32>,
    pub subcategoria: Option<i32>,
    pub marca: Option<i32>,
    pub familia: Option<i32>,
    pub precio_min: Option<f64>,
    pub precio_max: Option<f64>,
    pub destacados: Option<bool>,
    pub nuevos: Option<bool>,
    pub ofertas: Option<bool>,
    pub en_stock: Option<bool>,
    pub order_by: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CategoriaQuery {
    pub familia: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct SubcategoriaQuery {
    pub categoria: Option<i32>,
}

// ==================== HANDLERS ====================

/// GET /api/familias - Obtener todas las familias
pub async fn get_familias(
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, StatusCode> {
    match CatalogoService::get_familias(&pool).await {
        Ok(familias) => Ok(Json(ApiResponse {
            success: true,
            data: Some(familias),
            message: None,
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// GET /api/categorias - Obtener categorías (con filtro opcional por familia)
pub async fn get_categorias(
    State(pool): State<PgPool>,
    Query(params): Query<CategoriaQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    match CatalogoService::get_categorias(&pool, params.familia).await {
        Ok(categorias) => Ok(Json(ApiResponse {
            success: true,
            data: Some(categorias),
            message: None,
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// GET /api/subcategorias - Obtener subcategorías (con filtro opcional por categoría)
pub async fn get_subcategorias(
    State(pool): State<PgPool>,
    Query(params): Query<SubcategoriaQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    match CatalogoService::get_subcategorias(&pool, params.categoria).await {
        Ok(subcategorias) => Ok(Json(ApiResponse {
            success: true,
            data: Some(subcategorias),
            message: None,
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// GET /api/marcas - Obtener todas las marcas
pub async fn get_marcas(
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, StatusCode> {
    match CatalogoService::get_marcas(&pool).await {
        Ok(marcas) => Ok(Json(ApiResponse {
            success: true,
            data: Some(marcas),
            message: None,
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// GET /api/productos - Obtener productos con filtros
pub async fn get_productos(
    State(pool): State<PgPool>,
    Query(params): Query<ProductoQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let filters = ProductoFilters {
        search: params.search,
        id_categoria: params.categoria,
        id_subcategoria: params.subcategoria,
        id_marca: params.marca,
        id_familia: params.familia,
        precio_min: params.precio_min,
        precio_max: params.precio_max,
        destacados: params.destacados,
        nuevos: params.nuevos,
        ofertas: params.ofertas,
        en_stock: params.en_stock,
        order_by: params.order_by,
        limit: params.limit,
        offset: params.offset,
    };

    let limit = filters.limit.unwrap_or(20);
    let offset = filters.offset.unwrap_or(0);

    match CatalogoService::get_productos(&pool, filters).await {
        Ok((productos, total)) => {
            let total_pages = (total as f64 / limit as f64).ceil() as i64;
            let current_page = (offset / limit) as i64 + 1;

            Ok(Json(PaginatedResponse {
                success: true,
                data: productos,
                pagination: Pagination {
                    total,
                    limit,
                    offset,
                    total_pages,
                    current_page,
                },
            }))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// GET /api/productos/:id - Obtener producto por ID
pub async fn get_producto_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, StatusCode> {
    match CatalogoService::get_producto_by_id(&pool, id).await {
        Ok(Some(producto)) => Ok(Json(ApiResponse {
            success: true,
            data: Some(producto),
            message: None,
        })),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// GET /api/productos/slug/:slug - Obtener producto por slug
pub async fn get_producto_by_slug(
    State(pool): State<PgPool>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    match CatalogoService::get_producto_by_slug(&pool, &slug).await {
        Ok(Some(producto)) => Ok(Json(ApiResponse {
            success: true,
            data: Some(producto),
            message: None,
        })),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// GET /api/productos/:id/valoraciones - Obtener valoraciones de un producto
pub async fn get_valoraciones(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, StatusCode> {
    match CatalogoService::get_valoraciones(&pool, id).await {
        Ok(valoraciones) => Ok(Json(ApiResponse {
            success: true,
            data: Some(valoraciones),
            message: None,
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
