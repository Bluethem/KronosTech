use sqlx::PgPool;
use crate::models::MetodoPagoCliente;

pub struct MetodoPagoClienteRepository;

impl MetodoPagoClienteRepository {
    /// Obtener todos los métodos de pago del cliente
    pub async fn get_by_user(pool: &PgPool, id_usuario: i32) -> Result<Vec<MetodoPagoCliente>, sqlx::Error> {
        sqlx::query_as::<_, MetodoPagoCliente>(
            r#"
            SELECT
                id_metodo_pago_cliente,
                id_usuario,
                id_metodo_pago,
                tipo::text as tipo,
                token_pago,
                ultimos_4_digitos,
                marca,
                fecha_expiracion,
                nombre_titular,
                es_predeterminado,
                activo,
                fecha_creacion,
                fecha_actualizacion
            FROM metodo_pago_cliente
            WHERE id_usuario = $1 AND activo = true
            ORDER BY es_predeterminado DESC, fecha_creacion DESC
            "#
        )
        .bind(id_usuario)
        .fetch_all(pool)
        .await
    }

    /// Crear un nuevo método de pago
    pub async fn create(
        pool: &PgPool,
        id_usuario: i32,
        id_metodo_pago: i32,
        tipo: &str,
        token_pago: Option<&str>,
        ultimos_4_digitos: Option<&str>,
        marca: Option<&str>,
        fecha_expiracion: Option<&str>,
        nombre_titular: Option<&str>,
        es_predeterminado: bool,
    ) -> Result<MetodoPagoCliente, sqlx::Error> {
        // Si es predeterminado, desmarcar los demás
        if es_predeterminado {
            sqlx::query(
                "UPDATE metodo_pago_cliente SET es_predeterminado = false WHERE id_usuario = $1"
            )
            .bind(id_usuario)
            .execute(pool)
            .await?;
        }

        sqlx::query_as::<_, MetodoPagoCliente>(
            r#"
            INSERT INTO metodo_pago_cliente (
                id_usuario,
                id_metodo_pago,
                tipo,
                token_pago,
                ultimos_4_digitos,
                marca,
                fecha_expiracion,
                nombre_titular,
                es_predeterminado,
                activo
            )
            VALUES ($1, $2, $3::tipo_metodo_pago, $4, $5, $6, $7, $8, $9, true)
            RETURNING
                id_metodo_pago_cliente,
                id_usuario,
                id_metodo_pago,
                tipo::text as tipo,
                token_pago,
                ultimos_4_digitos,
                marca,
                fecha_expiracion,
                nombre_titular,
                es_predeterminado,
                activo,
                fecha_creacion,
                fecha_actualizacion
            "#
        )
        .bind(id_usuario)
        .bind(id_metodo_pago)
        .bind(tipo)
        .bind(token_pago)
        .bind(ultimos_4_digitos)
        .bind(marca)
        .bind(fecha_expiracion)
        .bind(nombre_titular)
        .bind(es_predeterminado)
        .fetch_one(pool)
        .await
    }

    /// Actualizar un método de pago
    pub async fn update(
        pool: &PgPool,
        id_metodo_pago_cliente: i32,
        id_usuario: i32,
        ultimos_4_digitos: Option<&str>,
        marca: Option<&str>,
        fecha_expiracion: Option<&str>,
        nombre_titular: Option<&str>,
        es_predeterminado: Option<bool>,
    ) -> Result<MetodoPagoCliente, sqlx::Error> {
        // Si es predeterminado, desmarcar los demás
        if let Some(true) = es_predeterminado {
            sqlx::query(
                "UPDATE metodo_pago_cliente SET es_predeterminado = false WHERE id_usuario = $1"
            )
            .bind(id_usuario)
            .execute(pool)
            .await?;
        }

        sqlx::query_as::<_, MetodoPagoCliente>(
            r#"
            UPDATE metodo_pago_cliente
            SET
                ultimos_4_digitos = COALESCE($1, ultimos_4_digitos),
                marca = COALESCE($2, marca),
                fecha_expiracion = COALESCE($3, fecha_expiracion),
                nombre_titular = COALESCE($4, nombre_titular),
                es_predeterminado = COALESCE($5, es_predeterminado),
                fecha_actualizacion = CURRENT_TIMESTAMP
            WHERE id_metodo_pago_cliente = $6 AND id_usuario = $7
            RETURNING
                id_metodo_pago_cliente,
                id_usuario,
                id_metodo_pago,
                tipo::text as tipo,
                token_pago,
                ultimos_4_digitos,
                marca,
                fecha_expiracion,
                nombre_titular,
                es_predeterminado,
                activo,
                fecha_creacion,
                fecha_actualizacion
            "#
        )
        .bind(ultimos_4_digitos)
        .bind(marca)
        .bind(fecha_expiracion)
        .bind(nombre_titular)
        .bind(es_predeterminado)
        .bind(id_metodo_pago_cliente)
        .bind(id_usuario)
        .fetch_one(pool)
        .await
    }

    /// Eliminar (soft delete) un método de pago
    pub async fn delete(
        pool: &PgPool,
        id_metodo_pago_cliente: i32,
        id_usuario: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE metodo_pago_cliente
            SET activo = false, fecha_actualizacion = CURRENT_TIMESTAMP
            WHERE id_metodo_pago_cliente = $1 AND id_usuario = $2
            "#
        )
        .bind(id_metodo_pago_cliente)
        .bind(id_usuario)
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Verificar si un método de pago pertenece a un usuario
    pub async fn belongs_to_user(
        pool: &PgPool,
        id_metodo_pago_cliente: i32,
        id_usuario: i32,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM metodo_pago_cliente WHERE id_metodo_pago_cliente = $1 AND id_usuario = $2 AND activo = true)"
        )
        .bind(id_metodo_pago_cliente)
        .bind(id_usuario)
        .fetch_one(pool)
        .await?;

        Ok(result)
    }
}
