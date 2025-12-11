use crate::models::*;
use sqlx::PgPool;
use chrono::{DateTime, Utc};

pub struct CatalogoRepository;

impl CatalogoRepository {
    // ==================== FAMILIAS ====================
    pub async fn get_familias(pool: &PgPool) -> Result<Vec<familia::FamiliaResponse>, sqlx::Error> {
        let familias = sqlx::query_as!(
            familia::FamiliaResponse,
            r#"
            SELECT 
                f.id_familia,
                f.nombre,
                f.descripcion,
                f.icono,
                f.slug,
                COUNT(DISTINCT pd.id_producto_detalle) as "total_productos?"
            FROM familia f
            LEFT JOIN categoria c ON f.id_familia = c.id_familia
            LEFT JOIN producto p ON c.id_categoria = p.id_categoria
            LEFT JOIN producto_detalle pd ON p.id_producto = pd.id_producto AND pd.estado = 'activo'
            WHERE f.estado = 'activo'
            GROUP BY f.id_familia
            ORDER BY f.orden
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(familias)
    }

    // ==================== CATEGORÍAS ====================
    pub async fn get_categorias(
        pool: &PgPool,
        id_familia: Option<i32>,
    ) -> Result<Vec<categoria::CategoriaResponse>, sqlx::Error> {
        let categorias = if let Some(familia_id) = id_familia {
            sqlx::query_as!(
                categoria::CategoriaResponse,
                r#"
                SELECT 
                    c.id_categoria,
                    c.nombre,
                    c.descripcion,
                    c.icono,
                    c.slug,
                    c.id_familia,
                    f.nombre as "familia_nombre?",
                    COUNT(DISTINCT pd.id_producto_detalle) as "total_productos?"
                FROM categoria c
                INNER JOIN familia f ON c.id_familia = f.id_familia
                LEFT JOIN producto p ON c.id_categoria = p.id_categoria
                LEFT JOIN producto_detalle pd ON p.id_producto = pd.id_producto AND pd.estado = 'activo'
                WHERE c.estado = 'activo' AND c.id_familia = $1
                GROUP BY c.id_categoria, f.nombre
                ORDER BY c.orden
                "#,
                familia_id
            )
            .fetch_all(pool)
            .await?
        } else {
            sqlx::query_as!(
                categoria::CategoriaResponse,
                r#"
                SELECT 
                    c.id_categoria,
                    c.nombre,
                    c.descripcion,
                    c.icono,
                    c.slug,
                    c.id_familia,
                    f.nombre as "familia_nombre?",
                    COUNT(DISTINCT pd.id_producto_detalle) as "total_productos?"
                FROM categoria c
                INNER JOIN familia f ON c.id_familia = f.id_familia
                LEFT JOIN producto p ON c.id_categoria = p.id_categoria
                LEFT JOIN producto_detalle pd ON p.id_producto = pd.id_producto AND pd.estado = 'activo'
                WHERE c.estado = 'activo'
                GROUP BY c.id_categoria, f.nombre
                ORDER BY c.orden
                "#
            )
            .fetch_all(pool)
            .await?
        };

        Ok(categorias)
    }

    // ==================== SUBCATEGORÍAS ====================
    pub async fn get_subcategorias(
        pool: &PgPool,
        id_categoria: Option<i32>,
    ) -> Result<Vec<Subcategoria>, sqlx::Error> {
        let subcategorias = if let Some(cat_id) = id_categoria {
            sqlx::query_as::<_, Subcategoria>(
                r#"
                SELECT 
                    id_subcategoria,
                    nombre,
                    descripcion,
                    slug,
                    id_categoria,
                    orden,
                    estado::text,
                    fecha_creacion,
                    fecha_actualizacion
                FROM subcategoria
                WHERE estado = 'activo' AND id_categoria = $1
                ORDER BY orden
                "#
            )
            .bind(cat_id)
            .fetch_all(pool)
            .await?
        } else {
            sqlx::query_as::<_, Subcategoria>(
                r#"
                SELECT 
                    id_subcategoria,
                    nombre,
                    descripcion,
                    slug,
                    id_categoria,
                    orden,
                    estado::text,
                    fecha_creacion,
                    fecha_actualizacion
                FROM subcategoria
                WHERE estado = 'activo'
                ORDER BY orden
                "#
            )
            .fetch_all(pool)
            .await?
        };

        Ok(subcategorias)
    }

    // ==================== MARCAS ====================
    pub async fn get_marcas(pool: &PgPool) -> Result<Vec<marca::MarcaResponse>, sqlx::Error> {
        let marcas = sqlx::query_as!(
            marca::MarcaResponse,
            r#"
            SELECT 
                m.id_marca,
                m.nombre,
                m.logo,
                m.slug,
                COUNT(DISTINCT pd.id_producto_detalle) as "total_productos?"
            FROM marca m
            LEFT JOIN producto_detalle pd ON m.id_marca = pd.id_marca AND pd.estado = 'activo'
            WHERE m.estado = 'activo'
            GROUP BY m.id_marca
            ORDER BY m.nombre
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(marcas)
    }

    // ==================== PRODUCTOS DETALLE (LISTADO) ====================
    pub async fn get_productos_list(
        pool: &PgPool,
        filters: &ProductoFilters,
    ) -> Result<Vec<producto_detalle::ProductoListItem>, sqlx::Error> {
        let limit = filters.limit.unwrap_or(20).min(100) as i64;
        let offset = filters.offset.unwrap_or(0) as i64;

        let mut query = String::from(
            r#"
            SELECT 
                pd.id_producto_detalle,
                pd.nombre,
                pd.sku,
                pd.slug,
                m.nombre as marca,
                CAST(pd.precio_venta AS FLOAT8) as precio_venta,
                CAST(pd.precio_base AS FLOAT8) as precio_base,
                CASE 
                    WHEN pd.descuento_adicional_activo THEN CAST(pd.descuento_adicional_porcentaje AS FLOAT8)
                    ELSE NULL
                END as descuento_porcentaje,
                pd.imagen_principal,
                pd.es_destacado,
                pd.es_nuevo,
                pd.es_oferta,
                COALESCE(i.cantidad_disponible, 0) as stock_disponible,
                CAST(p.valoracion_promedio AS FLOAT8) as valoracion_promedio,
                p.total_valoraciones,
                c.nombre as categoria
            FROM producto_detalle pd
            INNER JOIN producto p ON pd.id_producto = p.id_producto
            INNER JOIN marca m ON pd.id_marca = m.id_marca
            INNER JOIN categoria c ON p.id_categoria = c.id_categoria
            LEFT JOIN inventario i ON pd.id_producto_detalle = i.id_producto_detalle
            WHERE pd.estado = 'activo' AND p.estado = 'activo'
            "#,
        );

        // Aplicar filtros dinámicamente
        let mut conditions = Vec::new();
        
        if let Some(ref search) = filters.search {
            conditions.push(format!(
                "(pd.nombre ILIKE '%{}%' OR pd.descripcion ILIKE '%{}%' OR pd.modelo ILIKE '%{}%')",
                search, search, search
            ));
        }

        if let Some(categoria_id) = filters.id_categoria {
            conditions.push(format!("p.id_categoria = {}", categoria_id));
        }

        if let Some(subcategoria_id) = filters.id_subcategoria {
            conditions.push(format!("p.id_subcategoria = {}", subcategoria_id));
        }

        if let Some(marca_id) = filters.id_marca {
            conditions.push(format!("pd.id_marca = {}", marca_id));
        }

        if let Some(familia_id) = filters.id_familia {
            conditions.push(format!("c.id_familia = {}", familia_id));
        }

        if let Some(precio_min) = filters.precio_min {
            conditions.push(format!("pd.precio_venta >= {}", precio_min));
        }

        if let Some(precio_max) = filters.precio_max {
            conditions.push(format!("pd.precio_venta <= {}", precio_max));
        }

        if let Some(true) = filters.destacados {
            conditions.push("pd.es_destacado = true".to_string());
        }

        if let Some(true) = filters.nuevos {
            conditions.push("pd.es_nuevo = true".to_string());
        }

        if let Some(true) = filters.ofertas {
            conditions.push("pd.es_oferta = true".to_string());
        }

        if let Some(true) = filters.en_stock {
            conditions.push("i.cantidad_disponible > 0".to_string());
        }

        if !conditions.is_empty() {
            query.push_str(" AND ");
            query.push_str(&conditions.join(" AND "));
        }

        // Ordenamiento
        let order_by = match filters.order_by.as_deref() {
            Some("precio_asc") => "pd.precio_venta ASC",
            Some("precio_desc") => "pd.precio_venta DESC",
            Some("nombre_asc") => "pd.nombre ASC",
            Some("nombre_desc") => "pd.nombre DESC",
            Some("valoracion") => "p.valoracion_promedio DESC NULLS LAST",
            Some("mas_vendidos") => "pd.total_vendidos DESC",
            Some("nuevos") => "pd.fecha_creacion DESC",
            _ => "pd.id_producto_detalle DESC", // Por defecto: más recientes
        };

        query.push_str(&format!(" ORDER BY {} LIMIT {} OFFSET {}", order_by, limit, offset));

        let productos = sqlx::query_as::<_, producto_detalle::ProductoListItem>(&query)
            .fetch_all(pool)
            .await?;

        Ok(productos)
    }

    // ==================== CONTEO DE PRODUCTOS ====================
    pub async fn count_productos(
        pool: &PgPool,
        filters: &ProductoFilters,
    ) -> Result<i64, sqlx::Error> {
        let mut query = String::from(
            r#"
            SELECT COUNT(DISTINCT pd.id_producto_detalle)
            FROM producto_detalle pd
            INNER JOIN producto p ON pd.id_producto = p.id_producto
            INNER JOIN marca m ON pd.id_marca = m.id_marca
            INNER JOIN categoria c ON p.id_categoria = c.id_categoria
            LEFT JOIN inventario i ON pd.id_producto_detalle = i.id_producto_detalle
            WHERE pd.estado = 'activo' AND p.estado = 'activo'
            "#,
        );

        let mut conditions = Vec::new();

        if let Some(ref search) = filters.search {
            conditions.push(format!(
                "(pd.nombre ILIKE '%{}%' OR pd.descripcion ILIKE '%{}%' OR pd.modelo ILIKE '%{}%')",
                search, search, search
            ));
        }

        if let Some(categoria_id) = filters.id_categoria {
            conditions.push(format!("p.id_categoria = {}", categoria_id));
        }

        if let Some(subcategoria_id) = filters.id_subcategoria {
            conditions.push(format!("p.id_subcategoria = {}", subcategoria_id));
        }

        if let Some(marca_id) = filters.id_marca {
            conditions.push(format!("pd.id_marca = {}", marca_id));
        }

        if let Some(familia_id) = filters.id_familia {
            conditions.push(format!("c.id_familia = {}", familia_id));
        }

        if let Some(precio_min) = filters.precio_min {
            conditions.push(format!("pd.precio_venta >= {}", precio_min));
        }

        if let Some(precio_max) = filters.precio_max {
            conditions.push(format!("pd.precio_venta <= {}", precio_max));
        }

        if let Some(true) = filters.destacados {
            conditions.push("pd.es_destacado = true".to_string());
        }

        if let Some(true) = filters.nuevos {
            conditions.push("pd.es_nuevo = true".to_string());
        }

        if let Some(true) = filters.ofertas {
            conditions.push("pd.es_oferta = true".to_string());
        }

        if let Some(true) = filters.en_stock {
            conditions.push("i.cantidad_disponible > 0".to_string());
        }

        if !conditions.is_empty() {
            query.push_str(" AND ");
            query.push_str(&conditions.join(" AND "));
        }

        let count: (i64,) = sqlx::query_as(&query).fetch_one(pool).await?;

        Ok(count.0)
    }

    // ==================== PRODUCTO DETALLE (INDIVIDUAL) ====================
    pub async fn get_producto_by_id(
        pool: &PgPool,
        id: i32,
    ) -> Result<Option<producto_detalle::ProductoDetalleResponse>, sqlx::Error> {
        let producto = sqlx::query_as!(
            producto_detalle::ProductoDetalleResponse,
            r#"
            SELECT 
                pd.id_producto_detalle,
                pd.nombre,
                pd.descripcion,
                pd.modelo,
                pd.sku,
                pd.slug,
                m.nombre as marca,
                m.logo as marca_logo,
                CAST(pd.precio_base AS FLOAT8) as "precio_base?",
                CAST(pd.precio_venta AS FLOAT8) as "precio_venta?",
                CASE 
                    WHEN pd.descuento_adicional_activo THEN CAST(pd.descuento_adicional_porcentaje AS FLOAT8)
                    ELSE NULL
                END as "descuento_porcentaje?",
                pd.imagen_principal,
                pd.imagenes,
                CAST(pd.peso AS FLOAT8) as "peso?",
                pd.garantia_meses as "garantia_meses?",
                pd.total_vendidos as "total_vendidos?",
                pd.es_destacado as "es_destacado?",
                pd.es_nuevo as "es_nuevo?",
                pd.es_oferta as "es_oferta?",
                COALESCE(i.cantidad_disponible, 0) as "stock_disponible?",
                CASE 
                    WHEN i.cantidad_disponible = 0 OR i.cantidad_disponible IS NULL THEN 'agotado'
                    WHEN i.cantidad_disponible < i.cantidad_minima THEN 'bajo'
                    ELSE 'disponible'
                END as "stock_estado?",
                p.nombre as producto_nombre,
                p.slug as producto_slug,
                c.nombre as categoria_nombre,
                p.especificaciones_base,
                CAST(p.valoracion_promedio AS FLOAT8) as "valoracion_promedio?",
                p.total_valoraciones as "total_valoraciones?"
            FROM producto_detalle pd
            INNER JOIN producto p ON pd.id_producto = p.id_producto
            INNER JOIN marca m ON pd.id_marca = m.id_marca
            INNER JOIN categoria c ON p.id_categoria = c.id_categoria
            LEFT JOIN inventario i ON pd.id_producto_detalle = i.id_producto_detalle
            WHERE pd.id_producto_detalle = $1 AND pd.estado = 'activo'
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(producto)
    }

    // ==================== PRODUCTO DETALLE POR SLUG ====================
    pub async fn get_producto_by_slug(
        pool: &PgPool,
        slug: &str,
    ) -> Result<Option<producto_detalle::ProductoDetalleResponse>, sqlx::Error> {
        let producto = sqlx::query_as!(
            producto_detalle::ProductoDetalleResponse,
            r#"
            SELECT 
                pd.id_producto_detalle,
                pd.nombre,
                pd.descripcion,
                pd.modelo,
                pd.sku,
                pd.slug,
                m.nombre as marca,
                m.logo as marca_logo,
                CAST(pd.precio_base AS FLOAT8) as "precio_base?",
                CAST(pd.precio_venta AS FLOAT8) as "precio_venta?",
                CASE 
                    WHEN pd.descuento_adicional_activo THEN CAST(pd.descuento_adicional_porcentaje AS FLOAT8)
                    ELSE NULL
                END as "descuento_porcentaje?",
                pd.imagen_principal,
                pd.imagenes,
                CAST(pd.peso AS FLOAT8) as "peso?",
                pd.garantia_meses as "garantia_meses?",
                pd.total_vendidos as "total_vendidos?",
                pd.es_destacado as "es_destacado?",
                pd.es_nuevo as "es_nuevo?",
                pd.es_oferta as "es_oferta?",
                COALESCE(i.cantidad_disponible, 0) as "stock_disponible?",
                CASE 
                    WHEN i.cantidad_disponible = 0 OR i.cantidad_disponible IS NULL THEN 'agotado'
                    WHEN i.cantidad_disponible < i.cantidad_minima THEN 'bajo'
                    ELSE 'disponible'
                END as "stock_estado?",
                p.nombre as producto_nombre,
                p.slug as producto_slug,
                c.nombre as categoria_nombre,
                p.especificaciones_base,
                CAST(p.valoracion_promedio AS FLOAT8) as "valoracion_promedio?",
                p.total_valoraciones as "total_valoraciones?"
            FROM producto_detalle pd
            INNER JOIN producto p ON pd.id_producto = p.id_producto
            INNER JOIN marca m ON pd.id_marca = m.id_marca
            INNER JOIN categoria c ON p.id_categoria = c.id_categoria
            LEFT JOIN inventario i ON pd.id_producto_detalle = i.id_producto_detalle
            WHERE pd.slug = $1 AND pd.estado = 'activo'
            "#,
            slug
        )
        .fetch_optional(pool)
        .await?;

        Ok(producto)
    }

    // ==================== INCREMENTAR VISTAS ====================
    pub async fn increment_views(pool: &PgPool, id: i32) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE producto_detalle SET vistas = vistas + 1 WHERE id_producto_detalle = $1",
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    // ==================== VALORACIONES ====================
    pub async fn get_valoraciones(
        pool: &PgPool,
        id_producto_detalle: i32,
    ) -> Result<Vec<Valoracion>, sqlx::Error> {
        let rows = sqlx::query!(
            r#"
            SELECT 
                v.id_valoracion,
                v.id_producto,
                v.id_usuario,
                v.id_producto_detalle,
                v.calificacion,
                v.titulo,
                v.comentario,
                v.compra_verificada,
                v.votos_util,
                v.votos_no_util,
                v.aprobado,
                v.fecha_creacion,
                u.nombre as usuario_nombre,
                u.apellido as usuario_apellido,
                COALESCE(
                    (SELECT array_agg(url_imagen ORDER BY orden) 
                     FROM imagen_valoracion 
                     WHERE id_valoracion = v.id_valoracion), 
                    ARRAY[]::TEXT[]
                ) as imagenes
            FROM valoracion v
            INNER JOIN usuario u ON v.id_usuario = u.id_usuario
            WHERE v.id_producto_detalle = $1
              AND v.aprobado = TRUE
            ORDER BY v.fecha_creacion DESC
            LIMIT 50
            "#,
            id_producto_detalle
        )
        .fetch_all(pool)
        .await?;

        let valoraciones = rows
            .into_iter()
            .map(|row| Valoracion {
                id_valoracion: row.id_valoracion,
                id_producto: row.id_producto,
                id_usuario: row.id_usuario,
                id_producto_detalle: row.id_producto_detalle,
                calificacion: row.calificacion,
                titulo: row.titulo,
                comentario: row.comentario,
                compra_verificada: row.compra_verificada.unwrap_or(false),
                votos_util: row.votos_util.unwrap_or(0),
                votos_no_util: row.votos_no_util.unwrap_or(0),
                aprobado: row.aprobado.unwrap_or(false),
                fecha_creacion: row.fecha_creacion
                    .map(|dt| {
                        let unix_ts = dt.assume_utc().unix_timestamp();
                        DateTime::from_timestamp(unix_ts, 0).unwrap_or_else(|| Utc::now())
                    })
                    .unwrap_or_else(|| Utc::now()),
                usuario_nombre: Some(row.usuario_nombre),
                usuario_apellido: Some(row.usuario_apellido),
                imagenes: row.imagenes.and_then(|imgs| {
                    if imgs.is_empty() {
                        None
                    } else {
                        Some(imgs)
                    }
                }),
            })
            .collect();

        Ok(valoraciones)
    }
}

pub async fn crear_valoracion(
    pool: &PgPool,
    id_producto_detalle: i32,
    id_usuario: i32,
    calificacion: i32,
    titulo: Option<String>,
    comentario: Option<String>,
) -> Result<Valoracion, sqlx::Error> {
    // Obtener id_producto a partir del producto_detalle
    let prod = sqlx::query!(
        "SELECT id_producto FROM producto_detalle WHERE id_producto_detalle = $1",
        id_producto_detalle
    )
    .fetch_one(pool)
    .await?;

    let id_producto = prod.id_producto;

    let inserted = sqlx::query!(
        r#"
        INSERT INTO valoracion (
            id_producto,
            id_usuario,
            id_producto_detalle,
            calificacion,
            titulo,
            comentario,
            compra_verificada,
            votos_util,
            votos_no_util,
            aprobado,
            fecha_creacion
        )
        VALUES ($1, $2, $3, $4, $5, $6, FALSE, 0, 0, TRUE, CURRENT_TIMESTAMP)
        RETURNING
            id_valoracion,
            id_producto,
            id_usuario,
            id_producto_detalle,
            calificacion,
            titulo,
            comentario,
            compra_verificada,
            votos_util,
            votos_no_util,
            aprobado,
            fecha_creacion
        "#,
        id_producto,
        id_usuario,
        id_producto_detalle,
        calificacion,
        titulo,
        comentario
    )
    .fetch_one(pool)
    .await?;

    // Convertir fecha_creacion (PrimitiveDateTime) a DateTime<Utc>
    let fecha_creacion_utc = inserted
        .fecha_creacion
        .map(|dt| {
            let unix_ts = dt.assume_utc().unix_timestamp();
            DateTime::from_timestamp(unix_ts, 0).unwrap_or_else(|| Utc::now())
        })
        .unwrap_or_else(|| Utc::now());

    Ok(Valoracion {
        id_valoracion: inserted.id_valoracion,
        id_producto: inserted.id_producto,
        id_usuario: inserted.id_usuario,
        id_producto_detalle: inserted.id_producto_detalle,
        calificacion: inserted.calificacion,
        titulo: inserted.titulo,
        comentario: inserted.comentario,
        compra_verificada: inserted.compra_verificada.unwrap_or(false),
        votos_util: inserted.votos_util.unwrap_or(0),
        votos_no_util: inserted.votos_no_util.unwrap_or(0),
        aprobado: inserted.aprobado.unwrap_or(false),
        fecha_creacion: fecha_creacion_utc,
        usuario_nombre: None,
        usuario_apellido: None,
        imagenes: None,
    })
}

// ==================== FILTROS ====================
#[derive(Debug, Clone)]
pub struct ProductoFilters {
    pub search: Option<String>,
    pub id_categoria: Option<i32>,
    pub id_subcategoria: Option<i32>,
    pub id_marca: Option<i32>,
    pub id_familia: Option<i32>,
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

impl Default for ProductoFilters {
    fn default() -> Self {
        Self {
            search: None,
            id_categoria: None,
            id_subcategoria: None,
            id_marca: None,
            id_familia: None,
            precio_min: None,
            precio_max: None,
            destacados: None,
            nuevos: None,
            ofertas: None,
            en_stock: None,
            order_by: None,
            limit: Some(20),
            offset: Some(0),
        }
    }
}
