use sqlx::{PgPool, Postgres, Transaction};
use rust_decimal::Decimal;
use chrono::Utc;
use crate::models::{
    Venta, DetalleVenta, MetodoPago, Pago, Direccion,
    EstadoPedido, EstadoPago, DetalleVentaResponse
};

// Estructura intermedia para mapear queries de venta
#[derive(sqlx::FromRow)]
struct VentaRow {
    id_venta: i32,
    numero_pedido: String,
    id_usuario: i32,
    id_carrito: Option<i32>,
    subtotal: Decimal,
    descuento_total: Option<Decimal>,
    costo_envio: Option<Decimal>,
    total: Decimal,
    moneda: Option<String>,
    estado: String,
    estado_pago: String,
    direccion_envio: Option<String>,
    ciudad: Option<String>,
    departamento: Option<String>,
    codigo_postal: Option<String>,
    telefono_contacto: Option<String>,
    metodo_envio: Option<String>,
    numero_tracking: Option<String>,
    fecha_pedido: chrono::DateTime<chrono::Utc>,
    fecha_pago: Option<chrono::DateTime<chrono::Utc>>,
    fecha_confirmacion: Option<chrono::DateTime<chrono::Utc>>,
    fecha_envio: Option<chrono::DateTime<chrono::Utc>>,
    fecha_entrega_estimada: Option<chrono::NaiveDate>,
    fecha_entrega: Option<chrono::DateTime<chrono::Utc>>,
    fecha_cancelacion: Option<chrono::DateTime<chrono::Utc>>,
    notas_cliente: Option<String>,
    notas_admin: Option<String>,
    ip_cliente: Option<String>,
    user_agent: Option<String>,
    fecha_actualizacion: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<VentaRow> for Venta {
    fn from(row: VentaRow) -> Self {
        Venta {
            id_venta: row.id_venta,
            numero_pedido: row.numero_pedido,
            id_usuario: row.id_usuario,
            id_carrito: row.id_carrito,
            subtotal: row.subtotal,
            descuento_total: row.descuento_total,
            costo_envio: row.costo_envio,
            total: row.total,
            moneda: row.moneda,
            estado: Some(row.estado),
            estado_pago: Some(row.estado_pago),
            direccion_envio: row.direccion_envio,
            ciudad: row.ciudad,
            departamento: row.departamento,
            codigo_postal: row.codigo_postal,
            telefono_contacto: row.telefono_contacto,
            metodo_envio: row.metodo_envio,
            numero_tracking: row.numero_tracking,
            fecha_pedido: Some(row.fecha_pedido.naive_utc()),
            fecha_pago: row.fecha_pago.map(|dt| dt.naive_utc()),
            fecha_confirmacion: row.fecha_confirmacion.map(|dt| dt.naive_utc()),
            fecha_envio: row.fecha_envio.map(|dt| dt.naive_utc()),
            fecha_entrega_estimada: row.fecha_entrega_estimada,
            fecha_entrega: row.fecha_entrega.map(|dt| dt.naive_utc()),
            fecha_cancelacion: row.fecha_cancelacion.map(|dt| dt.naive_utc()),
            notas_cliente: row.notas_cliente,
            notas_admin: row.notas_admin,
            ip_cliente: row.ip_cliente,
            user_agent: row.user_agent,
            fecha_actualizacion: row.fecha_actualizacion.map(|dt| dt.naive_utc()),
        }
    }
}

pub struct CheckoutRepository;

impl CheckoutRepository {
    /// Obtener métodos de pago activos
    pub async fn get_metodos_pago_activos(pool: &PgPool) -> Result<Vec<MetodoPago>, sqlx::Error> {
        #[derive(sqlx::FromRow)]
        struct MetodoPagoRow {
            id_metodo_pago: i32,
            nombre: String,
            tipo: String,
            proveedor: Option<String>,
            descripcion: Option<String>,
            icono: Option<String>,
            comision_porcentaje: rust_decimal::Decimal,
            comision_fija: rust_decimal::Decimal,
            requiere_verificacion: bool,
            tiempo_procesamiento: Option<String>,
            instrucciones: Option<String>,
            orden: i32,
            activo: bool,
            fecha_creacion: chrono::DateTime<chrono::Utc>,
        }

        let rows = sqlx::query_as!(
            MetodoPagoRow,
            r#"
            SELECT
                id_metodo_pago,
                nombre as "nombre!",
                tipo::TEXT as "tipo!",
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

        let metodos = rows.into_iter().map(|row| MetodoPago {
            id_metodo_pago: row.id_metodo_pago,
            nombre: row.nombre,
            tipo: row.tipo,
            proveedor: row.proveedor,
            descripcion: row.descripcion,
            icono: row.icono,
            comision_porcentaje: Some(row.comision_porcentaje),
            comision_fija: Some(row.comision_fija),
            requiere_verificacion: Some(row.requiere_verificacion),
            tiempo_procesamiento: row.tiempo_procesamiento,
            instrucciones: row.instrucciones,
            orden: Some(row.orden),
            activo: Some(row.activo),
            fecha_creacion: Some(row.fecha_creacion.naive_utc()),
        }).collect();

        Ok(metodos)
    }

    /// Obtener método de pago por ID
    pub async fn get_metodo_pago_by_id(
        pool: &PgPool,
        id_metodo_pago: i32,
    ) -> Result<Option<MetodoPago>, sqlx::Error> {
        #[derive(sqlx::FromRow)]
        struct MetodoPagoRow {
            id_metodo_pago: i32,
            nombre: String,
            tipo: String,
            proveedor: Option<String>,
            descripcion: Option<String>,
            icono: Option<String>,
            comision_porcentaje: rust_decimal::Decimal,
            comision_fija: rust_decimal::Decimal,
            requiere_verificacion: bool,
            tiempo_procesamiento: Option<String>,
            instrucciones: Option<String>,
            orden: i32,
            activo: bool,
            fecha_creacion: chrono::DateTime<chrono::Utc>,
        }

        let row = sqlx::query_as!(
            MetodoPagoRow,
            r#"
            SELECT
                id_metodo_pago,
                nombre as "nombre!",
                tipo::TEXT as "tipo!",
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

        let metodo = row.map(|row| MetodoPago {
            id_metodo_pago: row.id_metodo_pago,
            nombre: row.nombre,
            tipo: row.tipo,
            proveedor: row.proveedor,
            descripcion: row.descripcion,
            icono: row.icono,
            comision_porcentaje: Some(row.comision_porcentaje),
            comision_fija: Some(row.comision_fija),
            requiere_verificacion: Some(row.requiere_verificacion),
            tiempo_procesamiento: row.tiempo_procesamiento,
            instrucciones: row.instrucciones,
            orden: Some(row.orden),
            activo: Some(row.activo),
            fecha_creacion: Some(row.fecha_creacion.naive_utc()),
        });

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

        let row = sqlx::query_as!(
            VentaRow,
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
                descuento_total,
                costo_envio,
                total as "total!",
                moneda,
                estado::TEXT as "estado!",
                estado_pago::TEXT as "estado_pago!",
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
                fecha_actualizacion as "fecha_actualizacion: chrono::DateTime<chrono::Utc>"
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

        Ok(row.into())
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
                respuesta_proveedor,
                ultimos_4_digitos,
                marca_tarjeta,
                ip_cliente,
                user_agent,
                fecha_pago as "fecha_pago: chrono::DateTime<chrono::Utc>",
                fecha_creacion as "fecha_creacion!: chrono::DateTime<chrono::Utc>",
                fecha_actualizacion as "fecha_actualizacion: chrono::DateTime<chrono::Utc>",
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
            estado: Some(pago.estado),
            monto: pago.monto,
            moneda: Some(pago.moneda),
            comision: Some(pago.comision),
            monto_neto: pago.monto_neto,
            proveedor_pago: pago.proveedor_pago,
            id_transaccion_proveedor: pago.id_transaccion_proveedor,
            token_pago: pago.token_pago,
            respuesta_proveedor: pago.respuesta_proveedor,
            ultimos_4_digitos: pago.ultimos_4_digitos,
            marca_tarjeta: pago.marca_tarjeta,
            ip_cliente: pago.ip_cliente,
            user_agent: pago.user_agent,
            fecha_pago: pago.fecha_pago.map(|dt| dt.naive_utc()),
            fecha_creacion: Some(pago.fecha_creacion.naive_utc()),
            fecha_actualizacion: pago.fecha_actualizacion.map(|dt| dt.naive_utc()),
            nota_error: pago.nota_error,
            intentos_fallidos: Some(pago.intentos_fallidos),
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
        let row = sqlx::query_as!(
            VentaRow,
            r#"
            SELECT
                id_venta,
                numero_pedido as "numero_pedido!",
                id_usuario,
                id_carrito,
                subtotal as "subtotal!",
                descuento_total,
                costo_envio,
                total as "total!",
                moneda,
                estado::TEXT as "estado!",
                estado_pago::TEXT as "estado_pago!",
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
                fecha_actualizacion as "fecha_actualizacion: chrono::DateTime<chrono::Utc>"
            FROM venta
            WHERE id_venta = $1 AND id_usuario = $2
            "#,
            id_venta,
            id_usuario
        )
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| r.into()))
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
        let rows = sqlx::query_as!(
            VentaRow,
            r#"
            SELECT
                id_venta,
                numero_pedido as "numero_pedido!",
                id_usuario,
                id_carrito,
                subtotal as "subtotal!",
                descuento_total,
                costo_envio,
                total as "total!",
                moneda,
                estado::TEXT as "estado!",
                estado_pago::TEXT as "estado_pago!",
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
                fecha_actualizacion as "fecha_actualizacion: chrono::DateTime<chrono::Utc>"
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

        let ventas = rows.into_iter().map(|r| r.into()).collect();
        Ok(ventas)
    }

    /// Validar y obtener cupón por código
    pub async fn validar_cupon(
        pool: &PgPool,
        codigo: &str,
        id_usuario: i32,
        subtotal: Decimal,
    ) -> Result<Option<crate::models::Cupon>, String> {
        // Obtener cupón activo y dentro de fechas válidas
        let cupon_result = sqlx::query_as!(
            crate::models::Cupon,
            r#"
            SELECT
                id_cupon,
                codigo,
                nombre,
                descripcion,
                tipo_cupon::TEXT as "tipo_cupon!",
                valor as "valor!",
                aplica_a::TEXT as "aplica_cupon!",
                id_referencia,
                compra_minima,
                usos_maximos as usos_maximos_totales,
                usos_maximos_por_usuario,
                usos_actuales,
                solo_nuevos_usuarios,
                solo_primera_compra,
                fecha_inicio as "fecha_inicio!: chrono::NaiveDateTime",
                fecha_fin as "fecha_fin!: chrono::NaiveDateTime",
                activo,
                fecha_creacion as "fecha_creacion!: chrono::NaiveDateTime",
                fecha_actualizacion as "fecha_actualizacion?: chrono::NaiveDateTime"
            FROM cupon
            WHERE UPPER(codigo) = UPPER($1)
              AND activo = true
              AND CURRENT_TIMESTAMP BETWEEN fecha_inicio AND fecha_fin
            "#,
            codigo
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("Error al buscar cupón: {}", e))?;

        let cupon = match cupon_result {
            Some(c) => c,
            None => return Err("Cupón no válido, inactivo o expirado".to_string()),
        };

        // Validar compra mínima
        if let Some(compra_min) = cupon.compra_minima {
            if subtotal < compra_min {
                return Err(format!(
                    "El cupón requiere una compra mínima de S/. {}",
                    compra_min
                ));
            }
        }

        // Validar usos máximos totales
        if let Some(usos_max) = cupon.usos_maximos_totales {
            let usos_actuales = cupon.usos_actuales.unwrap_or(0);
            if usos_actuales >= usos_max {
                return Err("El cupón ha alcanzado su límite de usos".to_string());
            }
        }

        // Validar usos por usuario
        if let Some(usos_max_usuario) = cupon.usos_maximos_por_usuario {
            let usos_usuario = sqlx::query!(
                r#"
                SELECT COUNT(*) as "count!"
                FROM uso_cupon
                WHERE id_cupon = $1 AND id_usuario = $2
                "#,
                cupon.id_cupon,
                id_usuario
            )
            .fetch_one(pool)
            .await
            .map_err(|e| format!("Error al verificar usos del usuario: {}", e))?
            .count;

            if usos_usuario >= usos_max_usuario as i64 {
                return Err("Ya has utilizado este cupón el máximo de veces permitido".to_string());
            }
        }

        // Validar si es solo para nuevos usuarios
        if cupon.solo_nuevos_usuarios.unwrap_or(false) {
            let es_nuevo = sqlx::query!(
                r#"
                SELECT
                    EXTRACT(EPOCH FROM (CURRENT_TIMESTAMP - fecha_registro)) / 86400 as "dias!: rust_decimal::Decimal"
                FROM usuario
                WHERE id_usuario = $1
                "#,
                id_usuario
            )
            .fetch_one(pool)
            .await
            .map_err(|e| format!("Error al verificar antigüedad del usuario: {}", e))?;

            use std::str::FromStr;
            let siete_dias = Decimal::from_str("7.0").unwrap();
            if es_nuevo.dias > siete_dias {
                return Err("Este cupón es solo para nuevos usuarios (registrados hace menos de 7 días)".to_string());
            }
        }

        // Validar si es solo para primera compra
        if cupon.solo_primera_compra.unwrap_or(false) {
            let compras_previas = sqlx::query!(
                r#"
                SELECT COUNT(*) as "count!"
                FROM venta
                WHERE id_usuario = $1 AND estado::TEXT != 'cancelado'
                "#,
                id_usuario
            )
            .fetch_one(pool)
            .await
            .map_err(|e| format!("Error al verificar compras previas: {}", e))?
            .count;

            if compras_previas > 0 {
                return Err("Este cupón es solo para tu primera compra".to_string());
            }
        }

        Ok(Some(cupon))
    }

    /// Registrar uso de cupón
    pub async fn registrar_uso_cupon(
        tx: &mut Transaction<'_, Postgres>,
        id_cupon: i32,
        id_usuario: i32,
        id_venta: i32,
        descuento_aplicado: Decimal,
    ) -> Result<(), sqlx::Error> {
        // Crear registro de uso
        sqlx::query!(
            r#"
            INSERT INTO uso_cupon (
                id_cupon, id_usuario, id_venta, descuento_aplicado, fecha_uso
            )
            VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP)
            "#,
            id_cupon,
            id_usuario,
            id_venta,
            descuento_aplicado
        )
        .execute(&mut **tx)
        .await?;

        // Incrementar contador de usos del cupón
        sqlx::query!(
            r#"
            UPDATE cupon
            SET usos_actuales = COALESCE(usos_actuales, 0) + 1,
                fecha_actualizacion = CURRENT_TIMESTAMP
            WHERE id_cupon = $1
            "#,
            id_cupon
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}
