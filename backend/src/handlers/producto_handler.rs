use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use rust_decimal::Decimal;

#[derive(Debug, Deserialize)]
pub struct CreateProductoRequest {
    pub nombre: String,
    pub descripcion: Option<String>,
    pub modelo: Option<String>,
    pub sku: String,
    pub codigo_barras: Option<String>,
    pub id_categoria: i32,
    pub id_marca: i32,
    pub precio_base: Decimal,
    pub precio_venta: Decimal,
    pub costo: Option<Decimal>,
    pub cantidad_inicial: Option<i32>,
    pub cantidad_minima: Option<i32>,
    pub ubicacion_fisica: Option<String>,
    pub peso: Option<Decimal>,
    pub dimensiones: Option<String>,
    pub garantia_meses: Option<i32>,
    pub activo: Option<bool>,
    pub es_destacado: Option<bool>,
    pub es_nuevo: Option<bool>,
    pub es_oferta: Option<bool>,
    pub imagen_principal: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateProductoResponse {
    pub id_producto: i32,
    pub id_producto_detalle: i32,
    pub id_inventario: Option<i32>,
}

pub async fn create_producto(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateProductoRequest>,
) -> Result<Json<CreateProductoResponse>, StatusCode> {
    let mut tx = pool.begin().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 1. Crear producto base
    let producto_result = sqlx::query!(
        r#"
        INSERT INTO producto (id_categoria, nombre)
        VALUES ($1, $2)
        RETURNING id_producto
        "#,
        payload.id_categoria,
        payload.nombre
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| {
        eprintln!("Error creating producto: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let id_producto = producto_result.id_producto;

    // 2. Crear producto_detalle
    let producto_detalle_result = sqlx::query!(
        r#"
        INSERT INTO producto_detalle (
            id_producto, id_marca, nombre, descripcion, modelo, sku, codigo_barras,
            precio_base, precio_venta, costo, peso, dimensiones, garantia_meses,
            es_destacado, es_nuevo, es_oferta, imagen_principal
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
        RETURNING id_producto_detalle
        "#,
        id_producto,
        payload.id_marca,
        payload.nombre,
        payload.descripcion,
        payload.modelo,
        payload.sku,
        payload.codigo_barras,
        payload.precio_base,
        payload.precio_venta,
        payload.costo,
        payload.peso,
        payload.dimensiones,
        payload.garantia_meses,
        payload.es_destacado.unwrap_or(false),
        payload.es_nuevo.unwrap_or(false),
        payload.es_oferta.unwrap_or(false),
        payload.imagen_principal
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| {
        eprintln!("Error creating producto_detalle: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let id_producto_detalle = producto_detalle_result.id_producto_detalle;

    // 3. Crear inventario inicial si se especificó cantidad
    let id_inventario = if let Some(cantidad) = payload.cantidad_inicial {
        let inventario_result = sqlx::query!(
            r#"
            INSERT INTO inventario (
                id_producto_detalle, cantidad_disponible, cantidad_minima, ubicacion_fisica
            )
            VALUES ($1, $2, $3, $4)
            RETURNING id_inventario
            "#,
            id_producto_detalle,
            cantidad,
            payload.cantidad_minima.unwrap_or(5),
            payload.ubicacion_fisica
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| {
            eprintln!("Error creating inventario: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        Some(inventario_result.id_inventario)
    } else {
        None
    };

    // Commit transaction
    tx.commit().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(CreateProductoResponse {
        id_producto,
        id_producto_detalle,
        id_inventario,
    }))
}

pub async fn check_sku_availability(
    State(pool): State<PgPool>,
    Json(sku): Json<String>,
) -> Result<Json<bool>, StatusCode> {
    let result = sqlx::query!(
        "SELECT COUNT(*) as count FROM producto_detalle WHERE sku = $1",
        sku
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(result.count.unwrap_or(0) == 0))
}
