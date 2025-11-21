use sqlx::PgPool;

use crate::models::usuario::{RegisterRequest, Usuario};

pub struct UsuarioRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> UsuarioRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_email(&self, email: &str) -> sqlx::Result<Option<Usuario>> {
        let usuario = sqlx::query_as::<_, Usuario>(
            r#"
            SELECT
                id_usuario,
                nombre,
                apellido,
                email,
                contrasena,
                telefono,
                dni,
                rol,
                email_verificado,
                activo,
                fecha_registro,
                ultima_conexion,
                fecha_actualizacion
            FROM usuario
            WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(self.pool)
        .await?;

        Ok(usuario)
    }

    pub async fn create(&self, data: &RegisterRequest, hash: &str) -> sqlx::Result<Usuario> {
        sqlx::query_as::<_, Usuario>(
            r#"
            INSERT INTO usuario (
                nombre,
                apellido,
                email,
                contrasena,
                telefono,
                dni,
                rol,
                email_verificado,
                activo
            )
            VALUES ($1, $2, $3, $4, $5, $6, 'cliente', FALSE, TRUE)
            RETURNING
                id_usuario,
                nombre,
                apellido,
                email,
                contrasena,
                telefono,
                dni,
                rol,
                email_verificado,
                activo,
                fecha_registro,
                ultima_conexion,
                fecha_actualizacion
            "#,
        )
        .bind(&data.nombre)
        .bind(&data.apellido)
        .bind(&data.email)
        .bind(hash)
        .bind(&data.telefono)
        .bind(&data.dni)
        .fetch_one(self.pool)
        .await
    }

    pub async fn actualizar_ultima_conexion(&self, id_usuario: i32) -> sqlx::Result<()> {
        sqlx::query(
            r#"
            UPDATE usuario
            SET ultima_conexion = NOW()
            WHERE id_usuario = $1
            "#,
        )
        .bind(id_usuario)
        .execute(self.pool)
        .await?
        .rows_affected();

        Ok(())
    }
}
