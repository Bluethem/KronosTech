use sqlx::PgPool;
use crate::models::{Carrito, CarritoDetalle, CarritoItemResponse};
use rust_decimal::Decimal;

pub struct CarritoRepository;

impl CarritoRepository {
    // Obtener o crear carrito activo para un usuario
    pub async fn get_or_create_carrito_usuario(
        pool: &PgPool,
        id_usuario: i32,
    ) -> Result<Carrito, sqlx::Error> {
        // Primero intentar obtener un carrito activo existente
        let carrito = sqlx::query_as!(
            Carrito,
            r#"
            SELECT
                id_carrito,
                id_usuario,
                id_sesion,
                estado::TEXT as "estado!",
                fecha_expiracion,
                fecha_creacion as "fecha_creacion!",
                fecha_actualizacion as "fecha_actualizacion!"
            FROM carrito
            WHERE id_usuario = $1 AND estado = 'activo'
            ORDER BY fecha_actualizacion DESC
            LIMIT 1
            "#,
            id_usuario
        )
        .fetch_optional(pool)
        .await?;

        // Si existe, devolverlo
        if let Some(carrito) = carrito {
            return Ok(carrito);
        }

        // Si no existe, crear uno nuevo
        let nuevo_carrito = sqlx::query_as!(
            Carrito,
            r#"
            INSERT INTO carrito (id_usuario, estado, fecha_expiracion)
            VALUES ($1, 'activo', CURRENT_TIMESTAMP + INTERVAL '7 days')
            RETURNING
                id_carrito,
                id_usuario,
                id_sesion,
                estado::TEXT as "estado!",
                fecha_expiracion,
                fecha_creacion as "fecha_creacion!",
                fecha_actualizacion as "fecha_actualizacion!"
            "#,
            id_usuario
        )
        .fetch_one(pool)
        .await?;

        Ok(nuevo_carrito)
    }

    // Obtener items del carrito con información del producto
    pub async fn get_carrito_items(
        pool: &PgPool,
        id_carrito: i32,
    ) -> Result<Vec<CarritoItemResponse>, sqlx::Error> {
        let items = sqlx::query!(
            r#"
            SELECT
                cd.id_carrito_detalle,
                cd.id_producto_detalle,
                pd.id_producto,
                pd.nombre as "nombre!",
                pd.sku as "sku!",
                pd.imagen_principal,
                cd.precio_unitario as "precio_unitario!",
                cd.cantidad as "cantidad!",
                COALESCE(i.cantidad_disponible, 0) as "stock_disponible!"
            FROM carrito_detalle cd
            INNER JOIN producto_detalle pd ON cd.id_producto_detalle = pd.id_producto_detalle
            LEFT JOIN inventario i ON pd.id_producto_detalle = i.id_producto_detalle
            WHERE cd.id_carrito = $1
            ORDER BY cd.fecha_agregado DESC
            "#,
            id_carrito
        )
        .fetch_all(pool)
        .await?;

        let items_response: Vec<CarritoItemResponse> = items
            .into_iter()
            .map(|row| {
                // Calcular subtotal usando Decimal
                let subtotal = row.precio_unitario * Decimal::from(row.cantidad);

                CarritoItemResponse {
                    id_carrito_detalle: row.id_carrito_detalle,
                    id_producto_detalle: row.id_producto_detalle,
                    id_producto: row.id_producto,
                    nombre: row.nombre,
                    sku: row.sku,
                    imagen_principal: row.imagen_principal,
                    precio_unitario: row.precio_unitario,
                    cantidad: row.cantidad,
                    subtotal,
                    stock_disponible: row.stock_disponible,
                }
            })
            .collect();

        Ok(items_response)
    }

    // Agregar item al carrito
    pub async fn add_item(
        pool: &PgPool,
        id_carrito: i32,
        id_producto_detalle: i32,
        cantidad: i32,
        precio_unitario: Decimal,
    ) -> Result<CarritoDetalle, sqlx::Error> {
        // Verificar si el producto ya existe en el carrito
        let existente = sqlx::query!(
            r#"
            SELECT id_carrito_detalle, cantidad
            FROM carrito_detalle
            WHERE id_carrito = $1 AND id_producto_detalle = $2
            "#,
            id_carrito,
            id_producto_detalle
        )
        .fetch_optional(pool)
        .await?;

        if let Some(item) = existente {
            // Si existe, actualizar la cantidad
            let detalle = sqlx::query_as!(
                CarritoDetalle,
                r#"
                UPDATE carrito_detalle
                SET cantidad = cantidad + $1,
                    fecha_actualizacion = CURRENT_TIMESTAMP
                WHERE id_carrito_detalle = $2
                RETURNING
                    id_carrito_detalle,
                    id_carrito,
                    id_producto_detalle,
                    cantidad,
                    precio_unitario,
                    fecha_agregado as "fecha_agregado!",
                    fecha_actualizacion as "fecha_actualizacion!"
                "#,
                cantidad,
                item.id_carrito_detalle
            )
            .fetch_one(pool)
            .await?;

            Ok(detalle)
        } else {
            // Si no existe, crear nuevo registro
            let detalle = sqlx::query_as!(
                CarritoDetalle,
                r#"
                INSERT INTO carrito_detalle (id_carrito, id_producto_detalle, cantidad, precio_unitario)
                VALUES ($1, $2, $3, $4)
                RETURNING
                    id_carrito_detalle,
                    id_carrito,
                    id_producto_detalle,
                    cantidad,
                    precio_unitario,
                    fecha_agregado as "fecha_agregado!",
                    fecha_actualizacion as "fecha_actualizacion!"
                "#,
                id_carrito,
                id_producto_detalle,
                cantidad,
                precio_unitario
            )
            .fetch_one(pool)
            .await?;

            Ok(detalle)
        }
    }

    // Actualizar cantidad de un item
    pub async fn update_item_cantidad(
        pool: &PgPool,
        id_carrito_detalle: i32,
        cantidad: i32,
    ) -> Result<CarritoDetalle, sqlx::Error> {
        let detalle = sqlx::query_as!(
            CarritoDetalle,
            r#"
            UPDATE carrito_detalle
            SET cantidad = $1,
                fecha_actualizacion = CURRENT_TIMESTAMP
            WHERE id_carrito_detalle = $2
            RETURNING
                id_carrito_detalle,
                id_carrito,
                id_producto_detalle,
                cantidad,
                precio_unitario,
                fecha_agregado as "fecha_agregado!",
                fecha_actualizacion as "fecha_actualizacion!"
            "#,
            cantidad,
            id_carrito_detalle
        )
        .fetch_one(pool)
        .await?;

        Ok(detalle)
    }

    // Eliminar item del carrito
    pub async fn remove_item(
        pool: &PgPool,
        id_carrito_detalle: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM carrito_detalle
            WHERE id_carrito_detalle = $1
            "#,
            id_carrito_detalle
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    // Limpiar todo el carrito
    pub async fn clear_carrito(
        pool: &PgPool,
        id_carrito: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM carrito_detalle
            WHERE id_carrito = $1
            "#,
            id_carrito
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    // Actualizar timestamp del carrito
    pub async fn update_carrito_timestamp(
        pool: &PgPool,
        id_carrito: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE carrito
            SET fecha_actualizacion = CURRENT_TIMESTAMP
            WHERE id_carrito = $1
            "#,
            id_carrito
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
