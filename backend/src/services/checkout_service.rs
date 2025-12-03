use sqlx::PgPool;
use rust_decimal::Decimal;
use crate::models::{
    VentaResponse, DetalleVentaResponse, ProcesarCheckoutRequest,
    CalcularTotalResponse, MetodoPagoResponse, EstadoPedido, EstadoPago,
};
use crate::repositories::{CheckoutRepository, DireccionRepository, CarritoRepository};

pub struct CheckoutService;

impl CheckoutService {
    /// Obtener métodos de pago disponibles
    pub async fn get_metodos_pago(pool: &PgPool) -> Result<Vec<MetodoPagoResponse>, String> {
        let metodos = CheckoutRepository::get_metodos_pago_activos(pool)
            .await
            .map_err(|e| format!("Error al obtener métodos de pago: {}", e))?;

        Ok(metodos.into_iter().map(MetodoPagoResponse::from).collect())
    }

    /// Calcular total del carrito (con descuentos y envío)
    pub async fn calcular_total(
        pool: &PgPool,
        id_usuario: i32,
        _id_direccion: Option<i32>, // Para futuro: calcular costo envío según ubicación
        // FUTURO: codigo_cupon: Option<String> - Para aplicar cupones
    ) -> Result<CalcularTotalResponse, String> {
        // Obtener carrito del usuario
        let carrito = CarritoRepository::get_or_create_carrito_usuario(pool, id_usuario)
            .await
            .map_err(|e| format!("Error al obtener carrito: {}", e))?;

        // Obtener items del carrito
        let items = CarritoRepository::get_carrito_items(pool, carrito.id_carrito)
            .await
            .map_err(|e| format!("Error al obtener items: {}", e))?;

        if items.is_empty() {
            return Err("El carrito está vacío".to_string());
        }

        // Calcular subtotal usando Decimal
        let subtotal_decimal: Decimal = items.iter()
            .map(|item| item.subtotal)
            .fold(Decimal::ZERO, |acc, x| acc + x);

        // FUTURO: Aplicar descuentos de cupón
        // TODO: Implementar lógica de validación y aplicación de cupones
        // let descuento_cupon = if let Some(codigo) = codigo_cupon {
        //     validar_y_calcular_descuento_cupon(pool, &codigo, subtotal_decimal).await?
        // } else {
        //     Decimal::ZERO
        // };
        let descuento_cupon = Decimal::ZERO;

        // Calcular descuentos de productos (ya incluidos en precio_venta)
        let descuento_productos = Decimal::ZERO; // Los descuentos ya están en el precio

        let descuento_total = descuento_cupon + descuento_productos;

        // Calcular costo de envío
        // TODO: Implementar lógica de cálculo según configuración del admin
        // Por ahora, envío gratis si la compra es mayor a S/. 100, sino S/. 15
        let costo_envio = if subtotal_decimal >= Decimal::from(100) {
            Decimal::ZERO
        } else {
            Decimal::from(15)
        };

        // Calcular total
        let total = subtotal_decimal - descuento_total + costo_envio;

        Ok(CalcularTotalResponse {
            subtotal: subtotal_decimal,
            descuento_total,
            costo_envio,
            total,
            items_count: items.len() as i32,
        })
    }

    /// Procesar checkout completo (TRANSACCIONAL)
    pub async fn procesar_checkout(
        pool: &PgPool,
        id_usuario: i32,
        request: ProcesarCheckoutRequest,
        ip_cliente: Option<String>,
        user_agent: Option<String>,
    ) -> Result<VentaResponse, String> {
        // Iniciar transacción
        let mut tx = pool
            .begin()
            .await
            .map_err(|e| format!("Error al iniciar transacción: {}", e))?;

        // ========== VALIDACIONES ==========

        // 1. Validar dirección
        let direccion = DireccionRepository::get_direccion_by_id(pool, request.id_direccion, id_usuario)
            .await
            .map_err(|e| format!("Error al buscar dirección: {}", e))?
            .ok_or("Dirección no encontrada")?;

        if !direccion.activo.unwrap_or(true) {
            return Err("La dirección seleccionada no está activa".to_string());
        }

        // 2. Validar método de pago
        let metodo_pago = CheckoutRepository::get_metodo_pago_by_id(pool, request.id_metodo_pago)
            .await
            .map_err(|e| format!("Error al buscar método de pago: {}", e))?
            .ok_or("Método de pago no encontrado o inactivo")?;

        // 3. Obtener carrito activo
        let carrito = CarritoRepository::get_or_create_carrito_usuario(pool, id_usuario)
            .await
            .map_err(|e| format!("Error al obtener carrito: {}", e))?;

        // 4. Obtener items del carrito
        let items = CarritoRepository::get_carrito_items(pool, carrito.id_carrito)
            .await
            .map_err(|e| format!("Error al obtener items: {}", e))?;

        if items.is_empty() {
            return Err("El carrito está vacío".to_string());
        }

        // 5. Validar stock de todos los productos
        for item in &items {
            if item.cantidad > item.stock_disponible {
                return Err(format!(
                    "Stock insuficiente para '{}'. Disponible: {}, Solicitado: {}",
                    item.nombre, item.stock_disponible, item.cantidad
                ));
            }
        }

        // ========== CALCULAR TOTALES ==========

        let subtotal_decimal: Decimal = items.iter()
            .map(|item| item.subtotal)
            .fold(Decimal::ZERO, |acc, x| acc + x);

        // FUTURO: Validar y aplicar cupón
        // TODO: Implementar validación de cupón (fecha vigencia, usos, compra mínima)
        let descuento_total = Decimal::ZERO;

        // Calcular costo de envío
        let costo_envio = if subtotal_decimal >= Decimal::from(100) {
            Decimal::ZERO
        } else {
            Decimal::from(15)
        };

        let total = subtotal_decimal - descuento_total + costo_envio;

        // Calcular comisión del método de pago
        let comision_porcentaje = metodo_pago.comision_porcentaje.unwrap_or(Decimal::ZERO);
        let comision_fija = metodo_pago.comision_fija.unwrap_or(Decimal::ZERO);
        let comision = (total * comision_porcentaje / Decimal::from(100)) + comision_fija;

        // ========== CREAR VENTA ==========

        // Generar número de pedido
        let numero_pedido = CheckoutRepository::generar_numero_pedido(pool)
            .await
            .map_err(|e| format!("Error al generar número de pedido: {}", e))?;

        // Crear venta con snapshot de dirección
        let venta = CheckoutRepository::crear_venta(
            &mut tx,
            &numero_pedido,
            id_usuario,
            carrito.id_carrito,
            subtotal_decimal,
            descuento_total,
            costo_envio,
            total,
            &direccion,
            request.notas_cliente,
            ip_cliente.clone(),
            user_agent.clone(),
        )
        .await
        .map_err(|e| format!("Error al crear venta: {}", e))?;

        // ========== CREAR DETALLES DE VENTA ==========

        let mut detalles_response = Vec::new();

        for item in &items {
            // Crear detalle con snapshot de precio
            let detalle = CheckoutRepository::crear_detalle_venta(
                &mut tx,
                venta.id_venta,
                item.id_producto_detalle,
                item.id_producto,
                item.cantidad,
                item.precio_unitario, // Ya es Decimal, no necesita conversión
                Decimal::ZERO, // Por ahora sin descuentos adicionales
            )
            .await
            .map_err(|e| format!("Error al crear detalle de venta: {}", e))?;

            // Actualizar inventario (restar stock)
            CheckoutRepository::actualizar_inventario(
                &mut tx,
                item.id_producto_detalle,
                item.cantidad,
            )
            .await
            .map_err(|e| format!("Error al actualizar inventario: {}", e))?;

            // Incrementar total vendidos
            CheckoutRepository::incrementar_total_vendidos(
                &mut tx,
                item.id_producto_detalle,
                item.cantidad,
            )
            .await
            .map_err(|e| format!("Error al actualizar total vendidos: {}", e))?;

            // Agregar a response
            detalles_response.push(DetalleVentaResponse {
                id_detalle_venta: detalle.id_detalle_venta,
                id_producto_detalle: item.id_producto_detalle,
                nombre_producto: item.nombre.clone(),
                sku: item.sku.clone(),
                imagen: item.imagen_principal.clone(),
                cantidad: item.cantidad,
                precio_unitario: detalle.precio_unitario,
                descuento_unitario: detalle.descuento_unitario.unwrap_or(Decimal::ZERO),
                precio_final: detalle.precio_final,
                subtotal: detalle.subtotal,
            });
        }

        // ========== PROCESAR PAGO ==========

        // Crear registro de pago (por ahora simulado)
        CheckoutRepository::crear_pago(
            &mut tx,
            venta.id_venta,
            request.id_metodo_pago,
            total,
            comision,
            ip_cliente,
            user_agent,
        )
        .await
        .map_err(|e| format!("Error al procesar pago: {}", e))?;

        // Actualizar estado de venta a confirmado y pago completado
        CheckoutRepository::actualizar_estado_venta(
            &mut tx,
            venta.id_venta,
            EstadoPedido::Confirmado,
            EstadoPago::Completado,
        )
        .await
        .map_err(|e| format!("Error al actualizar estado: {}", e))?;

        // ========== CONVERTIR CARRITO ==========

        // Marcar carrito como convertido
        CheckoutRepository::convertir_carrito(&mut tx, carrito.id_carrito)
            .await
            .map_err(|e| format!("Error al convertir carrito: {}", e))?;

        // FUTURO: Registrar uso de cupón si se aplicó
        // TODO: Crear registro en tabla uso_cupon

        // ========== COMMIT TRANSACCIÓN ==========

        tx.commit()
            .await
            .map_err(|e| format!("Error al confirmar transacción: {}", e))?;

        // ========== RESPUESTA ==========

        let mut venta_response = VentaResponse::from(venta);
        venta_response.items = detalles_response;
        venta_response.estado = "confirmado".to_string();
        venta_response.estado_pago = "completado".to_string();

        Ok(venta_response)
    }

    /// Obtener pedido por ID
    pub async fn get_pedido(
        pool: &PgPool,
        id_venta: i32,
        id_usuario: i32,
    ) -> Result<VentaResponse, String> {
        let venta = CheckoutRepository::get_venta_by_id(pool, id_venta, id_usuario)
            .await
            .map_err(|e| format!("Error al obtener venta: {}", e))?
            .ok_or("Pedido no encontrado")?;

        let detalles = CheckoutRepository::get_detalles_venta(pool, id_venta)
            .await
            .map_err(|e| format!("Error al obtener detalles: {}", e))?;

        let mut venta_response = VentaResponse::from(venta);
        venta_response.items = detalles;

        Ok(venta_response)
    }

    /// Listar pedidos del usuario
    pub async fn get_pedidos_usuario(
        pool: &PgPool,
        id_usuario: i32,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Vec<VentaResponse>, String> {
        let limit = limit.unwrap_or(20);
        let offset = offset.unwrap_or(0);

        let ventas = CheckoutRepository::get_ventas_usuario(pool, id_usuario, limit, offset)
            .await
            .map_err(|e| format!("Error al obtener pedidos: {}", e))?;

        // Obtener detalles de cada venta
        let mut ventas_response = Vec::new();
        for venta in ventas {
            let detalles = CheckoutRepository::get_detalles_venta(pool, venta.id_venta)
                .await
                .map_err(|e| format!("Error al obtener detalles: {}", e))?;

            let mut venta_response = VentaResponse::from(venta);
            venta_response.items = detalles;
            ventas_response.push(venta_response);
        }

        Ok(ventas_response)
    }
}
