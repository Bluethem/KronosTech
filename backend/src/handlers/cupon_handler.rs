use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow};

use crate::models::cupon::{Cupon, CuponListItem, CuponStats};

#[derive(Debug, Deserialize)]
pub struct CuponQuery {
    pub search: Option<String>,
    pub tipo: Option<String>,
    pub estado: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ProductoDropdown {
    pub id_producto_detalle: i32,
    pub nombre: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateCuponRequest {
    pub codigo: String,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub tipo_cupon: String,
    pub valor: Decimal,
    pub aplica_cupon: String,
    pub id_referencia: Option<i32>,
    pub compra_minima: Option<Decimal>,
    pub usos_maximos_totales: Option<i32>,
    pub usos_maximos_por_usuario: Option<i32>,
    pub solo_nuevos_usuarios: Option<bool>,
    pub solo_primera_compra: Option<bool>,
    pub fecha_inicio: String,
    pub fecha_fin: String,
    pub activo: Option<bool>,
}

pub async fn get_cupones(
    State(pool): State<PgPool>,
    Query(query): Query<CuponQuery>,
) -> Result<Json<Vec<CuponListItem>>, StatusCode> {
    let mut sql = String::from(
        r#"
        SELECT 
            id_cupon,
            codigo,
            nombre,
            tipo_cupon::TEXT as tipo_cupon,
            valor,
            fecha_inicio,
            fecha_fin,
            usos_actuales,
            usos_maximos as usos_maximos_totales,
            activo
        FROM cupon
        WHERE 1=1
        "#,
    );

    if let Some(search) = &query.search {
        if !search.is_empty() {
            sql.push_str(&format!(" AND (codigo ILIKE '%{}%' OR nombre ILIKE '%{}%')", search, search));
        }
    }

    if let Some(tipo) = &query.tipo {
        if !tipo.is_empty() && tipo != "todos" {
            sql.push_str(&format!(" AND tipo_cupon::TEXT = '{}'", tipo));
        }
    }

    if let Some(estado) = &query.estado {
        match estado.as_str() {
            "activo" => sql.push_str(" AND activo = true"),
            "inactivo" => sql.push_str(" AND activo = false"),
            "vigente" => sql.push_str(" AND activo = true AND CURRENT_TIMESTAMP BETWEEN fecha_inicio AND fecha_fin"),
            "proximo" => sql.push_str(" AND activo = true AND CURRENT_TIMESTAMP < fecha_inicio"),
            "vencido" => sql.push_str(" AND activo = true AND CURRENT_TIMESTAMP > fecha_fin"),
            _ => {}
        }
    }

    sql.push_str(" ORDER BY id_cupon DESC");

    match sqlx::query_as::<_, CuponListItem>(&sql)
        .fetch_all(&pool)
        .await
    {
        Ok(cupones) => Ok(Json(cupones)),
        Err(e) => {
            eprintln!("Error fetching cupones: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_cupon_stats(
    State(pool): State<PgPool>,
) -> Result<Json<CuponStats>, StatusCode> {
    match sqlx::query_as::<_, CuponStats>(
        r#"
        SELECT 
            COUNT(*) FILTER (WHERE activo = true AND CURRENT_TIMESTAMP BETWEEN fecha_inicio AND fecha_fin) as total_activos,
            COUNT(*) FILTER (WHERE DATE(fecha_creacion) = CURRENT_DATE) as usos_hoy,
            SUM(valor) FILTER (WHERE DATE(fecha_creacion) >= DATE_TRUNC('month', CURRENT_DATE)) as descuento_mes
        FROM cupon
        "#
    )
    .fetch_one(&pool)
    .await
    {
        Ok(stats) => Ok(Json(stats)),
        Err(e) => {
            eprintln!("Error fetching cupon stats: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_cupon_detalle(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Cupon>, StatusCode> {
    match sqlx::query_as::<_, Cupon>(
        r#"
        SELECT 
            id_cupon,
            codigo,
            nombre,
            descripcion,
            tipo_cupon::TEXT as tipo_cupon,
            valor,
            aplica_a::TEXT as aplica_cupon,
            id_referencia,
            compra_minima,
            usos_maximos as usos_maximos_totales,
            usos_maximos_por_usuario,
            usos_actuales,
            solo_nuevos_usuarios,
            solo_primera_compra,
            fecha_inicio,
            fecha_fin,
            activo,
            fecha_creacion,
            fecha_actualizacion
        FROM cupon 
        WHERE id_cupon = $1
        "#
    )
    .bind(id)
    .fetch_one(&pool)
    .await
    {
        Ok(cupon) => Ok(Json(cupon)),
        Err(e) => {
            eprintln!("Error fetching cupon detalle: {:?}", e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

pub async fn create_cupon(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateCuponRequest>,
) -> Result<Json<Cupon>, StatusCode> {
    // Validate required fields
    if payload.codigo.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    if payload.nombre.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    if payload.valor <= Decimal::ZERO {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Validate tipo_cupon
    if !["porcentaje", "monto_fijo", "envio_gratis"].contains(&payload.tipo_cupon.as_str()) {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Validate aplica_cupon
    if !["todo", "producto", "categoria", "marca", "familia"].contains(&payload.aplica_cupon.as_str()) {
        return Err(StatusCode::BAD_REQUEST);
    }

    // If aplica_cupon is producto, id_referencia is required
    if payload.aplica_cupon == "producto" && payload.id_referencia.is_none() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Parse dates from string
    let fecha_inicio = NaiveDateTime::parse_from_str(&payload.fecha_inicio, "%Y-%m-%d %H:%M:%S")
        .map_err(|e| {
            eprintln!("Error parsing fecha_inicio: {:?}", e);
            StatusCode::BAD_REQUEST
        })?;
    
    let fecha_fin = NaiveDateTime::parse_from_str(&payload.fecha_fin, "%Y-%m-%d %H:%M:%S")
        .map_err(|e| {
            eprintln!("Error parsing fecha_fin: {:?}", e);
            StatusCode::BAD_REQUEST
        })?;

    // Validate dates
    if fecha_fin <= fecha_inicio {
        return Err(StatusCode::BAD_REQUEST);
    }

    match sqlx::query_as::<_, Cupon>(
        r#"
        INSERT INTO cupon (
            codigo, nombre, descripcion, tipo_cupon, valor, aplica_a, id_referencia,
            compra_minima, usos_maximos, usos_maximos_por_usuario, usos_actuales,
            solo_nuevos_usuarios, solo_primera_compra, fecha_inicio, fecha_fin, activo
        )
        VALUES ($1, $2, $3, CAST($4 AS tipo_cupon), $5, CAST($6 AS aplica_cupon), $7, $8, $9, $10, 0, $11, $12, $13, $14, $15)
        RETURNING 
            id_cupon,
            codigo,
            nombre,
            descripcion,
            tipo_cupon::TEXT as tipo_cupon,
            valor,
            aplica_a::TEXT as aplica_cupon,
            id_referencia,
            compra_minima,
            usos_maximos as usos_maximos_totales,
            usos_maximos_por_usuario,
            usos_actuales,
            solo_nuevos_usuarios,
            solo_primera_compra,
            fecha_inicio,
            fecha_fin,
            activo,
            fecha_creacion,
            fecha_actualizacion
        "#
    )
    .bind(&payload.codigo)
    .bind(&payload.nombre)
    .bind(&payload.descripcion)
    .bind(&payload.tipo_cupon)
    .bind(&payload.valor)
    .bind(&payload.aplica_cupon)
    .bind(&payload.id_referencia)
    .bind(&payload.compra_minima)
    .bind(&payload.usos_maximos_totales)
    .bind(&payload.usos_maximos_por_usuario)
    .bind(payload.solo_nuevos_usuarios.unwrap_or(false))
    .bind(payload.solo_primera_compra.unwrap_or(false))
    .bind(&fecha_inicio)
    .bind(&fecha_fin)
    .bind(payload.activo.unwrap_or(true))
    .fetch_one(&pool)
    .await
    {
        Ok(cupon) => Ok(Json(cupon)),
        Err(e) => {
            eprintln!("Error creating cupon: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update_cupon(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<CreateCuponRequest>,
) -> Result<Json<Cupon>, StatusCode> {
    // Same validation as create
    if payload.codigo.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    if payload.nombre.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    if payload.valor <= Decimal::ZERO {
        return Err(StatusCode::BAD_REQUEST);
    }

    if !["porcentaje", "monto_fijo", "envio_gratis"].contains(&payload.tipo_cupon.as_str()) {
        return Err(StatusCode::BAD_REQUEST);
    }

    if !["todo", "producto", "categoria", "marca", "familia"].contains(&payload.aplica_cupon.as_str()) {
        return Err(StatusCode::BAD_REQUEST);
    }

    if payload.aplica_cupon == "producto" && payload.id_referencia.is_none() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Parse dates
    let fecha_inicio = NaiveDateTime::parse_from_str(&payload.fecha_inicio, "%Y-%m-%d %H:%M:%S")
        .map_err(|e| {
            eprintln!("Error parsing fecha_inicio: {:?}", e);
            StatusCode::BAD_REQUEST
        })?;
    
    let fecha_fin = NaiveDateTime::parse_from_str(&payload.fecha_fin, "%Y-%m-%d %H:%M:%S")
        .map_err(|e| {
            eprintln!("Error parsing fecha_fin: {:?}", e);
            StatusCode::BAD_REQUEST
        })?;

    if fecha_fin <= fecha_inicio {
        return Err(StatusCode::BAD_REQUEST);
    }

    match sqlx::query_as::<_, Cupon>(
        r#"
        UPDATE cupon SET
            codigo = $1,
            nombre = $2,
            descripcion = $3,
            tipo_cupon = CAST($4 AS tipo_cupon),
            valor = $5,
            aplica_a = CAST($6 AS aplica_cupon),
            id_referencia = $7,
            compra_minima = $8,
            usos_maximos = $9,
            usos_maximos_por_usuario = $10,
            solo_nuevos_usuarios = $11,
            solo_primera_compra = $12,
            fecha_inicio = $13,
            fecha_fin = $14,
            activo = $15,
            fecha_actualizacion = CURRENT_TIMESTAMP
        WHERE id_cupon = $16
        RETURNING 
            id_cupon,
            codigo,
            nombre,
            descripcion,
            tipo_cupon::TEXT as tipo_cupon,
            valor,
            aplica_a::TEXT as aplica_cupon,
            id_referencia,
            compra_minima,
            usos_maximos as usos_maximos_totales,
            usos_maximos_por_usuario,
            usos_actuales,
            solo_nuevos_usuarios,
            solo_primera_compra,
            fecha_inicio,
            fecha_fin,
            activo,
            fecha_creacion,
            fecha_actualizacion
        "#
    )
    .bind(&payload.codigo)
    .bind(&payload.nombre)
    .bind(&payload.descripcion)
    .bind(&payload.tipo_cupon)
    .bind(&payload.valor)
    .bind(&payload.aplica_cupon)
    .bind(&payload.id_referencia)
    .bind(&payload.compra_minima)
    .bind(&payload.usos_maximos_totales)
    .bind(&payload.usos_maximos_por_usuario)
    .bind(payload.solo_nuevos_usuarios.unwrap_or(false))
    .bind(payload.solo_primera_compra.unwrap_or(false))
    .bind(&fecha_inicio)
    .bind(&fecha_fin)
    .bind(payload.activo.unwrap_or(true))
    .bind(id)
    .fetch_one(&pool)
    .await
    {
        Ok(cupon) => Ok(Json(cupon)),
        Err(e) => {
            eprintln!("Error updating cupon: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete_cupon(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    match sqlx::query("DELETE FROM cupon WHERE id_cupon = $1")
        .bind(id)
        .execute(&pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                Ok(StatusCode::NO_CONTENT)
            } else {
                Err(StatusCode::NOT_FOUND)
            }
        }
        Err(e) => {
            eprintln!("Error deleting cupon: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_productos_dropdown(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<ProductoDropdown>>, StatusCode> {
    match sqlx::query_as::<_, ProductoDropdown>(
        r#"
        SELECT id_producto_detalle, nombre
        FROM producto_detalle
        ORDER BY nombre ASC
        "#
    )
    .fetch_all(&pool)
    .await
    {
        Ok(productos) => Ok(Json(productos)),
        Err(e) => {
            eprintln!("Error fetching productos dropdown: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Structures for mass assignment
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Usuario {
    pub id_usuario: i32,
    pub nombre: String,
    pub email: String,
    pub telefono: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AssignCuponRequest {
    pub id_cupon: i32,
    pub usuarios: Vec<i32>, // List of user IDs
}

// Get users for assignment
pub async fn get_usuarios(
    State(pool): State<PgPool>,
    Query(query): Query<CuponQuery>,
) -> Result<Json<Vec<Usuario>>, StatusCode> {
    let mut sql = String::from(
        r#"
        SELECT 
            id_usuario,
            nombre,
            email,
            telefono
        FROM usuario
        WHERE 1=1
        "#,
    );

    if let Some(search) = &query.search {
        if !search.is_empty() {
            sql.push_str(&format!(" AND (nombre ILIKE '%{}%' OR email ILIKE '%{}%')", search, search));
        }
    }

    sql.push_str(" ORDER BY nombre ASC LIMIT 100");

    match sqlx::query_as::<_, Usuario>(&sql)
        .fetch_all(&pool)
        .await
    {
        Ok(usuarios) => Ok(Json(usuarios)),
        Err(e) => {
            eprintln!("Error fetching usuarios: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Assign coupon to multiple users
pub async fn assign_cupon_to_users(
    State(pool): State<PgPool>,
    Json(payload): Json<AssignCuponRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    if payload.usuarios.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut assigned_count = 0;
    let mut failed_count = 0;

    for id_usuario in &payload.usuarios {
        // Check if assignment already exists
        let exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM asignacion_cupon WHERE id_cupon = $1 AND id_usuario = $2)"
        )
        .bind(payload.id_cupon)
        .bind(id_usuario)
        .fetch_one(&pool)
        .await;

        match exists {
            Ok(true) => {
                // Already assigned, skip
                continue;
            }
            Ok(false) => {
                // Insert new assignment
                let result = sqlx::query(
                    r#"
                    INSERT INTO asignacion_cupon (id_cupon, id_usuario, fecha_asignacion, activo)
                    VALUES ($1, $2, CURRENT_TIMESTAMP, true)
                    "#
                )
                .bind(payload.id_cupon)
                .bind(id_usuario)
                .execute(&pool)
                .await;

                match result {
                    Ok(_) => assigned_count += 1,
                    Err(e) => {
                        eprintln!("Error assigning cupon to user {}: {:?}", id_usuario, e);
                        failed_count += 1;
                    }
                }
            }
            Err(e) => {
                eprintln!("Error checking assignment for user {}: {:?}", id_usuario, e);
                failed_count += 1;
            }
        }
    }

    Ok(Json(serde_json::json!({
        "success": true,
        "assigned": assigned_count,
        "failed": failed_count,
        "total": payload.usuarios.len()
    })))
}

// Get users assigned to a coupon
pub async fn get_assigned_users(
    State(pool): State<PgPool>,
    Path(id_cupon): Path<i32>,
) -> Result<Json<Vec<Usuario>>, StatusCode> {
    match sqlx::query_as::<_, Usuario>(
        r#"
        SELECT 
            u.id_usuario,
            u.nombre,
            u.email,
            u.telefono
        FROM usuario u
        INNER JOIN asignacion_cupon ac ON u.id_usuario = ac.id_usuario
        WHERE ac.id_cupon = $1 AND ac.activo = true
        ORDER BY u.nombre ASC
        "#
    )
    .bind(id_cupon)
    .fetch_all(&pool)
    .await
    {
        Ok(usuarios) => Ok(Json(usuarios)),
        Err(e) => {
            eprintln!("Error fetching assigned users: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
