use sqlx::{PgPool, Postgres, Transaction};
use rust_decimal::Decimal;
use chrono::Utc;
use crate::models::{
    Venta, DetalleVenta, MetodoPago, Pago, Direccion,
    EstadoPedido, EstadoPago, DetalleVentaResponse
};

pub struct CheckoutRepository;

impl CheckoutRepository {
    /// Obtener métodos de pago activos
    pub async fn get_metodos_pago_activos(pool: &PgPool) -> Result<Vec<MetodoPago>, sqlx::Error> {
        let metodos = sqlx::query_as!(
            MetodoPago,
            r#"
            SELECT
                id_metodo_pago,
                nombre as "nombre!",
                tipo as "tipo!: _",
                proveedor,
                descripcion,
                icono,
                comision_porcentaje as "comision_porcentaje!",
                comision_fija as "comision_fija!",
                requiere_verificacion as "requiere_verificacion!",
                tiempo_procesamiento,
                instrucciones,
                orden as "orden!",
                activo as "activo!",
                fecha_creacion as "fecha_creacion!: chrono::DateTime<chrono::Utc>"
            FROM metodo_pago
            WHERE activo = TRUE
            ORDER BY orden ASC
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(metodos)
    }

    /// Obtener método de pago por ID
    pub async fn get_metodo_pago_by_id(
        pool: &PgPool,
        id_metodo_pago: i32,
    ) -> Result<Option<MetodoPago>, sqlx::Error> {
        let metodo = sqlx::query_as!(
            MetodoPago,
            r#"
            SELECT
                id_metodo_pago,
                nombre as "nombre!",
                tipo as "tipo!: _",
                proveedor,
                descripcion,
                icono,
                comision_porcentaje as "comision_porcentaje!",
                comision_fija as "comision_fija!",
                requiere_verificacion as "requiere_verificacion!",
                tiempo_procesamiento,
                instrucciones,
                orden as "orden!",
                activo as "activo!",
                fecha_creacion as "fecha_creacion!: chrono::DateTime<chrono::Utc>"
            FROM metodo_pago
            WHERE id_metodo_pago = $1 AND activo = TRUE
            "#,
            id_metodo_pago
        )
        .fetch_optional(pool)
        .await?;

        Ok(metodo)
    }

    /// Generar número de pedido único
    pub async fn generar_numero_pedido(pool: &PgPool) -> Result<String, sqlx::Error> {
        let fecha = Utc::now();
        let fecha_str = fecha.format("%Y%m%d").to_string();

        // Obtener contador del día
        let contador = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) as "count!"
            FROM venta
            WHERE DATE(fecha_pedido) = CURRENT_DATE
            "#
        )
        .fetch_one(pool)
        .await?;

        let numero_pedido = format!("PED-{}-{:04}", fecha_str, contador + 1);
        Ok(numero_pedido)
    }

    /// Crear venta (dentro de transacción)
    pub async fn crear_venta(
        tx: &mut Transaction<'_, Postgres>,
        numero_pedido: &str,
        id_usuario: i32,
        id_carrito: i32,
        subtotal: Decimal,
        descuento_total: Decimal,
        costo_envio: Decimal,
        total: Decimal,
        direccion: &Direccion,
        notas_cliente: Option<String>,
        ip_cliente: Option<String>,
        user_agent: Option<String>,
    ) -> Result<Venta, sqlx::Error> {
        // Crear snapshot de dirección
        let direccion_completa = format!(
            "{}, {}",
            direccion.direccion_linea1,
            direccion.direccion_linea2.as_ref().unwrap_or(&String::new())
        );

        let venta = sqlx::query_as!(
            Venta,
            r#"
            INSERT INTO venta (
                numero_pedido, id_usuario, id_carrito,
                subtotal, descuento_total, costo_envio, total, moneda,
                estado, estado_pago,
                direccion_envio, ciudad, departamento, codigo_postal, telefono_contacto,
                fecha_pedido, notas_cliente, ip_cliente, user_agent
            )
            VALUES (
                $1, $2, $3,
                $4, $5, $6, $7, 'PEN',
                'pendiente'::estado_pedido, 'pendiente'::estado_pago,
                $8, $9, $10, $11, $12,
                CURRENT_TIMESTAMP, $13, $14, $15
            )
            RETURNING
                id_venta,
                numero_pedido as "numero_pedido!",
                id_usuario,
                id_carrito,
                subtotal as "subtotal!",
                descuento_total as "descuento_total!",
                costo_envio as "costo_envio!",
                total as "total!",
                moneda as "moneda!",
                estado as "estado!: _",
                estado_pago as "estado_pago!: _",
                direccion_envio,
                ciudad,
                departamento,
                codigo_postal,
                telefono_contacto,
                metodo_envio,
                numero_tracking,
                fecha_pedido as "fecha_pedido!: chrono::DateTime<chrono::Utc>",
                fecha_pago as "fecha_pago: chrono::DateTime<chrono::Utc>",
                fecha_confirmacion as "fecha_confirmacion: chrono::DateTime<chrono::Utc>",
                fecha_envio as "fecha_envio: chrono::DateTime<chrono::Utc>",
                fecha_entrega_estimada as "fecha_entrega_estimada: chrono::NaiveDate",
                fecha_entrega as "fecha_entrega: chrono::DateTime<chrono::Utc>",
                fecha_cancelacion as "fecha_cancelacion: chrono::DateTime<chrono::Utc>",
                notas_cliente,
                notas_admin,
                ip_cliente,
                user_agent,
                fecha_actualizacion as "fecha_actualizacion!: chrono::DateTime<chrono::Utc>"
            "#,
            numero_pedido,
            id_usuario,
            id_carrito,
            subtotal,
            descuento_total,
            costo_envio,
            total,
            direccion_completa,
            direccion.ciudad,
            direccion.departamento,
            direccion.codigo_postal,
            direccion.telefono_contacto,
            notas_cliente,
            ip_cliente,
            user_agent
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(venta)
    }

    /// Crear detalle de venta
    pub async fn crear_detalle_venta(
        tx: &mut Transaction<'_, Postgres>,
        id_venta: i32,
        id_producto_detalle: i32,
        id_producto: i32,
        cantidad: i32,
        precio_unitario: Decimal,
        descuento_unitario: Decimal,
    ) -> Result<DetalleVenta, sqlx::Error> {
        let precio_final = precio_unitario - descuento_unitario;
        let subtotal = precio_final * Decimal::from(cantidad);

        let detalle = sqlx::query_as!(
            DetalleVenta,
            r#"
            INSERT INTO detalle_venta (
                id_venta, id_producto_detalle, id_producto,
                cantidad, precio_unitario, descuento_unitario, precio_final, subtotal
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING
                id_detalle_venta,
                id_venta,
                id_producto_detalle,
                id_producto,
                cantidad,
                precio_unitario as "precio_unitario!",
                descuento_unitario as "descuento_unitario!",
                precio_final as "precio_final!",
                subtotal as "subtotal!"
            "#,
            id_venta,
            id_producto_detalle,
            id_producto,
            cantidad,
            precio_unitario,
            descuento_unitario,
            precio_final,
            subtotal
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(detalle)
    }

    /// Crear pago
    pub async fn crear_pago(
        tx: &mut Transaction<'_, Postgres>,
        id_venta: i32,
        id_metodo_pago: i32,
        monto: Decimal,
        comision: Decimal,
        ip_cliente: Option<String>,
        user_agent: Option<String>,
    ) -> Result<Pago, sqlx::Error> {
        let monto_neto = monto - comision;
        let numero_transaccion = format!("TXN-{}-{:06}", Utc::now().format("%Y%m%d"), id_venta);

        // NOTA: Por ahora el pago es simulado, se marca como completado inmediatamente
        // TODO: Integrar con procesador de pagos real (Stripe, Culqi, etc.)
        let pago = sqlx::query!(
            r#"
            INSERT INTO pago (
                id_venta, id_metodo_pago,
                numero_transaccion, estado,
                monto, moneda, comision, monto_neto,
                proveedor_pago, fecha_pago,
                ip_cliente, user_agent
            )
            VALUES (
                $1, $2,
                $3, 'completado'::estado_pago,
                $4, 'PEN', $5, $6,
                'SIMULADO', CURRENT_TIMESTAMP,
                $7, $8
            )
            RETURNING
                id_pago,
                id_venta,
                id_metodo_pago,
                id_metodo_pago_cliente,
                numero_transaccion,
                estado::TEXT as "estado!",
                monto as "monto!",
                moneda as "moneda!",
                comision as "comision!",
                monto_neto as "monto_neto!",
                proveedor_pago,
                id_transaccion_proveedor,
                token_pago,
                ultimos_4_digitos,
                marca_tarjeta,
                ip_cliente,
                user_agent,
                fecha_pago as "fecha_pago: chrono::DateTime<chrono::Utc>",
                fecha_creacion as "fecha_creacion!: chrono::DateTime<chrono::Utc>",
                fecha_actualizacion,
                nota_error,
                intentos_fallidos as "intentos_fallidos!"
            "#,
            id_venta,
            id_metodo_pago,
            numero_transaccion,
            monto,
            comision,
            monto_neto,
            ip_cliente,
            user_agent
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(Pago {
            id_pago: pago.id_pago,
            id_venta: pago.id_venta,
            id_metodo_pago: pago.id_metodo_pago,
            id_metodo_pago_cliente: pago.id_metodo_pago_cliente,
            numero_transaccion: pago.numero_transaccion,
            estado: pago.estado,
            monto: pago.monto,
            moneda: pago.moneda,
            comision: pago.comision,
            monto_neto: pago.monto_neto,
            proveedor_pago: pago.proveedor_pago,
            id_transaccion_proveedor: pago.id_transaccion_proveedor,
            token_pago: pago.token_pago,
            respuesta_proveedor: pago.respuesta_proveedor,
            ultimos_4_digitos: pago.ultimos_4_digitos,
            marca_tarjeta: pago.marca_tarjeta,
            ip_cliente: pago.ip_cliente,
            user_agent: pago.user_agent,
            fecha_pago: pago.fecha_pago,
            fecha_creacion: pago.fecha_creacion,
            fecha_actualizacion: pago.fecha_actualizacion,
            nota_error: pago.nota_error,
            intentos_fallidos: pago.intentos_fallidos,
        })
    }

    /// Actualizar estado de venta y pago
    pub async fn actualizar_estado_venta(
        tx: &mut Transaction<'_, Postgres>,
        id_venta: i32,
        estado: EstadoPedido,
        estado_pago: EstadoPago,
    ) -> Result<(), sqlx::Error> {
        let estado_str = format!("{:?}", estado).to_lowercase();
        let estado_pago_str = format!("{:?}", estado_pago).to_lowercase();

        sqlx::query!(
            r#"
            UPDATE venta
            SET
                estado = $2::TEXT::estado_pedido,
                estado_pago = $3::TEXT::estado_pago,
                fecha_confirmacion = CASE WHEN $2 = 'confirmado' THEN CURRENT_TIMESTAMP ELSE fecha_confirmacion END,
                fecha_pago = CASE WHEN $3 = 'completado' THEN CURRENT_TIMESTAMP ELSE fecha_pago END,
                fecha_actualizacion = CURRENT_TIMESTAMP
            WHERE id_venta = $1
            "#,
            id_venta,
            estado_str,
            estado_pago_str
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    /// Convertir carrito (cambiar estado a 'convertido')
    pub async fn convertir_carrito(
        tx: &mut Transaction<'_, Postgres>,
        id_carrito: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE carrito
            SET estado = 'convertido'::estado_carrito
            WHERE id_carrito = $1
            "#,
            id_carrito
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    /// Actualizar inventario (restar stock)
    pub async fn actualizar_inventario(
        tx: &mut Transaction<'_, Postgres>,
        id_producto_detalle: i32,
        cantidad: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE inventario
            SET cantidad_disponible = cantidad_disponible - $2,
                fecha_ultima_salida = CURRENT_DATE,
                fecha_actualizacion = CURRENT_TIMESTAMP
            WHERE id_producto_detalle = $1
            "#,
            id_producto_detalle,
            cantidad
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    /// Incrementar total vendidos del producto
    pub async fn incrementar_total_vendidos(
        tx: &mut Transaction<'_, Postgres>,
        id_producto_detalle: i32,
        cantidad: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE producto_detalle
            SET total_vendidos = total_vendidos + $2,
                fecha_ultima_venta = CURRENT_TIMESTAMP
            WHERE id_producto_detalle = $1
            "#,
            id_producto_detalle,
            cantidad
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    /// Obtener venta por ID con detalles
    pub async fn get_venta_by_id(
        pool: &PgPool,
        id_venta: i32,
        id_usuario: i32,
    ) -> Result<Option<Venta>, sqlx::Error> {
        let venta = sqlx::query_as!(
            Venta,
            r#"
            SELECT
                id_venta,
                numero_pedido as "numero_pedido!",
                id_usuario,
                id_carrito,
                subtotal as "subtotal!",
                descuento_total as "descuento_total!",
                costo_envio as "costo_envio!",
                total as "total!",
                moneda as "moneda!",
                estado as "estado!: _",
                estado_pago as "estado_pago!: _",
                direccion_envio,
                ciudad,
                departamento,
                codigo_postal,
                telefono_contacto,
                metodo_envio,
                numero_tracking,
                fecha_pedido as "fecha_pedido!: chrono::DateTime<chrono::Utc>",
                fecha_pago as "fecha_pago: chrono::DateTime<chrono::Utc>",
                fecha_confirmacion as "fecha_confirmacion: chrono::DateTime<chrono::Utc>",
                fecha_envio as "fecha_envio: chrono::DateTime<chrono::Utc>",
                fecha_entrega_estimada as "fecha_entrega_estimada: chrono::NaiveDate",
                fecha_entrega as "fecha_entrega: chrono::DateTime<chrono::Utc>",
                fecha_cancelacion as "fecha_cancelacion: chrono::DateTime<chrono::Utc>",
                notas_cliente,
                notas_admin,
                ip_cliente,
                user_agent,
                fecha_actualizacion as "fecha_actualizacion!: chrono::DateTime<chrono::Utc>"
            FROM venta
            WHERE id_venta = $1 AND id_usuario = $2
            "#,
            id_venta,
            id_usuario
        )
        .fetch_optional(pool)
        .await?;

        Ok(venta)
    }

    /// Obtener detalles de venta con información de productos
    pub async fn get_detalles_venta(
        pool: &PgPool,
        id_venta: i32,
    ) -> Result<Vec<DetalleVentaResponse>, sqlx::Error> {
        let detalles = sqlx::query_as!(
            DetalleVentaResponse,
            r#"
            SELECT
                dv.id_detalle_venta,
                dv.id_producto_detalle,
                pd.nombre as "nombre_producto!",
                pd.sku as "sku!",
                pd.imagen_principal as imagen,
                dv.cantidad,
                dv.precio_unitario as "precio_unitario!",
                dv.descuento_unitario as "descuento_unitario!",
                dv.precio_final as "precio_final!",
                dv.subtotal as "subtotal!"
            FROM detalle_venta dv
            INNER JOIN producto_detalle pd ON dv.id_producto_detalle = pd.id_producto_detalle
            WHERE dv.id_venta = $1
            ORDER BY dv.id_detalle_venta
            "#,
            id_venta
        )
        .fetch_all(pool)
        .await?;

        Ok(detalles)
    }

    /// Obtener ventas del usuario
    pub async fn get_ventas_usuario(
        pool: &PgPool,
        id_usuario: i32,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<Venta>, sqlx::Error> {
        let ventas = sqlx::query_as!(
            Venta,
            r#"
            SELECT
                id_venta,
                numero_pedido as "numero_pedido!",
                id_usuario,
                id_carrito,
                subtotal as "subtotal!",
                descuento_total as "descuento_total!",
                costo_envio as "costo_envio!",
                total as "total!",
                moneda as "moneda!",
                estado as "estado!: _",
                estado_pago as "estado_pago!: _",
                direccion_envio,
                ciudad,
                departamento,
                codigo_postal,
                telefono_contacto,
                metodo_envio,
                numero_tracking,
                fecha_pedido as "fecha_pedido!: chrono::DateTime<chrono::Utc>",
                fecha_pago as "fecha_pago: chrono::DateTime<chrono::Utc>",
                fecha_confirmacion as "fecha_confirmacion: chrono::DateTime<chrono::Utc>",
                fecha_envio as "fecha_envio: chrono::DateTime<chrono::Utc>",
                fecha_entrega_estimada as "fecha_entrega_estimada: chrono::NaiveDate",
                fecha_entrega as "fecha_entrega: chrono::DateTime<chrono::Utc>",
                fecha_cancelacion as "fecha_cancelacion: chrono::DateTime<chrono::Utc>",
                notas_cliente,
                notas_admin,
                ip_cliente,
                user_agent,
                fecha_actualizacion as "fecha_actualizacion!: chrono::DateTime<chrono::Utc>"
            FROM venta
            WHERE id_usuario = $1
            ORDER BY fecha_pedido DESC
            LIMIT $2 OFFSET $3
            "#,
            id_usuario,
            limit as i64,
            offset as i64
        )
        .fetch_all(pool)
        .await?;

        Ok(ventas)
    }
}
