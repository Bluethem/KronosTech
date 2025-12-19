// Módulos de rutas - Catálogo
pub mod catalogo_routes;

// Módulos de rutas - Autenticación y Carrito (H3nr7)
pub mod auth_routes;
pub mod carrito_routes;
pub mod direccion_routes;
pub mod checkout_routes;
pub mod metodo_pago_cliente_routes;
pub mod admin_routes;

// Módulos de rutas - Administración (main)
pub mod venta_routes;
pub mod inventario_routes;
pub mod producto_routes;
pub mod descuento_routes;
pub mod cupon_routes;
pub mod reembolso_routes;
pub mod log_routes;
pub mod config_routes;

// Re-exportaciones - Catálogo
pub use catalogo_routes::*;

// Re-exportaciones - Auth & Carrito
pub use auth_routes::auth_routes;
pub use carrito_routes::carrito_routes;
pub use direccion_routes::direccion_routes;
pub use checkout_routes::checkout_routes;
pub use metodo_pago_cliente_routes::metodo_pago_cliente_routes;
pub use admin_routes::admin_routes;

// Re-exportaciones - Administración
pub use venta_routes::*;
pub use inventario_routes::*;
pub use producto_routes::*;
pub use descuento_routes::*;
pub use cupon_routes::*;
pub use reembolso_routes::*;
pub use log_routes::log_routes;
pub use config_routes::config_routes;
