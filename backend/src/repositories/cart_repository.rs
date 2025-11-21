use crate::models::{CartItemResponse, CartOwner};
use rust_decimal::prelude::ToPrimitive;
use sqlx::PgPool;

pub struct CartRepository;

impl CartRepository {
    pub async fn get_or_create_cart(pool: &PgPool, owner: &CartOwner) -> Result<i32, sqlx::Error> {
        if let Some(id) = Self::find_active_cart(pool, owner).await? {
            return Ok(id);
        }

        let cart_id = sqlx::query!(
            r#"
            INSERT INTO carrito (id_usuario, id_sesion, estado, fecha_expiracion)
            VALUES ($1, $2, 'activo', NOW() + INTERVAL '7 days')
            RETURNING id_carrito
            "#,
            owner.user_id,
            owner.session_id
        )
        .fetch_one(pool)
        .await?
        .id_carrito;

        Ok(cart_id)
    }

    pub async fn find_active_cart(pool: &PgPool, owner: &CartOwner) -> Result<Option<i32>, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            SELECT id_carrito
            FROM carrito
            WHERE estado = 'activo'
              AND (
                    ($1::INT IS NOT NULL AND id_usuario = $1)
                 OR ($1::INT IS NULL AND $2::TEXT IS NOT NULL AND id_usuario IS NULL AND id_sesion = $2)
              )
            ORDER BY id_carrito DESC
            LIMIT 1
            "#,
            owner.user_id,
            owner.session_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(result.map(|row| row.id_carrito))
    }

    pub async fn get_cart_items(pool: &PgPool, cart_id: i32) -> Result<Vec<CartItemResponse>, sqlx::Error> {
        let rows = sqlx::query!(
            r#"
            SELECT
                cd.id_carrito_detalle       AS "id_carrito_detalle?",
                cd.id_producto_detalle      AS "id_producto_detalle?",
                cd.cantidad                 AS "cantidad?",
                cd.precio_unitario          AS "precio_unitario?",
                pd.nombre,
                pd.sku,
                pd.imagen_principal,
                i.cantidad_disponible       AS "cantidad_disponible?"
            FROM carrito c
            LEFT JOIN carrito_detalle cd ON c.id_carrito = cd.id_carrito
            LEFT JOIN producto_detalle pd ON cd.id_producto_detalle = pd.id_producto_detalle
            LEFT JOIN inventario i ON i.id_producto_detalle = cd.id_producto_detalle
            WHERE c.id_carrito = $1
            ORDER BY cd.fecha_agregado ASC
            "#,
            cart_id
        )
        .fetch_all(pool)
        .await?;

        let items = rows
            .into_iter()
            .filter_map(|row| {
                let cart_item_id = row.id_carrito_detalle?;
                let product_id = row.id_producto_detalle?;
                let cantidad = row.cantidad?;
                let precio_unitario = row
                    .precio_unitario?
                    .to_f64()
                    .unwrap_or_default();

                Some(CartItemResponse {
                    id_carrito_detalle: cart_item_id,
                    id_producto_detalle: product_id,
                    nombre: row.nombre,
                    sku: row.sku,
                    imagen_principal: row.imagen_principal,
                    precio_unitario,
                    cantidad,
                    stock_disponible: row.cantidad_disponible,
                })
            })
            .collect();

        Ok(items)
    }

    pub async fn add_item(pool: &PgPool, cart_id: i32, product_id: i32, quantity: i32) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO carrito_detalle (id_carrito, id_producto_detalle, cantidad, precio_unitario)
            VALUES (
                $1,
                $2,
                $3,
                (SELECT precio_venta FROM producto_detalle WHERE id_producto_detalle = $2)
            )
            ON CONFLICT (id_carrito, id_producto_detalle)
            DO UPDATE SET
                cantidad = carrito_detalle.cantidad + EXCLUDED.cantidad,
                fecha_actualizacion = CURRENT_TIMESTAMP
            "#,
            cart_id,
            product_id,
            quantity
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn update_item_quantity(
        pool: &PgPool,
        cart_id: i32,
        cart_item_id: i32,
        quantity: i32,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            UPDATE carrito_detalle
            SET cantidad = $1,
                fecha_actualizacion = CURRENT_TIMESTAMP
            WHERE id_carrito_detalle = $2 AND id_carrito = $3
            RETURNING id_carrito_detalle
            "#,
            quantity,
            cart_item_id,
            cart_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(result.is_some())
    }

    pub async fn remove_item(pool: &PgPool, cart_id: i32, cart_item_id: i32) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM carrito_detalle
            WHERE id_carrito_detalle = $1 AND id_carrito = $2
            RETURNING id_carrito_detalle
            "#,
            cart_item_id,
            cart_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(result.is_some())
    }
}
