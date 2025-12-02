// Módulos de rutas - Catálogo
pub mod catalogo_routes;

// Módulos de rutas - Autenticación y Carrito (H3nr7)
pub mod auth_routes;
pub mod carrito_routes;
pub mod direccion_routes;
pub mod checkout_routes;

// Módulos de rutas - Administración (main)
pub mod venta_routes;
pub mod inventario_routes;
pub mod producto_routes;
pub mod descuento_routes;

// Re-exportaciones
pub use catalogo_routes::catalogo_routes;
pub use auth_routes::auth_routes;
pub use carrito_routes::carrito_routes;
pub use direccion_routes::direccion_routes;
pub use checkout_routes::checkout_routes;
pub use venta_routes::*;
pub use inventario_routes::*;
pub use producto_routes::*;
pub use descuento_routes::*;
