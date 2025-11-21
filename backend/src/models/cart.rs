use axum::http::{header, HeaderMap};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CartOwner {
    pub user_id: Option<i32>,
    pub session_id: Option<String>,
}

impl CartOwner {
    pub fn new(user_id: Option<i32>, session_id: Option<String>) -> Self {
        Self { user_id, session_id }
    }

    pub fn from_headers(headers: &HeaderMap) -> (Self, Option<String>) {
        let user_id = headers
            .get("x-user-id")
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse::<i32>().ok());

        let session_from_header = headers
            .get("x-session-id")
            .and_then(|value| value.to_str().ok())
            .map(|s| s.to_string());

        let session_from_cookie = extract_cookie_value(headers, "kronos_session");

        let mut new_session = None;

        let session_id = session_from_header
            .or(session_from_cookie)
            .or_else(|| {
                if user_id.is_none() {
                    let generated = Uuid::new_v4().to_string();
                    new_session = Some(generated.clone());
                    Some(generated)
                } else {
                    None
                }
            });

        (
            Self {
                user_id,
                session_id,
            },
            new_session,
        )
    }

    pub fn is_valid(&self) -> bool {
        self.user_id.is_some() || self.session_id.is_some()
    }
}

fn extract_cookie_value(headers: &HeaderMap, name: &str) -> Option<String> {
    let cookies = headers.get(header::COOKIE)?.to_str().ok()?;
    for pair in cookies.split(';') {
        let mut parts = pair.trim().splitn(2, '=');
        let key = parts.next()?.trim();
        if key.eq_ignore_ascii_case(name) {
            let value = parts.next().unwrap_or("").trim();
            if !value.is_empty() {
                return Some(value.to_string());
            }
        }
    }
    None
}

#[derive(Debug, Serialize)]
pub struct CartItemResponse {
    pub id_carrito_detalle: i32,
    pub id_producto_detalle: i32,
    pub nombre: String,
    pub sku: String,
    pub imagen_principal: Option<String>,
    pub precio_unitario: f64,
    pub cantidad: i32,
    pub stock_disponible: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct CartResponse {
    pub id_carrito: i32,
    pub items: Vec<CartItemResponse>,
}

#[derive(Debug, Deserialize)]
pub struct AddCartItemRequest {
    pub id_producto_detalle: i32,
    pub cantidad: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCartItemRequest {
    pub cantidad: i32,
}
