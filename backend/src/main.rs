mod config;
mod handlers;
mod models;
mod repositories;
mod routes;
mod services;
mod utils;

use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    Router,
};
use config::{DatabaseConfig, Settings};
use routes::{
    catalogo_routes,
    auth_routes,
    carrito_routes,
    direccion_routes,
    checkout_routes,
    metodo_pago_cliente_routes,
    admin_routes,
    venta_routes,
    inventario_routes,
    producto_routes,
    descuento_routes,
    cupon_routes,
    reembolso_routes,
    log_routes,
    config_routes
};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    // Cargar configuración
    let settings = Settings::new().expect("Failed to load settings");

    println!("🚀 Starting KronosTech E-Commerce Backend");
    println!("📊 Connecting to database...");

    // Conectar a la base de datos
    let db_config = DatabaseConfig::new(&settings.database_url)
        .await
        .expect("Failed to connect to database");

    println!("✅ Database connected successfully");

    let pool = db_config.pool().clone();

    // Configurar CORS
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::PATCH])
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_credentials(true);

    // Construir rutas
    let app = Router::new()
        // Rutas de catálogo (público)
        .nest("/api", catalogo_routes(pool.clone()))
        // Rutas de autenticación y carrito (H3nr7)
        .nest("/api/auth", auth_routes(pool.clone()))
        .nest("/api", carrito_routes(pool.clone()))
        .nest("/api", direccion_routes(pool.clone()))
        .nest("/api", checkout_routes(pool.clone()))
        .nest("/api/metodos-pago-cliente", metodo_pago_cliente_routes(pool.clone()))
        .nest("/api/admin", admin_routes(pool.clone()))
        // Rutas de administración (main)
        .nest("/api", venta_routes(pool.clone()))
        .nest("/api", inventario_routes(pool.clone()))
        .nest("/api", producto_routes(pool.clone()))
        .nest("/api", descuento_routes(pool.clone()))
        .nest("/api", cupon_routes(pool.clone()))
        .nest("/api", reembolso_routes(pool.clone()))
        // Rutas de logs y auditoría
        .nest("/api/logs", log_routes(pool.clone()))
        // Rutas de configuración del sistema
        .nest("/api/config", config_routes(pool))
        .layer(cors);

    let addr = settings.server_address();
    println!("🌐 Server listening on http://{}", addr);
    println!("📦 API Endpoints:");
    println!("   === Catálogo (Público) ===");
    println!("   GET  /api/familias");
    println!("   GET  /api/categorias");
    println!("   GET  /api/subcategorias");
    println!("   GET  /api/marcas");
    println!("   GET  /api/productos");
    println!("   GET  /api/productos/{{id}}");
    println!("   GET  /api/productos/slug/{{slug}}");
    println!("   === Autenticación ===");
    println!("   POST /api/auth/register");
    println!("   POST /api/auth/login");
    println!("   GET  /api/auth/me");
    println!("   POST /api/auth/logout");
    println!("   === Carrito de Compras ===");
    println!("   GET    /api/carrito");
    println!("   POST   /api/carrito/items");
    println!("   PATCH  /api/carrito/items/{{id}}");
    println!("   DELETE /api/carrito/items/{{id}}");
    println!("   DELETE /api/carrito");
    println!("   === Direcciones ===");
    println!("   GET    /api/direcciones");
    println!("   POST   /api/direcciones");
    println!("   PUT    /api/direcciones/{{id}}");
    println!("   DELETE /api/direcciones/{{id}}");
    println!("   === Checkout y Pedidos ===");
    println!("   GET    /api/metodos-pago");
    println!("   GET    /api/checkout/calcular-total");
    println!("   POST   /api/checkout/procesar");
    println!("   GET    /api/pedidos");
    println!("   GET    /api/pedidos/{{id}}");
    println!("   === Administración - Ventas ===");
    println!("   GET    /api/ventas");
    println!("   GET    /api/ventas/{{id}}");
    println!("   PATCH  /api/ventas/{{id}}/estado");
    println!("   === Administración - Inventario ===");
    println!("   GET    /api/inventario");
    println!("   POST   /api/inventario/movimiento");
    println!("   GET    /api/inventario/movimientos");
    println!("   === Administración - Productos ===");
    println!("   POST   /api/productos");
    println!("   PUT    /api/productos/{{id}}");
    println!("   DELETE /api/productos/{{id}}");
    println!("   === Administración - Descuentos ===");
    println!("   GET    /api/descuentos");
    println!("   POST   /api/descuentos");
    println!("   PUT    /api/descuentos/{{id}}");
    println!("   DELETE /api/descuentos/{{id}}");
    println!("   === Administración - Cupones ===");
    println!("   GET    /api/cupones");
    println!("   POST   /api/cupones");
    println!("   PUT    /api/cupones/{{id}}");
    println!("   DELETE /api/cupones/{{id}}");
    println!("   === Administración - Reembolsos ===");
    println!("   GET    /api/reembolsos");
    println!("   POST   /api/reembolsos");
    println!("   PATCH  /api/reembolsos/{{id}}/estado");
    println!("   === Logs y Auditoría ===");
    println!("   GET    /api/logs");
    println!("   POST   /api/logs");
    println!("   DELETE /api/logs/limpiar");
    println!("   === Configuración del Sistema ===");
    println!("   GET    /api/config");
    println!("   PUT    /api/config");
    println!("   GET    /api/config/session-timeout");
    println!("   GET    /api/config/:clave");
    println!("   PUT    /api/config/:clave");

    // Iniciar servidor
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind server");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
