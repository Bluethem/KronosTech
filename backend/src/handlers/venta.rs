use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use chrono::NaiveDateTime;
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct VentaWithUser {
    pub id_venta: i32,
    pub numero_pedido: String,
    pub id_usuario: i32,
    pub nombre_usuario: Option<String>,
    pub email_usuario: Option<String>,
    pub subtotal: Decimal,
    pub descuento_total: Option<Decimal>,
    pub costo_envio: Option<Decimal>,
    pub total: Decimal,
    pub moneda: Option<String>,
    pub estado: Option<String>,
    pub estado_pago: Option<String>,
    pub fecha_pedido: Option<NaiveDateTime>,
    pub fecha_entrega_estimada: Option<chrono::NaiveDate>,
}

#[derive(Debug, Deserialize)]
pub struct VentaQuery {
    pub estado: Option<String>,
    pub estado_pago: Option<String>,
    pub search: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

pub async fn get_ventas(
    State(pool): State<PgPool>,
    Query(query): Query<VentaQuery>,
) -> Result<Json<Vec<VentaWithUser>>, StatusCode> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(50);
    let offset = (page - 1) * limit;

    let mut sql = String::from(
        r#"
        SELECT 
            v.id_venta,
            v.numero_pedido,
            v.id_usuario,
            u.nombre as nombre_usuario,
            u.email as email_usuario,
            v.subtotal,
            v.descuento_total,
            v.costo_envio,
            v.total,
            v.moneda,
            v.estado::text,
            v.estado_pago::text,
            v.fecha_pedido,
            v.fecha_entrega_estimada
        FROM venta v
        LEFT JOIN usuario u ON v.id_usuario = u.id_usuario
        WHERE 1=1
        "#,
    );

    // Add filters
    if let Some(estado) = &query.estado {
        if estado != "Todos" {
            sql.push_str(&format!(" AND v.estado = '{}'", estado));
        }
    }

    if let Some(estado_pago) = &query.estado_pago {
        if estado_pago != "Todos" {
            sql.push_str(&format!(" AND v.estado_pago = '{}'", estado_pago));
        }
    }

    if let Some(search) = &query.search {
        if !search.is_empty() {
            sql.push_str(&format!(
                " AND (v.numero_pedido ILIKE '%{}%' OR u.nombre ILIKE '%{}%' OR u.email ILIKE '%{}%')",
                search, search, search
            ));
        }
    }

    sql.push_str(" ORDER BY v.fecha_pedido DESC NULLS LAST");
    sql.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));

    match sqlx::query_as::<_, VentaWithUser>(&sql)
        .fetch_all(&pool)
        .await
    {
        Ok(ventas) => Ok(Json(ventas)),
        Err(e) => {
            eprintln!("Error fetching ventas: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct VentaDetalle {
    pub id_venta: i32,
    pub numero_pedido: String,
    pub id_usuario: i32,
    pub nombre_usuario: Option<String>,
    pub email_usuario: Option<String>,
    pub telefono_usuario: Option<String>,
    pub dni_usuario: Option<String>,
    pub subtotal: Decimal,
    pub descuento_total: Option<Decimal>,
    pub costo_envio: Option<Decimal>,
    pub total: Decimal,
    pub moneda: Option<String>,
    pub estado: Option<String>,
    pub estado_pago: Option<String>,
    pub direccion_envio: Option<String>,
    pub ciudad: Option<String>,
    pub departamento: Option<String>,
    pub codigo_postal: Option<String>,
    pub telefono_contacto: Option<String>,
    pub metodo_envio: Option<String>,
    pub numero_tracking: Option<String>,
    pub fecha_pedido: Option<NaiveDateTime>,
    pub fecha_entrega_estimada: Option<chrono::NaiveDate>,
    pub fecha_actualizacion: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ProductoVenta {
    pub id_detalle_venta: i32,
    pub id_producto: i32,
    pub nombre_producto: String,
    pub sku: String,
    pub cantidad: i32,
    pub precio_unitario: Decimal,
    pub subtotal: Decimal,
    pub imagen_principal: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VentaCompleta {
    #[serde(flatten)]
    pub venta: VentaDetalle,
    pub productos: Vec<ProductoVenta>,
}

pub async fn get_venta_by_id(
    State(pool): State<PgPool>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> Result<Json<VentaCompleta>, StatusCode> {
    let venta = sqlx::query_as::<_, VentaDetalle>(
        r#"
        SELECT 
            v.id_venta,
            v.numero_pedido,
            v.id_usuario,
            u.nombre as nombre_usuario,
            u.email as email_usuario,
            u.telefono as telefono_usuario,
            u.dni as dni_usuario,
            v.subtotal,
            v.descuento_total,
            v.costo_envio,
            v.total,
            v.moneda,
            v.estado::text,
            v.estado_pago::text,
            v.direccion_envio,
            v.ciudad,
            v.departamento,
            v.codigo_postal,
            v.telefono_contacto,
            v.metodo_envio,
            v.numero_tracking,
            v.fecha_pedido,
            v.fecha_entrega_estimada,
            v.fecha_actualizacion
        FROM venta v
        LEFT JOIN usuario u ON v.id_usuario = u.id_usuario
        WHERE v.id_venta = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        eprintln!("Error fetching venta: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .ok_or(StatusCode::NOT_FOUND)?;

    let productos = sqlx::query_as::<_, ProductoVenta>(
        r#"
        SELECT 
            dv.id_detalle_venta,
            dv.id_producto,
            pd.nombre as nombre_producto,
            pd.sku,
            dv.cantidad,
            dv.precio_unitario,
            dv.subtotal,
            pd.imagen_principal
        FROM detalle_venta dv
        JOIN producto_detalle pd ON dv.id_producto_detalle = pd.id_producto_detalle
        WHERE dv.id_venta = $1
        "#,
    )
    .bind(id)
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        eprintln!("Error fetching productos venta: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(VentaCompleta { venta, productos }))
}
