use sqlx::PgPool;
use crate::models::Usuario;

pub struct AuthRepository;

impl AuthRepository {
    // Buscar usuario por email
    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<Usuario>, sqlx::Error> {
        let usuario = sqlx::query_as!(
            Usuario,
            r#"
            SELECT
                id_usuario,
                nombre,
                apellido,
                email,
                contrasena,
                telefono,
                dni,
                rol::TEXT as "rol!",
                email_verificado as "email_verificado!",
                token_verificacion,
                activo as "activo!",
                fecha_registro as "fecha_registro!: chrono::DateTime<chrono::Utc>",
                ultima_conexion as "ultima_conexion: chrono::DateTime<chrono::Utc>",
                fecha_actualizacion as "fecha_actualizacion!: chrono::DateTime<chrono::Utc>"
            FROM usuario
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(pool)
        .await?;

        Ok(usuario)
    }

    // Crear nuevo usuario
    pub async fn create_user(
        pool: &PgPool,
        nombre: &str,
        apellido: &str,
        email: &str,
        password_hash: &str,
        telefono: Option<&str>,
        dni: Option<&str>,
    ) -> Result<Usuario, sqlx::Error> {
        let usuario = sqlx::query_as!(
            Usuario,
            r#"
            INSERT INTO usuario (nombre, apellido, email, contrasena, telefono, dni, rol)
            VALUES ($1, $2, $3, $4, $5, $6, 'cliente')
            RETURNING
                id_usuario,
                nombre,
                apellido,
                email,
                contrasena,
                telefono,
                dni,
                rol::TEXT as "rol!",
                email_verificado as "email_verificado!",
                token_verificacion,
                activo as "activo!",
                fecha_registro as "fecha_registro!: chrono::DateTime<chrono::Utc>",
                ultima_conexion as "ultima_conexion: chrono::DateTime<chrono::Utc>",
                fecha_actualizacion as "fecha_actualizacion!: chrono::DateTime<chrono::Utc>"
            "#,
            nombre,
            apellido,
            email,
            password_hash,
            telefono,
            dni
        )
        .fetch_one(pool)
        .await?;

        Ok(usuario)
    }

    // Actualizar última conexión
    pub async fn update_last_login(pool: &PgPool, user_id: i32) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE usuario
            SET ultima_conexion = CURRENT_TIMESTAMP
            WHERE id_usuario = $1
            "#,
            user_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    // Buscar usuario por ID
    pub async fn find_by_id(pool: &PgPool, user_id: i32) -> Result<Option<Usuario>, sqlx::Error> {
        let usuario = sqlx::query_as!(
            Usuario,
            r#"
            SELECT
                id_usuario,
                nombre,
                apellido,
                email,
                contrasena,
                telefono,
                dni,
                rol::TEXT as "rol!",
                email_verificado as "email_verificado!",
                token_verificacion,
                activo as "activo!",
                fecha_registro as "fecha_registro!: chrono::DateTime<chrono::Utc>",
                ultima_conexion as "ultima_conexion: chrono::DateTime<chrono::Utc>",
                fecha_actualizacion as "fecha_actualizacion!: chrono::DateTime<chrono::Utc>"
            FROM usuario
            WHERE id_usuario = $1
            "#,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(usuario)
    }

    // Verificar si el email ya existe
    pub async fn email_exists(pool: &PgPool, email: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            SELECT EXISTS(SELECT 1 FROM usuario WHERE email = $1) as "exists!"
            "#,
            email
        )
        .fetch_one(pool)
        .await?;

        Ok(result.exists)
    }
}
