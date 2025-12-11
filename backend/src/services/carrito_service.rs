use sqlx::PgPool;
use rust_decimal::Decimal;
use crate::models::{CarritoResponse, AgregarAlCarritoRequest, ActualizarCantidadRequest};
use crate::repositories::CarritoRepository;

pub struct CarritoService;

impl CarritoService {
    // Obtener carrito del usuario
    pub async fn get_carrito(pool: &PgPool, id_usuario: i32) -> Result<CarritoResponse, String> {
        // Obtener o crear carrito
        let carrito = CarritoRepository::get_or_create_carrito_usuario(pool, id_usuario)
            .await
            .map_err(|e| format!("Error al obtener carrito: {}", e))?;

        // Obtener items del carrito
        let items = CarritoRepository::get_carrito_items(pool, carrito.id_carrito)
            .await
            .map_err(|e| format!("Error al obtener items: {}", e))?;

        // Calcular totales
        let total_items: i32 = items.iter().map(|item| item.cantidad).sum();
        let subtotal_decimal: Decimal = items.iter()
            .map(|item| item.subtotal)
            .fold(Decimal::ZERO, |acc, x| acc + x);

        Ok(CarritoResponse {
            id_carrito: carrito.id_carrito,
            id_usuario: carrito.id_usuario,
            items,
            total_items,
            subtotal: subtotal_decimal,
            fecha_actualizacion: carrito.fecha_actualizacion,
        })
    }

    // Agregar producto al carrito
    pub async fn agregar_producto(
        pool: &PgPool,
        id_usuario: i32,
        request: AgregarAlCarritoRequest,
    ) -> Result<CarritoResponse, String> {
        // Validar cantidad
        if request.cantidad <= 0 {
            return Err("La cantidad debe ser mayor a 0".to_string());
        }

        // Obtener información del producto
        let producto = sqlx::query!(
            r#"
            SELECT
                pd.id_producto_detalle,
                pd.precio_venta,
                pd.estado::TEXT as estado,
                COALESCE(i.cantidad_disponible, 0) as "stock_disponible!"
            FROM producto_detalle pd
            LEFT JOIN inventario i ON pd.id_producto_detalle = i.id_producto_detalle
            WHERE pd.id_producto_detalle = $1
            "#,
            request.id_producto_detalle
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("Error al buscar producto: {}", e))?;

        let producto = producto.ok_or("Producto no encontrado")?;

        // Validar estado y stock
        if producto.estado.as_deref() != Some("activo") {
            return Err("El producto no está disponible".to_string());
        }

        if request.cantidad > producto.stock_disponible {
            return Err(format!(
                "Stock insuficiente. Disponible: {}",
                producto.stock_disponible
            ));
        }

        // Obtener o crear carrito
        let carrito = CarritoRepository::get_or_create_carrito_usuario(pool, id_usuario)
            .await
            .map_err(|e| format!("Error al obtener carrito: {}", e))?;

        // Agregar item al carrito
        CarritoRepository::add_item(
            pool,
            carrito.id_carrito,
            request.id_producto_detalle,
            request.cantidad,
            producto.precio_venta,
        )
        .await
        .map_err(|e| format!("Error al agregar al carrito: {}", e))?;

        // Actualizar timestamp del carrito
        CarritoRepository::update_carrito_timestamp(pool, carrito.id_carrito)
            .await
            .map_err(|e| format!("Error al actualizar carrito: {}", e))?;

        // Devolver carrito actualizado
        Self::get_carrito(pool, id_usuario).await
    }

    // Actualizar cantidad de un item
    pub async fn actualizar_cantidad(
        pool: &PgPool,
        id_usuario: i32,
        id_carrito_detalle: i32,
        request: ActualizarCantidadRequest,
    ) -> Result<CarritoResponse, String> {
        // Validar cantidad
        if request.cantidad <= 0 {
            return Err("La cantidad debe ser mayor a 0".to_string());
        }

        // Verificar que el item pertenece al usuario
        let item = sqlx::query!(
            r#"
            SELECT
                cd.id_carrito_detalle,
                cd.id_producto_detalle,
                c.id_usuario
            FROM carrito_detalle cd
            INNER JOIN carrito c ON cd.id_carrito = c.id_carrito
            WHERE cd.id_carrito_detalle = $1
            "#,
            id_carrito_detalle
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("Error al buscar item: {}", e))?;

        let item = item.ok_or("Item no encontrado en el carrito")?;

        // Verificar permisos
        if item.id_usuario != Some(id_usuario) {
            return Err("No tienes permiso para modificar este item".to_string());
        }

        // Verificar stock disponible
        let stock = sqlx::query!(
            r#"
            SELECT COALESCE(cantidad_disponible, 0) as "stock!"
            FROM inventario
            WHERE id_producto_detalle = $1
            "#,
            item.id_producto_detalle
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("Error al verificar stock: {}", e))?;

        if let Some(stock_info) = stock {
            if request.cantidad > stock_info.stock {
                return Err(format!("Stock insuficiente. Disponible: {}", stock_info.stock));
            }
        }

        // Actualizar cantidad
        CarritoRepository::update_item_cantidad(pool, id_carrito_detalle, request.cantidad)
            .await
            .map_err(|e| format!("Error al actualizar cantidad: {}", e))?;

        // Devolver carrito actualizado
        Self::get_carrito(pool, id_usuario).await
    }

    // Eliminar item del carrito
    pub async fn eliminar_item(
        pool: &PgPool,
        id_usuario: i32,
        id_carrito_detalle: i32,
    ) -> Result<CarritoResponse, String> {
        // Verificar que el item pertenece al usuario
        let item = sqlx::query!(
            r#"
            SELECT c.id_usuario
            FROM carrito_detalle cd
            INNER JOIN carrito c ON cd.id_carrito = c.id_carrito
            WHERE cd.id_carrito_detalle = $1
            "#,
            id_carrito_detalle
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("Error al buscar item: {}", e))?;

        let item = item.ok_or("Item no encontrado en el carrito")?;

        // Verificar permisos
        if item.id_usuario != Some(id_usuario) {
            return Err("No tienes permiso para eliminar este item".to_string());
        }

        // Eliminar item
        CarritoRepository::remove_item(pool, id_carrito_detalle)
            .await
            .map_err(|e| format!("Error al eliminar item: {}", e))?;

        // Devolver carrito actualizado
        Self::get_carrito(pool, id_usuario).await
    }

    // Limpiar carrito
    pub async fn limpiar_carrito(
        pool: &PgPool,
        id_usuario: i32,
    ) -> Result<CarritoResponse, String> {
        // Obtener carrito
        let carrito = CarritoRepository::get_or_create_carrito_usuario(pool, id_usuario)
            .await
            .map_err(|e| format!("Error al obtener carrito: {}", e))?;

        // Limpiar items
        CarritoRepository::clear_carrito(pool, carrito.id_carrito)
            .await
            .map_err(|e| format!("Error al limpiar carrito: {}", e))?;

        // Devolver carrito vacío
        Self::get_carrito(pool, id_usuario).await
    }
}
