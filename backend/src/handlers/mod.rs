// Handlers - Catálogo
pub mod catalogo_handler;

// Handlers - Autenticación y Carrito (H3nr7)
pub mod auth_handler;
pub mod carrito_handler;
pub mod direccion_handler;
pub mod checkout_handler;

// Handlers - Administración (main)
pub mod venta;
pub mod inventario_handler;
pub mod producto_handler;
pub mod descuento_handler;
pub mod cupon_handler;
pub mod reembolso_handler;

// Re-exportaciones para uso en routes - Catálogo
pub use catalogo_handler::*;

// Re-exportaciones para uso en routes - Auth & Carrito
pub use auth_handler::{register_handler, login_handler, get_current_user_handler, logout_handler};
pub use carrito_handler::{
    get_carrito_handler,
    agregar_item_handler,
    actualizar_cantidad_handler,
    eliminar_item_handler,
    limpiar_carrito_handler,
};
pub use direccion_handler::{
    get_direcciones_handler,
    crear_direccion_handler,
    actualizar_direccion_handler,
    eliminar_direccion_handler,
};
pub use checkout_handler::{
    get_metodos_pago_handler,
    calcular_total_handler,
    procesar_checkout_handler,
    get_pedidos_handler,
    get_pedido_handler,
};
