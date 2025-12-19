// Modelos base - Catálogo
pub mod familia;
pub mod categoria;
pub mod subcategoria;
pub mod marca;
pub mod producto;
pub mod producto_detalle;
pub mod valoracion;

// Modelos - Usuario y Carrito (H3nr7 - mantener tu implementación)
pub mod usuario;
pub mod carrito;

// Modelos - Direcciones, Ventas, Pagos (main - usar implementación del compañero)
pub mod direccion;
pub mod venta;
pub mod metodo_pago;

// Modelos - Administración y otros (main - del compañero)
pub mod administrador;
pub mod especificacion_producto;
pub mod imagen_producto;
pub mod inventario;
pub mod movimiento_inventario;
pub mod descuento;
pub mod cupon;
pub mod asignacion_cupon;
pub mod uso_cupon;
pub mod carrito_detalle;
pub mod detalle_venta;
pub mod metodo_pago_cliente;
pub mod pago;
pub mod historial_estado_pago;
pub mod reembolso;
pub mod envio;
pub mod imagen_valoracion;
pub mod notificacion;
pub mod lista_deseos;
pub mod log_auditoria;
pub mod configuracion;

// Re-exportaciones para uso interno
pub use subcategoria::Subcategoria;
pub use valoracion::Valoracion;

// Usuario y Carrito - TU implementación (con DTOs)
pub use usuario::{Usuario, UsuarioResponse, LoginRequest, LoginResponse, RegisterRequest, RegisterResponse};
pub use carrito::{Carrito, CarritoDetalle, CarritoResponse, CarritoItemResponse, AgregarAlCarritoRequest, ActualizarCantidadRequest};

// Direcciones, Ventas, Pagos - Implementación del compañero + DTOs agregados
pub use direccion::{Direccion, CrearDireccionRequest, ActualizarDireccionRequest, TipoDireccion, DireccionResponse};
pub use venta::{Venta, ProcesarCheckoutRequest, CalcularTotalResponse, VentaResponse, DetalleVentaResponse, EstadoPedido, EstadoPago};
pub use metodo_pago::{MetodoPago, MetodoPagoResponse};

// Modelos adicionales del compañero
pub use administrador::Administrador;
pub use especificacion_producto::EspecificacionProducto;
pub use imagen_producto::ImagenProducto;
pub use inventario::Inventario;
pub use movimiento_inventario::MovimientoInventario;
pub use descuento::Descuento;
pub use cupon::Cupon;
pub use asignacion_cupon::AsignacionCupon;
pub use uso_cupon::UsoCupon;
// CarritoDetalle ya está exportado desde carrito.rs arriba
pub use detalle_venta::DetalleVenta;
pub use metodo_pago_cliente::MetodoPagoCliente;
pub use pago::Pago;
pub use historial_estado_pago::HistorialEstadoPago;
pub use reembolso::Reembolso;
pub use envio::Envio;
pub use imagen_valoracion::ImagenValoracion;
pub use notificacion::Notificacion;
pub use lista_deseos::ListaDeseos;
pub use log_auditoria::{LogAuditoria, CrearLogRequest, FiltrarLogsQuery, LogResponse};
pub use configuracion::{ConfiguracionSistema, ActualizarConfigRequest, ActualizarConfigBatchRequest};
