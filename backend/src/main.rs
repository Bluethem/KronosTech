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
use routes::{catalogo_routes, venta_routes, inventario_routes, producto_routes, descuento_routes, cupon_routes};
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
        .nest("/api", catalogo_routes(pool.clone()))
        .nest("/api", venta_routes(pool.clone()))
        .nest("/api", inventario_routes(pool.clone()))
        .nest("/api", producto_routes(pool.clone()))
        .nest("/api", descuento_routes(pool.clone()))
        .nest("/api", cupon_routes(pool))
        .layer(cors);

    let addr = settings.server_address();
    println!("🌐 Server listening on http://{}", addr);
    println!("📦 API Endpoints:");
    println!("   GET  /api/familias");
    println!("   GET  /api/categorias");
    println!("   GET  /api/subcategorias");
    println!("   GET  /api/marcas");
    println!("   GET  /api/productos");
    println!("   GET  /api/productos/{{id}}");
    println!("   GET  /api/productos/slug/{{slug}}");
    println!("   GET  /api/ventas");

    // Iniciar servidor
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind server");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
