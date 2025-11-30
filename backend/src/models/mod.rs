pub mod familia;
pub mod categoria;
pub mod subcategoria;
pub mod marca;
pub mod producto;
pub mod producto_detalle;
pub mod valoracion;
pub mod usuario;
pub mod carrito;
pub mod direccion;
pub mod venta;
pub mod metodo_pago;

// Re-exportaciones para uso interno
pub use subcategoria::Subcategoria;
pub use valoracion::Valoracion;
pub use usuario::{Usuario, UsuarioResponse, LoginRequest, LoginResponse, RegisterRequest, RegisterResponse};
pub use carrito::{Carrito, CarritoDetalle, CarritoResponse, CarritoItemResponse, AgregarAlCarritoRequest, ActualizarCantidadRequest};
pub use direccion::{Direccion, DireccionResponse, CrearDireccionRequest, ActualizarDireccionRequest};
pub use venta::{Venta, DetalleVenta, VentaResponse, DetalleVentaResponse, ProcesarCheckoutRequest, CalcularTotalResponse, EstadoPedido, EstadoPago};
pub use metodo_pago::{MetodoPago, MetodoPagoResponse, Pago};
