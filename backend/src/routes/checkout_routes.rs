use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

use crate::handlers::{
    get_metodos_pago_handler,
    calcular_total_handler,
    procesar_checkout_handler,
    get_pedidos_handler,
    get_pedido_handler,
};

pub fn checkout_routes(pool: PgPool) -> Router {
    Router::new()
        // Métodos de pago
        .route("/metodos-pago", get(get_metodos_pago_handler))
        // Checkout
        .route("/checkout/calcular-total", get(calcular_total_handler))
        .route("/checkout/procesar", post(procesar_checkout_handler))
        // Pedidos
        .route("/pedidos", get(get_pedidos_handler))
        .route("/pedidos/{id}", get(get_pedido_handler))
        .with_state(pool)
}
