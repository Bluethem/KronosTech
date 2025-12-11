use axum::{
    extract::{State, Path, Query},
    http::{StatusCode, HeaderMap},
    Json,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow};
use chrono::NaiveDateTime;

use crate::services::AuthService;

// ==================== RESPONSES ====================

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct UsuarioAdmin {
    pub id_usuario: i32,
    pub nombre: String,
    pub apellido: String,
    pub email: String,
    pub rol: String,
    pub activo: bool,
    pub email_verificado: bool,
    pub telefono: Option<String>,
    pub dni: Option<String>,
    pub fecha_registro: NaiveDateTime,
    pub ultima_conexion: Option<NaiveDateTime>,
}

// ==================== REQUESTS ====================

#[derive(Debug, Deserialize)]
pub struct ListarUsuariosQuery {
    pub rol: Option<String>,
    pub activo: Option<bool>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct ActualizarUsuarioAdminRequest {
    pub nombre: Option<String>,
    pub apellido: Option<String>,
    pub rol: Option<String>,
    pub activo: Option<bool>,
    pub email_verificado: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CrearAdministradorRequest {
    pub nombre: String,
    pub apellido: String,
    pub email: String,
    pub password: String,
    pub rol: String, // "administrador" o "super_admin"
}

// ==================== HELPER FUNCTIONS ====================

/// Verificar si el usuario es super_admin
fn verify_super_admin(token: &str) -> Result<i32, (StatusCode, Json<ErrorResponse>)> {
    let claims = AuthService::verify_token(token).map_err(|e| {
        (
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                success: false,
                message: e,
            }),
        )
    })?;

    if claims.rol != "super_admin" {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse {
                success: false,
                message: "Acceso denegado. Solo super_admin puede acceder a este recurso".to_string(),
            }),
        ));
    }

    Ok(claims.sub)
}

/// Extraer token del header
fn extract_token(headers: &HeaderMap) -> Result<&str, (StatusCode, Json<ErrorResponse>)> {
    headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| {
            if value.starts_with("Bearer ") {
                Some(&value[7..])
            } else {
                None
            }
        })
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    success: false,
                    message: "Token no proporcionado".to_string(),
                }),
            )
        })
}

// ==================== HANDLERS ====================

/// GET /api/admin/usuarios
/// Listar todos los usuarios (solo super_admin)
pub async fn listar_usuarios_handler(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Query(query): Query<ListarUsuariosQuery>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let token = extract_token(&headers)?;
    verify_super_admin(token)?;

    let limit = query.limit.unwrap_or(50).min(100);
    let offset = query.offset.unwrap_or(0);

    let mut sql = String::from(
        "SELECT id_usuario, nombre, apellido, email, rol::TEXT as rol, activo, email_verificado,
         telefono, dni, fecha_registro, ultima_conexion
         FROM usuario WHERE 1=1"
    );

    if let Some(rol) = &query.rol {
        sql.push_str(&format!(" AND rol = '{}'", rol));
    }

    if let Some(activo) = query.activo {
        sql.push_str(&format!(" AND activo = {}", activo));
    }

    sql.push_str(" ORDER BY fecha_registro DESC");
    sql.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));

    match sqlx::query_as::<_, UsuarioAdmin>(&sql)
        .fetch_all(&pool)
        .await
    {
        Ok(usuarios) => Ok((
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: Some(usuarios),
                message: None,
            }),
        )),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                success: false,
                message: format!("Error al listar usuarios: {}", err),
            }),
        )),
    }
}

/// PUT /api/admin/usuarios/:id
/// Actualizar usuario (solo super_admin)
pub async fn actualizar_usuario_admin_handler(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Path(id_usuario): Path<i32>,
    Json(payload): Json<ActualizarUsuarioAdminRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let token = extract_token(&headers)?;
    let admin_id = verify_super_admin(token)?;

    // No permitir que el super_admin se desactive a sí mismo
    if id_usuario == admin_id && payload.activo == Some(false) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                message: "No puedes desactivar tu propia cuenta".to_string(),
            }),
        ));
    }

    // Construir query dinámicamente
    let mut updates = Vec::new();

    if let Some(nombre) = &payload.nombre {
        updates.push(format!("nombre = '{}'", nombre));
    }

    if let Some(apellido) = &payload.apellido {
        updates.push(format!("apellido = '{}'", apellido));
    }

    if let Some(rol) = &payload.rol {
        // Validar rol
        if !["cliente", "administrador", "super_admin"].contains(&rol.as_str()) {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    success: false,
                    message: "Rol inválido".to_string(),
                }),
            ));
        }
        updates.push(format!("rol = '{}'::rol_usuario", rol));
    }

    if let Some(activo) = payload.activo {
        updates.push(format!("activo = {}", activo));
    }

    if let Some(email_verificado) = payload.email_verificado {
        updates.push(format!("email_verificado = {}", email_verificado));
    }

    if updates.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                message: "No se proporcionaron campos para actualizar".to_string(),
            }),
        ));
    }

    let sql = format!(
        "UPDATE usuario SET {} WHERE id_usuario = {} RETURNING id_usuario, nombre, apellido, email, rol::TEXT as rol, activo, email_verificado, telefono, dni, fecha_registro, ultima_conexion",
        updates.join(", "),
        id_usuario
    );

    match sqlx::query_as::<_, UsuarioAdmin>(&sql)
        .fetch_one(&pool)
        .await
    {
        Ok(usuario) => Ok((
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: Some(usuario),
                message: Some("Usuario actualizado exitosamente".to_string()),
            }),
        )),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                success: false,
                message: format!("Error al actualizar usuario: {}", err),
            }),
        )),
    }
}

/// POST /api/admin/administradores
/// Crear nuevo administrador (solo super_admin)
pub async fn crear_administrador_handler(
    State(pool): State<PgPool>,
    headers: HeaderMap,
    Json(payload): Json<CrearAdministradorRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let token = extract_token(&headers)?;
    verify_super_admin(token)?;

    // Validar rol
    if !["administrador", "super_admin"].contains(&payload.rol.as_str()) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                message: "Rol inválido. Debe ser 'administrador' o 'super_admin'".to_string(),
            }),
        ));
    }

    // Validar email
    if !payload.email.contains('@') || !payload.email.contains('.') {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                message: "Email inválido".to_string(),
            }),
        ));
    }

    // Validar contraseña
    if payload.password.len() < 6 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                message: "La contraseña debe tener al menos 6 caracteres".to_string(),
            }),
        ));
    }

    // Verificar si el email ya existe
    let email_exists: Option<(i32,)> = sqlx::query_as("SELECT id_usuario FROM usuario WHERE email = $1")
        .bind(&payload.email)
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    success: false,
                    message: format!("Error al verificar email: {}", e),
                }),
            )
        })?;

    if email_exists.is_some() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                message: "El email ya está registrado".to_string(),
            }),
        ));
    }

    // Hash de la contraseña
    let password_hash = bcrypt::hash(&payload.password, bcrypt::DEFAULT_COST)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    success: false,
                    message: format!("Error al encriptar contraseña: {}", e),
                }),
            )
        })?;

    // Crear usuario
    match sqlx::query_as::<_, UsuarioAdmin>(
        "INSERT INTO usuario (nombre, apellido, email, contrasena, rol, activo, email_verificado)
         VALUES ($1, $2, $3, $4, $5::rol_usuario, true, true)
         RETURNING id_usuario, nombre, apellido, email, rol::TEXT as rol, activo, email_verificado, telefono, dni, fecha_registro, ultima_conexion"
    )
    .bind(&payload.nombre)
    .bind(&payload.apellido)
    .bind(&payload.email)
    .bind(&password_hash)
    .bind(&payload.rol)
    .fetch_one(&pool)
    .await
    {
        Ok(usuario) => Ok((
            StatusCode::CREATED,
            Json(ApiResponse {
                success: true,
                data: Some(usuario),
                message: Some("Administrador creado exitosamente".to_string()),
            }),
        )),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                success: false,
                message: format!("Error al crear administrador: {}", err),
            }),
        )),
    }
}
