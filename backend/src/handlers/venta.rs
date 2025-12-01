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

#[derive(Debug, Deserialize)]
pub struct ReporteQuery {
    pub fecha_inicio: Option<String>,
    pub fecha_fin: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct VentaPorDia {
    pub fecha: NaiveDateTime,
    pub total_ventas: Decimal,
    pub cantidad_pedidos: i64,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct VentaPorEstado {
    pub estado: String,
    pub cantidad: i64,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct TopProducto {
    pub nombre_producto: String,
    pub total_vendido: i64,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct VentaProductoTabla {
    pub nombre_producto: String,
    pub sku: String,
    pub categoria: String,
    pub unidades_vendidas: i64,
    pub monto_total: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReporteStats {
    pub total_pedidos: i64,
    pub ingresos_totales: Decimal,
    pub ticket_promedio: Decimal,
    pub tasa_conversion: f64, // Placeholder, hard to calculate without visit data
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReporteResponse {
    pub stats: ReporteStats,
    pub ventas_por_dia: Vec<VentaPorDia>,
    pub ventas_por_estado: Vec<VentaPorEstado>,
    pub top_productos: Vec<TopProducto>,
    pub tabla_productos: Vec<VentaProductoTabla>,
}

pub async fn get_reporte_ventas(
    State(pool): State<PgPool>,
    Query(query): Query<ReporteQuery>,
) -> Result<Json<ReporteResponse>, StatusCode> {
    // Parse dates from strings
    let fecha_fin = if let Some(fecha_str) = query.fecha_fin {
        chrono::DateTime::parse_from_rfc3339(&fecha_str)
            .map(|dt| dt.naive_local())
            .unwrap_or_else(|_| chrono::Local::now().naive_local())
    } else {
        chrono::Local::now().naive_local()
    };

    let fecha_inicio = if let Some(fecha_str) = query.fecha_inicio {
        chrono::DateTime::parse_from_rfc3339(&fecha_str)
            .map(|dt| dt.naive_local())
            .unwrap_or_else(|_| fecha_fin - chrono::Duration::days(30))
    } else {
        fecha_fin - chrono::Duration::days(30)
    };

    // 1. Stats Generales
    let stats_query = r#"
        SELECT 
            COUNT(*) as total_pedidos,
            COALESCE(SUM(total), 0) as ingresos_totales,
            COALESCE(AVG(total), 0) as ticket_promedio
        FROM venta
        WHERE fecha_pedido BETWEEN $1 AND $2
    "#;

    let (total_pedidos, ingresos_totales, ticket_promedio): (i64, Decimal, Decimal) = sqlx::query_as(stats_query)
        .bind(fecha_inicio)
        .bind(fecha_fin)
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            eprintln!("Error fetching stats: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // 2. Ventas por Día
    let ventas_dia_query = r#"
        SELECT 
            DATE_TRUNC('day', fecha_pedido) as fecha,
            COALESCE(SUM(total), 0) as total_ventas,
            COUNT(*) as cantidad_pedidos
        FROM venta
        WHERE fecha_pedido BETWEEN $1 AND $2
        GROUP BY DATE_TRUNC('day', fecha_pedido)
        ORDER BY fecha ASC
    "#;

    let ventas_por_dia = sqlx::query_as::<_, VentaPorDia>(ventas_dia_query)
        .bind(fecha_inicio)
        .bind(fecha_fin)
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            eprintln!("Error fetching ventas por dia: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // 3. Ventas por Estado
    let ventas_estado_query = r#"
        SELECT 
            estado::text,
            COUNT(*) as cantidad
        FROM venta
        WHERE fecha_pedido BETWEEN $1 AND $2
        GROUP BY estado
    "#;

    let ventas_por_estado = sqlx::query_as::<_, VentaPorEstado>(ventas_estado_query)
        .bind(fecha_inicio)
        .bind(fecha_fin)
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            eprintln!("Error fetching ventas por estado: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // 4. Top Productos
    let top_productos_query = r#"
        SELECT 
            pd.nombre as nombre_producto,
            SUM(dv.cantidad) as total_vendido
        FROM detalle_venta dv
        JOIN venta v ON dv.id_venta = v.id_venta
        JOIN producto_detalle pd ON dv.id_producto_detalle = pd.id_producto_detalle
        WHERE v.fecha_pedido BETWEEN $1 AND $2
        GROUP BY pd.nombre
        ORDER BY total_vendido DESC
        LIMIT 10
    "#;

    let top_productos = sqlx::query_as::<_, TopProducto>(top_productos_query)
        .bind(fecha_inicio)
        .bind(fecha_fin)
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            eprintln!("Error fetching top productos: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // 5. Tabla Detallada por Producto
    let tabla_productos_query = r#"
        SELECT 
            pd.nombre as nombre_producto,
            pd.sku,
            c.nombre as categoria,
            SUM(dv.cantidad) as unidades_vendidas,
            SUM(dv.subtotal) as monto_total
        FROM detalle_venta dv
        JOIN venta v ON dv.id_venta = v.id_venta
        JOIN producto_detalle pd ON dv.id_producto_detalle = pd.id_producto_detalle
        JOIN producto p ON pd.id_producto = p.id_producto
        JOIN categoria c ON p.id_categoria = c.id_categoria
        WHERE v.fecha_pedido BETWEEN $1 AND $2
        GROUP BY pd.nombre, pd.sku, c.nombre
        ORDER BY unidades_vendidas DESC
        LIMIT 50
    "#;

    let tabla_productos = sqlx::query_as::<_, VentaProductoTabla>(tabla_productos_query)
        .bind(fecha_inicio)
        .bind(fecha_fin)
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            eprintln!("Error fetching tabla productos: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ReporteResponse {
        stats: ReporteStats {
            total_pedidos,
            ingresos_totales,
            ticket_promedio,
            tasa_conversion: 0.0, // Not implemented yet
        },
        ventas_por_dia,
        ventas_por_estado,
        top_productos,
        tabla_productos,
    }))
}
