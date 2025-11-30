use sqlx::PgPool;
use crate::models::{Direccion, CrearDireccionRequest, ActualizarDireccionRequest};

pub struct DireccionRepository;

impl DireccionRepository {
    /// Obtener todas las direcciones de un usuario
    pub async fn get_direcciones_usuario(
        pool: &PgPool,
        id_usuario: i32,
    ) -> Result<Vec<Direccion>, sqlx::Error> {
        let direcciones = sqlx::query_as!(
            Direccion,
            r#"
            SELECT
                id_direccion,
                id_usuario,
                tipo as "tipo!: _",
                nombre_completo,
                direccion_linea1 as "direccion_linea1!",
                direccion_linea2,
                ciudad as "ciudad!",
                departamento as "departamento!",
                codigo_postal,
                pais as "pais!",
                telefono_contacto,
                referencia,
                es_predeterminada as "es_predeterminada!",
                activo as "activo!",
                fecha_creacion as "fecha_creacion!: chrono::DateTime<chrono::Utc>"
            FROM direccion
            WHERE id_usuario = $1 AND activo = TRUE
            ORDER BY es_predeterminada DESC, fecha_creacion DESC
            "#,
            id_usuario
        )
        .fetch_all(pool)
        .await?;

        Ok(direcciones)
    }

    /// Obtener dirección por ID
    pub async fn get_direccion_by_id(
        pool: &PgPool,
        id_direccion: i32,
        id_usuario: i32,
    ) -> Result<Option<Direccion>, sqlx::Error> {
        let direccion = sqlx::query_as!(
            Direccion,
            r#"
            SELECT
                id_direccion,
                id_usuario,
                tipo as "tipo!: _",
                nombre_completo,
                direccion_linea1 as "direccion_linea1!",
                direccion_linea2,
                ciudad as "ciudad!",
                departamento as "departamento!",
                codigo_postal,
                pais as "pais!",
                telefono_contacto,
                referencia,
                es_predeterminada as "es_predeterminada!",
                activo as "activo!",
                fecha_creacion as "fecha_creacion!: chrono::DateTime<chrono::Utc>"
            FROM direccion
            WHERE id_direccion = $1 AND id_usuario = $2
            "#,
            id_direccion,
            id_usuario
        )
        .fetch_optional(pool)
        .await?;

        Ok(direccion)
    }

    /// Crear nueva dirección
    pub async fn crear_direccion(
        pool: &PgPool,
        id_usuario: i32,
        request: &CrearDireccionRequest,
    ) -> Result<Direccion, sqlx::Error> {
        // Si esta dirección es predeterminada, remover predeterminada de otras
        if request.es_predeterminada.unwrap_or(false) {
            Self::remover_predeterminada(pool, id_usuario).await?;
        }

        let pais = request.pais.clone().unwrap_or_else(|| "Perú".to_string());
        let tipo = request.tipo.as_str();

        let direccion = sqlx::query_as!(
            Direccion,
            r#"
            INSERT INTO direccion (
                id_usuario, tipo, nombre_completo, direccion_linea1, direccion_linea2,
                ciudad, departamento, codigo_postal, pais, telefono_contacto,
                referencia, es_predeterminada, activo
            )
            VALUES ($1, $2::TEXT::tipo_direccion, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, TRUE)
            RETURNING
                id_direccion,
                id_usuario,
                tipo as "tipo!: _",
                nombre_completo,
                direccion_linea1 as "direccion_linea1!",
                direccion_linea2,
                ciudad as "ciudad!",
                departamento as "departamento!",
                codigo_postal,
                pais as "pais!",
                telefono_contacto,
                referencia,
                es_predeterminada as "es_predeterminada!",
                activo as "activo!",
                fecha_creacion as "fecha_creacion!: chrono::DateTime<chrono::Utc>"
            "#,
            id_usuario,
            tipo,
            request.nombre_completo,
            request.direccion_linea1,
            request.direccion_linea2,
            request.ciudad,
            request.departamento,
            request.codigo_postal,
            pais,
            request.telefono_contacto,
            request.referencia,
            request.es_predeterminada.unwrap_or(false)
        )
        .fetch_one(pool)
        .await?;

        Ok(direccion)
    }

    /// Actualizar dirección
    pub async fn actualizar_direccion(
        pool: &PgPool,
        id_direccion: i32,
        id_usuario: i32,
        request: &ActualizarDireccionRequest,
    ) -> Result<Direccion, sqlx::Error> {
        // Si se marca como predeterminada, remover de otras
        if request.es_predeterminada == Some(true) {
            Self::remover_predeterminada(pool, id_usuario).await?;
        }

        // Construir query dinámica basada en campos presentes
        let direccion = sqlx::query_as!(
            Direccion,
            r#"
            UPDATE direccion
            SET
                tipo = COALESCE($3::TEXT::tipo_direccion, tipo),
                nombre_completo = COALESCE($4, nombre_completo),
                direccion_linea1 = COALESCE($5, direccion_linea1),
                direccion_linea2 = COALESCE($6, direccion_linea2),
                ciudad = COALESCE($7, ciudad),
                departamento = COALESCE($8, departamento),
                codigo_postal = COALESCE($9, codigo_postal),
                pais = COALESCE($10, pais),
                telefono_contacto = COALESCE($11, telefono_contacto),
                referencia = COALESCE($12, referencia),
                es_predeterminada = COALESCE($13, es_predeterminada),
                activo = COALESCE($14, activo)
            WHERE id_direccion = $1 AND id_usuario = $2
            RETURNING
                id_direccion,
                id_usuario,
                tipo as "tipo!: _",
                nombre_completo,
                direccion_linea1 as "direccion_linea1!",
                direccion_linea2,
                ciudad as "ciudad!",
                departamento as "departamento!",
                codigo_postal,
                pais as "pais!",
                telefono_contacto,
                referencia,
                es_predeterminada as "es_predeterminada!",
                activo as "activo!",
                fecha_creacion as "fecha_creacion!: chrono::DateTime<chrono::Utc>"
            "#,
            id_direccion,
            id_usuario,
            request.tipo.as_deref(),
            request.nombre_completo,
            request.direccion_linea1,
            request.direccion_linea2,
            request.ciudad,
            request.departamento,
            request.codigo_postal,
            request.pais,
            request.telefono_contacto,
            request.referencia,
            request.es_predeterminada,
            request.activo
        )
        .fetch_one(pool)
        .await?;

        Ok(direccion)
    }

    /// Eliminar dirección (soft delete)
    pub async fn eliminar_direccion(
        pool: &PgPool,
        id_direccion: i32,
        id_usuario: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE direccion
            SET activo = FALSE
            WHERE id_direccion = $1 AND id_usuario = $2
            "#,
            id_direccion,
            id_usuario
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Remover predeterminada de todas las direcciones del usuario
    async fn remover_predeterminada(
        pool: &PgPool,
        id_usuario: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE direccion
            SET es_predeterminada = FALSE
            WHERE id_usuario = $1 AND es_predeterminada = TRUE
            "#,
            id_usuario
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
