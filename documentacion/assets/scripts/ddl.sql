-- ============================================================================
-- BASE DE DATOS: E-COMMERCE DE COMPONENTES DE PC
-- DBMS: PostgreSQL 15+
-- Stack: Rust + Svelte + PostgreSQL
-- ============================================================================

-- Extensiones necesarias
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";
CREATE EXTENSION IF NOT EXISTS "pg_trgm";  -- Para búsqueda difusa

-- ============================================================================
-- TIPOS ENUMERADOS (ENUMS)
-- ============================================================================

CREATE TYPE estado_general AS ENUM ('activo', 'inactivo');

CREATE TYPE rol_usuario AS ENUM ('cliente', 'administrador', 'super_admin');

CREATE TYPE tipo_metodo_pago AS ENUM (
    'tarjeta_credito',
    'tarjeta_debito',
    'billetera_digital',
    'transferencia',
    'efectivo',
    'contrareembolso'
);

CREATE TYPE estado_pedido AS ENUM (
    'pendiente',
    'confirmado',
    'procesando',
    'enviado',
    'entregado',
    'cancelado',
    'devuelto'
);

CREATE TYPE estado_pago AS ENUM (
    'pendiente',
    'procesando',
    'completado',
    'fallido',
    'rechazado',
    'cancelado',
    'reembolsado',
    'parcialmente_reembolsado'
);

CREATE TYPE tipo_descuento AS ENUM ('porcentaje', 'monto_fijo');

CREATE TYPE aplica_descuento AS ENUM ('producto', 'categoria', 'marca', 'familia', 'global');

CREATE TYPE tipo_cupon AS ENUM ('porcentaje', 'monto_fijo', 'envio_gratis');

CREATE TYPE aplica_cupon AS ENUM ('todo', 'producto', 'categoria', 'marca', 'familia');

CREATE TYPE estado_carrito AS ENUM ('activo', 'convertido', 'abandonado', 'expirado');

CREATE TYPE tipo_movimiento_inventario AS ENUM (
    'entrada',
    'salida',
    'ajuste',
    'merma',
    'devolucion',
    'transferencia'
);

CREATE TYPE estado_reembolso AS ENUM ('solicitado', 'procesando', 'completado', 'rechazado');

CREATE TYPE tipo_direccion AS ENUM ('envio', 'facturacion', 'ambos');

CREATE TYPE estado_envio AS ENUM (
    'pendiente',
    'preparando',
    'enviado',
    'en_transito',
    'entregado',
    'fallido'
);

-- ============================================================================
-- TABLAS: GESTIÓN DE USUARIOS
-- ============================================================================

CREATE TABLE usuario (
    id_usuario SERIAL PRIMARY KEY,
    nombre VARCHAR(100) NOT NULL,
    apellido VARCHAR(100) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    contrasena VARCHAR(255) NOT NULL,  -- Hash con Argon2/bcrypt
    telefono VARCHAR(20),
    dni VARCHAR(20),
    rol rol_usuario DEFAULT 'cliente',
    email_verificado BOOLEAN DEFAULT FALSE,
    token_verificacion VARCHAR(100),
    activo BOOLEAN DEFAULT TRUE,
    fecha_registro TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    ultima_conexion TIMESTAMP,
    fecha_actualizacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    CONSTRAINT email_valido CHECK (email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$')
);

CREATE INDEX idx_usuario_email ON usuario(email);
CREATE INDEX idx_usuario_rol ON usuario(rol);
CREATE INDEX idx_usuario_activo ON usuario(activo);

COMMENT ON TABLE usuario IS 'Tabla principal de usuarios del sistema';
COMMENT ON COLUMN usuario.contrasena IS 'Hash de contraseña - NUNCA almacenar en texto plano';

-- ============================================================================

CREATE TABLE direccion (
    id_direccion SERIAL PRIMARY KEY,
    id_usuario INTEGER NOT NULL,
    tipo tipo_direccion DEFAULT 'envio',
    nombre_completo VARCHAR(200),  -- Nombre del destinatario
    direccion_linea1 TEXT NOT NULL,
    direccion_linea2 TEXT,
    ciudad VARCHAR(100) NOT NULL,
    departamento VARCHAR(100) NOT NULL,
    codigo_postal VARCHAR(20),
    pais VARCHAR(100) DEFAULT 'Perú',
    telefono_contacto VARCHAR(20),
    referencia TEXT,
    es_predeterminada BOOLEAN DEFAULT FALSE,
    activo BOOLEAN DEFAULT TRUE,
    fecha_creacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (id_usuario) REFERENCES usuario(id_usuario) ON DELETE CASCADE
);

CREATE INDEX idx_direccion_usuario ON direccion(id_usuario);
CREATE INDEX idx_direccion_predeterminada ON direccion(id_usuario, es_predeterminada) WHERE es_predeterminada = TRUE;

-- ============================================================================

CREATE TABLE administrador (
    id_administrador SERIAL PRIMARY KEY,
    id_usuario INTEGER NOT NULL UNIQUE,
    es_super_admin BOOLEAN DEFAULT FALSE,
    permisos JSONB,  -- Permisos granulares
    fecha_creacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (id_usuario) REFERENCES usuario(id_usuario) ON DELETE CASCADE
);

-- ============================================================================
-- TABLAS: CATÁLOGO DE PRODUCTOS
-- ============================================================================

CREATE TABLE familia (
    id_familia SERIAL PRIMARY KEY,
    nombre VARCHAR(100) NOT NULL UNIQUE,
    descripcion TEXT,
    icono VARCHAR(255),
    slug VARCHAR(100) UNIQUE,  -- Para URLs amigables
    orden INTEGER DEFAULT 0,
    estado estado_general DEFAULT 'activo',
    fecha_creacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    fecha_actualizacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_familia_estado ON familia(estado);
CREATE INDEX idx_familia_slug ON familia(slug);

-- ============================================================================

CREATE TABLE categoria (
    id_categoria SERIAL PRIMARY KEY,
    nombre VARCHAR(100) NOT NULL,
    descripcion TEXT,
    icono VARCHAR(255),
    slug VARCHAR(100) UNIQUE,
    id_familia INTEGER NOT NULL,
    orden INTEGER DEFAULT 0,
    estado estado_general DEFAULT 'activo',
    fecha_creacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    fecha_actualizacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (id_familia) REFERENCES familia(id_familia) ON DELETE RESTRICT,
    UNIQUE(nombre, id_familia)
);

CREATE INDEX idx_categoria_familia ON categoria(id_familia);
CREATE INDEX idx_categoria_estado ON categoria(estado);
CREATE INDEX idx_categoria_slug ON categoria(slug);

-- ============================================================================

CREATE TABLE subcategoria (
    id_subcategoria SERIAL PRIMARY KEY,
    nombre VARCHAR(100) NOT NULL,
    descripcion TEXT,
    slug VARCHAR(100) UNIQUE,
    id_categoria INTEGER NOT NULL,
    orden INTEGER DEFAULT 0,
    estado estado_general DEFAULT 'activo',
    fecha_creacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    fecha_actualizacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (id_categoria) REFERENCES categoria(id_categoria) ON DELETE RESTRICT,
    UNIQUE(nombre, id_categoria)
);

CREATE INDEX idx_subcategoria_categoria ON subcategoria(id_categoria);
CREATE INDEX idx_subcategoria_estado ON subcategoria(estado);

-- ============================================================================

CREATE TABLE marca (
    id_marca SERIAL PRIMARY KEY,
    nombre VARCHAR(100) NOT NULL UNIQUE,
    descripcion TEXT,
    logo VARCHAR(255),
    slug VARCHAR(100) UNIQUE,
    pais_origen VARCHAR(100),
    sitio_web VARCHAR(255),
    estado estado_general DEFAULT 'activo',
    fecha_creacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    fecha_actualizacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_marca_estado ON marca(estado);
CREATE INDEX idx_marca_slug ON marca(slug);

-- ============================================================================

CREATE TABLE producto (
    id_producto SERIAL PRIMARY KEY,
    nombre VARCHAR(200) NOT NULL,
    descripcion TEXT,
    slug VARCHAR(200) UNIQUE,
    id_categoria INTEGER NOT NULL,
    id_subcategoria INTEGER,
    especificaciones_base JSONB,  -- Especificaciones comunes del tipo
    imagen_referencia VARCHAR(255),
    -- Valoraciones
    valoracion_promedio DECIMAL(3,2) DEFAULT 0 CHECK (valoracion_promedio BETWEEN 0 AND 5),
    total_valoraciones INTEGER DEFAULT 0,
    -- SEO
    meta_title VARCHAR(200),
    meta_description TEXT,
    keywords TEXT,
    -- Estado
    estado estado_general DEFAULT 'activo',
    fecha_creacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    fecha_actualizacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (id_categoria) REFERENCES categoria(id_categoria) ON DELETE RESTRICT,
    FOREIGN KEY (id_subcategoria) REFERENCES subcategoria(id_subcategoria) ON DELETE SET NULL
);

CREATE INDEX idx_producto_categoria ON producto(id_categoria);
CREATE INDEX idx_producto_subcategoria ON producto(id_subcategoria);
CREATE INDEX idx_producto_estado ON producto(estado);
CREATE INDEX idx_producto_valoracion ON producto(valoracion_promedio DESC);
CREATE INDEX idx_producto_slug ON producto(slug);
-- Índice GIN para búsqueda full-text
CREATE INDEX idx_producto_busqueda ON producto USING gin(to_tsvector('spanish', nombre || ' ' || COALESCE(descripcion, '')));

COMMENT ON COLUMN producto.especificaciones_base IS 'Especificaciones en formato JSON para flexibilidad';

-- ============================================================================

CREATE TABLE producto_detalle (
    id_producto_detalle SERIAL PRIMARY KEY,
    nombre VARCHAR(200) NOT NULL,
    descripcion TEXT,
    modelo VARCHAR(100),
    sku VARCHAR(100) NOT NULL UNIQUE,
    slug VARCHAR(200) UNIQUE,
    codigo_barras VARCHAR(50),
    id_producto INTEGER NOT NULL,
    id_marca INTEGER NOT NULL,
    -- Precios
    precio_base DECIMAL(10,2) NOT NULL CHECK (precio_base >= 0),
    precio_venta DECIMAL(10,2) NOT NULL CHECK (precio_venta >= 0),
    costo DECIMAL(10,2) CHECK (costo >= 0),
    -- Descuentos adicionales del detalle
    descuento_adicional_porcentaje DECIMAL(5,2) DEFAULT 0 CHECK (descuento_adicional_porcentaje BETWEEN 0 AND 100),
    descuento_adicional_activo BOOLEAN DEFAULT FALSE,
    -- Imágenes
    imagen_principal VARCHAR(255),
    imagenes JSONB,  -- Array de URLs de imágenes
    -- Físico
    peso DECIMAL(8,2),  -- kg
    dimensiones VARCHAR(100),  -- "30x20x10 cm"
    -- Garantía
    garantia_meses INTEGER DEFAULT 12,
    -- Estadísticas
    total_vendidos INTEGER DEFAULT 0,
    fecha_ultima_venta TIMESTAMP,
    vistas INTEGER DEFAULT 0,
    -- Destacados
    es_destacado BOOLEAN DEFAULT FALSE,
    es_nuevo BOOLEAN DEFAULT FALSE,
    es_oferta BOOLEAN DEFAULT FALSE,
    -- Estado
    estado estado_general DEFAULT 'activo',
    fecha_creacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    fecha_actualizacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (id_producto) REFERENCES producto(id_producto) ON DELETE RESTRICT,
    FOREIGN KEY (id_marca) REFERENCES marca(id_marca) ON DELETE RESTRICT
);

CREATE INDEX idx_producto_detalle_producto ON producto_detalle(id_producto);
CREATE INDEX idx_producto_detalle_marca ON producto_detalle(id_marca);
CREATE INDEX idx_producto_detalle_sku ON producto_detalle(sku);
CREATE INDEX idx_producto_detalle_estado ON producto_detalle(estado);
CREATE INDEX idx_producto_detalle_destacado ON producto_detalle(es_destacado) WHERE es_destacado = TRUE;
CREATE INDEX idx_producto_detalle_precio ON producto_detalle(precio_venta);
CREATE INDEX idx_producto_detalle_ventas ON producto_detalle(total_vendidos DESC);
CREATE INDEX idx_producto_detalle_slug ON producto_detalle(slug);

-- ============================================================================

CREATE TABLE especificacion_producto (
    id_especificacion SERIAL PRIMARY KEY,
    id_producto_detalle INTEGER NOT NULL,
    nombre_atributo VARCHAR(100) NOT NULL,
    valor_atributo TEXT NOT NULL,
    unidad_medida VARCHAR(50),
    orden INTEGER DEFAULT 0,
    
    FOREIGN KEY (id_producto_detalle) REFERENCES producto_detalle(id_producto_detalle) ON DELETE CASCADE
);

CREATE INDEX idx_especificacion_producto_detalle ON especificacion_producto(id_producto_detalle);

-- ============================================================================

CREATE TABLE imagen_producto (
    id_imagen SERIAL PRIMARY KEY,
    id_producto_detalle INTEGER NOT NULL,
    url_imagen VARCHAR(255) NOT NULL,
    es_principal BOOLEAN DEFAULT FALSE,
    orden INTEGER DEFAULT 0,
    alt_text VARCHAR(200),
    titulo VARCHAR(200),
    fecha_subida TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (id_producto_detalle) REFERENCES producto_detalle(id_producto_detalle) ON DELETE CASCADE
);

CREATE INDEX idx_imagen_producto_detalle ON imagen_producto(id_producto_detalle);

-- ============================================================================
-- TABLAS: INVENTARIO
-- ============================================================================

CREATE TABLE inventario (
    id_inventario SERIAL PRIMARY KEY,
    id_producto_detalle INTEGER NOT NULL UNIQUE,
    cantidad_disponible INTEGER NOT NULL DEFAULT 0 CHECK (cantidad_disponible >= 0),
    cantidad_minima INTEGER NOT NULL DEFAULT 5 CHECK (cantidad_minima >= 0),
    cantidad_maxima INTEGER DEFAULT 1000,
    ubicacion_fisica VARCHAR(100),
    lote VARCHAR(50),
    fecha_vencimiento DATE,
    fecha_ultima_entrada DATE,
    fecha_ultima_salida DATE,
    fecha_actualizacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (id_producto_detalle) REFERENCES producto_detalle(id_producto_detalle) ON DELETE RESTRICT
);

CREATE INDEX idx_inventario_producto_detalle ON inventario(id_producto_detalle);
CREATE INDEX idx_inventario_stock_bajo ON inventario(cantidad_disponible) WHERE cantidad_disponible < cantidad_minima;
CREATE INDEX idx_inventario_sin_stock ON inventario(cantidad_disponible) WHERE cantidad_disponible = 0;

-- ============================================================================

CREATE TABLE movimiento_inventario (
    id_movimiento SERIAL PRIMARY KEY,
    id_inventario INTEGER NOT NULL,
    id_producto_detalle INTEGER NOT NULL,
    tipo_movimiento tipo_movimiento_inventario NOT NULL,
    cantidad INTEGER NOT NULL CHECK (cantidad != 0),
    cantidad_anterior INTEGER NOT NULL,
    cantidad_nueva INTEGER NOT NULL,
    motivo TEXT,
    id_usuario INTEGER,
    id_venta INTEGER,
    id_proveedor INTEGER,
    costo_unitario DECIMAL(10,2),
    documento_referencia VARCHAR(100),
    fecha_movimiento TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    notas TEXT,
    
    FOREIGN KEY (id_inventario) REFERENCES inventario(id_inventario) ON DELETE RESTRICT,
    FOREIGN KEY (id_producto_detalle) REFERENCES producto_detalle(id_producto_detalle) ON DELETE RESTRICT,
    FOREIGN KEY (id_usuario) REFERENCES usuario(id_usuario) ON DELETE SET NULL
);

CREATE INDEX idx_movimiento_inventario ON movimiento_inventario(id_inventario);
CREATE INDEX idx_movimiento_producto_detalle ON movimiento_inventario(id_producto_detalle);
CREATE INDEX idx_movimiento_tipo ON movimiento_inventario(tipo_movimiento);
CREATE INDEX idx_movimiento_fecha ON movimiento_inventario(fecha_movimiento DESC);

-- ============================================================================
-- TABLAS: DESCUENTOS Y CUPONES
-- ============================================================================

CREATE TABLE descuento (
    id_descuento SERIAL PRIMARY KEY,
    nombre VARCHAR(100) NOT NULL,
    descripcion TEXT,
    tipo_descuento tipo_descuento NOT NULL,
    valor DECIMAL(10,2) NOT NULL CHECK (valor >= 0),
    aplica_a aplica_descuento NOT NULL,
    id_referencia INTEGER,  -- ID del producto/categoria/marca/familia
    -- Restricciones
    compra_minima DECIMAL(10,2) DEFAULT 0,
    cantidad_minima INTEGER DEFAULT 1,
    usos_maximos INTEGER,  -- NULL = ilimitado
    usos_actuales INTEGER DEFAULT 0,
    -- Vigencia
    fecha_inicio TIMESTAMP NOT NULL,
    fecha_fin TIMESTAMP NOT NULL,
    activo BOOLEAN DEFAULT TRUE,
    fecha_creacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    fecha_actualizacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    CONSTRAINT fechas_validas CHECK (fecha_fin > fecha_inicio)
);

CREATE INDEX idx_descuento_vigencia ON descuento(fecha_inicio, fecha_fin) WHERE activo = TRUE;
CREATE INDEX idx_descuento_aplica ON descuento(aplica_a, id_referencia);
CREATE INDEX idx_descuento_activo ON descuento(activo);

-- ============================================================================

CREATE TABLE cupon (
    id_cupon SERIAL PRIMARY KEY,
    codigo VARCHAR(50) NOT NULL UNIQUE,
    nombre VARCHAR(100) NOT NULL,
    descripcion TEXT,
    tipo_cupon tipo_cupon NOT NULL,
    valor DECIMAL(10,2) NOT NULL CHECK (valor >= 0),
    -- Aplicabilidad
    aplica_a aplica_cupon NOT NULL,
    id_referencia INTEGER,
    -- Restricciones
    compra_minima DECIMAL(10,2) DEFAULT 0,
    usos_maximos INTEGER,  -- NULL = ilimitado
    usos_maximos_por_usuario INTEGER DEFAULT 1,
    usos_actuales INTEGER DEFAULT 0,
    -- Vigencia
    fecha_inicio TIMESTAMP NOT NULL,
    fecha_fin TIMESTAMP NOT NULL,
    -- Restricciones de usuario
    solo_nuevos_usuarios BOOLEAN DEFAULT FALSE,
    solo_primera_compra BOOLEAN DEFAULT FALSE,
    activo BOOLEAN DEFAULT TRUE,
    fecha_creacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    fecha_actualizacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    CONSTRAINT fechas_validas_cupon CHECK (fecha_fin > fecha_inicio)
);

CREATE INDEX idx_cupon_codigo ON cupon(codigo);
CREATE INDEX idx_cupon_vigencia ON cupon(fecha_inicio, fecha_fin) WHERE activo = TRUE;
CREATE INDEX idx_cupon_activo ON cupon(activo);

-- ============================================================================

CREATE TABLE asignacion_cupon (
    id_asignacion SERIAL PRIMARY KEY,
    id_cupon INTEGER NOT NULL,
    id_usuario INTEGER NOT NULL,
    usado BOOLEAN DEFAULT FALSE,
    fecha_asignacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    fecha_uso TIMESTAMP,
    
    FOREIGN KEY (id_cupon) REFERENCES cupon(id_cupon) ON DELETE CASCADE,
    FOREIGN KEY (id_usuario) REFERENCES usuario(id_usuario) ON DELETE CASCADE,
    UNIQUE(id_cupon, id_usuario)
);

CREATE INDEX idx_asignacion_cupon ON asignacion_cupon(id_cupon);
CREATE INDEX idx_asignacion_usuario ON asignacion_cupon(id_usuario);

-- ============================================================================
-- TABLAS: CARRITO DE COMPRAS
-- ============================================================================

CREATE TABLE carrito (
    id_carrito SERIAL PRIMARY KEY,
    id_usuario INTEGER,  -- NULL para usuarios no registrados
    id_sesion VARCHAR(100),  -- Para usuarios anónimos
    estado estado_carrito DEFAULT 'activo',
    fecha_expiracion TIMESTAMP,
    fecha_creacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    fecha_actualizacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (id_usuario) REFERENCES usuario(id_usuario) ON DELETE CASCADE,
    CONSTRAINT usuario_o_sesion CHECK (id_usuario IS NOT NULL OR id_sesion IS NOT NULL)
);

CREATE INDEX idx_carrito_usuario ON carrito(id_usuario);
CREATE INDEX idx_carrito_sesion ON carrito(id_sesion);
CREATE INDEX idx_carrito_estado ON carrito(estado);
CREATE INDEX idx_carrito_expiracion ON carrito(fecha_expiracion) WHERE estado = 'activo';

-- ============================================================================

CREATE TABLE carrito_detalle (
    id_carrito_detalle SERIAL PRIMARY KEY,
    id_carrito INTEGER NOT NULL,
    id_producto_detalle INTEGER NOT NULL,
    cantidad INTEGER NOT NULL CHECK (cantidad > 0),
    precio_unitario DECIMAL(10,2) NOT NULL,  -- Snapshot del precio
    fecha_agregado TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    fecha_actualizacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (id_carrito) REFERENCES carrito(id_carrito) ON DELETE CASCADE,
    FOREIGN KEY (id_producto_detalle) REFERENCES producto_detalle(id_producto_detalle) ON DELETE CASCADE,
    UNIQUE(id_carrito, id_producto_detalle)
);

CREATE INDEX idx_carrito_detalle_carrito ON carrito_detalle(id_carrito);
CREATE INDEX idx_carrito_detalle_producto ON carrito_detalle(id_producto_detalle);

-- ============================================================================
-- TABLAS: VENTAS/PEDIDOS
-- ============================================================================

CREATE TABLE venta (
    id_venta SERIAL PRIMARY KEY,
    numero_pedido VARCHAR(50) NOT NULL UNIQUE,
    id_usuario INTEGER NOT NULL,
    id_carrito INTEGER,
    -- Montos
    subtotal DECIMAL(10,2) NOT NULL CHECK (subtotal >= 0),
    descuento_total DECIMAL(10,2) DEFAULT 0 CHECK (descuento_total >= 0),
    costo_envio DECIMAL(10,2) DEFAULT 0 CHECK (costo_envio >= 0),
    total DECIMAL(10,2) NOT NULL CHECK (total >= 0),
    moneda VARCHAR(3) DEFAULT 'PEN',
    -- Estados
    estado estado_pedido DEFAULT 'pendiente',
    estado_pago estado_pago DEFAULT 'pendiente',
    -- Envío (snapshot)
    direccion_envio TEXT,
    ciudad VARCHAR(100),
    departamento VARCHAR(100),
    codigo_postal VARCHAR(20),
    telefono_contacto VARCHAR(20),
    metodo_envio VARCHAR(100),
    numero_tracking VARCHAR(100),
    -- Fechas
    fecha_pedido TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    fecha_pago TIMESTAMP,
    fecha_confirmacion TIMESTAMP,
    fecha_envio TIMESTAMP,
    fecha_entrega_estimada DATE,
    fecha_entrega TIMESTAMP,
    fecha_cancelacion TIMESTAMP,
    -- Notas
    notas_cliente TEXT,
    notas_admin TEXT,
    -- Tracking
    ip_cliente VARCHAR(45),
    user_agent TEXT,
    fecha_actualizacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (id_usuario) REFERENCES usuario(id_usuario) ON DELETE RESTRICT,
    FOREIGN KEY (id_carrito) REFERENCES carrito(id_carrito) ON DELETE SET NULL
);

CREATE INDEX idx_venta_usuario ON venta(id_usuario);
CREATE INDEX idx_venta_numero ON venta(numero_pedido);
CREATE INDEX idx_venta_estado ON venta(estado);
CREATE INDEX idx_venta_estado_pago ON venta(estado_pago);
CREATE INDEX idx_venta_fecha ON venta(fecha_pedido DESC);

-- ============================================================================

CREATE TABLE detalle_venta (
    id_detalle_venta SERIAL PRIMARY KEY,
    id_venta INTEGER NOT NULL,
    id_producto_detalle INTEGER NOT NULL,
    id_producto INTEGER NOT NULL,  -- Para reportes agregados
    cantidad INTEGER NOT NULL CHECK (cantidad > 0),
    precio_unitario DECIMAL(10,2) NOT NULL,  -- Precio al momento de venta
    descuento_unitario DECIMAL(10,2) DEFAULT 0,
    precio_final DECIMAL(10,2) NOT NULL,
    subtotal DECIMAL(10,2) NOT NULL,  -- cantidad * precio_final
    
    FOREIGN KEY (id_venta) REFERENCES venta(id_venta) ON DELETE RESTRICT,
    FOREIGN KEY (id_producto_detalle) REFERENCES producto_detalle(id_producto_detalle) ON DELETE RESTRICT,
    FOREIGN KEY (id_producto) REFERENCES producto(id_producto) ON DELETE RESTRICT
);

CREATE INDEX idx_detalle_venta ON detalle_venta(id_venta);
CREATE INDEX idx_detalle_producto_detalle ON detalle_venta(id_producto_detalle);
CREATE INDEX idx_detalle_producto ON detalle_venta(id_producto);

-- ============================================================================

CREATE TABLE uso_cupon (
    id_uso_cupon SERIAL PRIMARY KEY,
    id_cupon INTEGER NOT NULL,
    id_venta INTEGER NOT NULL,
    id_usuario INTEGER NOT NULL,
    descuento_aplicado DECIMAL(10,2) NOT NULL,
    fecha_uso TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (id_cupon) REFERENCES cupon(id_cupon) ON DELETE RESTRICT,
    FOREIGN KEY (id_venta) REFERENCES venta(id_venta) ON DELETE RESTRICT,
    FOREIGN KEY (id_usuario) REFERENCES usuario(id_usuario) ON DELETE RESTRICT
);

CREATE INDEX idx_uso_cupon_cupon ON uso_cupon(id_cupon);
CREATE INDEX idx_uso_cupon_venta ON uso_cupon(id_venta);
CREATE INDEX idx_uso_cupon_usuario ON uso_cupon(id_usuario);

-- ============================================================================
-- TABLAS: PAGOS
-- ============================================================================

CREATE TABLE metodo_pago (
    id_metodo_pago SERIAL PRIMARY KEY,
    nombre VARCHAR(100) NOT NULL,
    tipo tipo_metodo_pago NOT NULL,
    proveedor VARCHAR(100),  -- Visa, Mastercard, Yape, Stripe, etc
    descripcion TEXT,
    icono VARCHAR(255),
    comision_porcentaje DECIMAL(5,2) DEFAULT 0,
    comision_fija DECIMAL(10,2) DEFAULT 0,
    requiere_verificacion BOOLEAN DEFAULT FALSE,
    tiempo_procesamiento VARCHAR(50),
    instrucciones TEXT,
    orden INTEGER DEFAULT 0,
    activo BOOLEAN DEFAULT TRUE,
    fecha_creacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_metodo_pago_tipo ON metodo_pago(tipo);
CREATE INDEX idx_metodo_pago_activo ON metodo_pago(activo);

-- ============================================================================

CREATE TABLE metodo_pago_cliente (
    id_metodo_pago_cliente SERIAL PRIMARY KEY,
    id_usuario INTEGER NOT NULL,
    id_metodo_pago INTEGER NOT NULL,
    tipo tipo_metodo_pago NOT NULL,
    -- Token del procesador (NUNCA datos reales)
    token_pago VARCHAR(200),
    ultimos_4_digitos CHAR(4),
    marca VARCHAR(50),  -- Visa, Mastercard, Amex
    fecha_expiracion CHAR(7),  -- MM/YYYY
    nombre_titular VARCHAR(200),
    -- Configuración
    es_predeterminado BOOLEAN DEFAULT FALSE,
    activo BOOLEAN DEFAULT TRUE,
    fecha_creacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    fecha_actualizacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (id_usuario) REFERENCES usuario(id_usuario) ON DELETE CASCADE,
    FOREIGN KEY (id_metodo_pago) REFERENCES metodo_pago(id_metodo_pago) ON DELETE RESTRICT
);

CREATE INDEX idx_metodo_pago_cliente_usuario ON metodo_pago_cliente(id_usuario);
CREATE INDEX idx_metodo_pago_cliente_predeterminado ON metodo_pago_cliente(id_usuario, es_predeterminado) WHERE es_predeterminado = TRUE;

COMMENT ON TABLE metodo_pago_cliente IS 'Métodos de pago guardados del cliente - SOLO TOKENS, NUNCA datos reales';

-- ============================================================================

CREATE TABLE pago (
    id_pago SERIAL PRIMARY KEY,
    id_venta INTEGER NOT NULL,
    id_metodo_pago INTEGER NOT NULL,
    id_metodo_pago_cliente INTEGER,
    numero_transaccion VARCHAR(100) UNIQUE,
    estado estado_pago DEFAULT 'pendiente',
    monto DECIMAL(10,2) NOT NULL CHECK (monto >= 0),
    moneda VARCHAR(3) DEFAULT 'PEN',
    comision DECIMAL(10,2) DEFAULT 0,
    monto_neto DECIMAL(10,2) NOT NULL,  -- monto - comision
    -- Datos del procesador externo
    proveedor_pago VARCHAR(50),  -- Stripe, PayPal, MercadoPago, Culqi
    id_transaccion_proveedor VARCHAR(200),
    token_pago VARCHAR(200),  -- Token de un solo uso
    respuesta_proveedor JSONB,
    -- Datos de tarjeta tokenizados
    ultimos_4_digitos CHAR(4),
    marca_tarjeta VARCHAR(50),
    -- Seguridad
    ip_cliente VARCHAR(45),
    user_agent TEXT,
    -- Fechas
    fecha_pago TIMESTAMP,
    fecha_creacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    fecha_actualizacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    -- Errores
    nota_error TEXT,
    intentos_fallidos INTEGER DEFAULT 0,
    
    FOREIGN KEY (id_venta) REFERENCES venta(id_venta) ON DELETE RESTRICT,
    FOREIGN KEY (id_metodo_pago) REFERENCES metodo_pago(id_metodo_pago) ON DELETE RESTRICT,
    FOREIGN KEY (id_metodo_pago_cliente) REFERENCES metodo_pago_cliente(id_metodo_pago_cliente) ON DELETE SET NULL
);

CREATE INDEX idx_pago_venta ON pago(id_venta);
CREATE INDEX idx_pago_estado ON pago(estado);
CREATE INDEX idx_pago_transaccion ON pago(numero_transaccion);
CREATE INDEX idx_pago_proveedor ON pago(id_transaccion_proveedor);
CREATE INDEX idx_pago_fecha ON pago(fecha_pago DESC);

-- ============================================================================

CREATE TABLE historial_estado_pago (
    id_historial SERIAL PRIMARY KEY,
    id_pago INTEGER NOT NULL,
    estado_anterior VARCHAR(50),
    estado_nuevo VARCHAR(50) NOT NULL,
    razon TEXT,
    id_usuario INTEGER,
    metadatos JSONB,
    fecha_cambio TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (id_pago) REFERENCES pago(id_pago) ON DELETE CASCADE,
    FOREIGN KEY (id_usuario) REFERENCES usuario(id_usuario) ON DELETE SET NULL
);

CREATE INDEX idx_historial_pago ON historial_estado_pago(id_pago);
CREATE INDEX idx_historial_fecha ON historial_estado_pago(fecha_cambio DESC);

-- ============================================================================

CREATE TABLE reembolso (
    id_reembolso SERIAL PRIMARY KEY,
    id_pago INTEGER NOT NULL,
    id_venta INTEGER NOT NULL,
    tipo_reembolso VARCHAR(20) NOT NULL CHECK (tipo_reembolso IN ('total', 'parcial')),
    monto_reembolsado DECIMAL(10,2) NOT NULL CHECK (monto_reembolsado > 0),
    motivo TEXT NOT NULL,
    estado estado_reembolso DEFAULT 'solicitado',
    -- Datos del procesador
    id_reembolso_proveedor VARCHAR(200),
    respuesta_proveedor JSONB,
    -- Responsables
    id_usuario_solicitante INTEGER,
    id_usuario_aprobador INTEGER,
    -- Fechas
    fecha_solicitado TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    fecha_aprobado TIMESTAMP,
    fecha_completado TIMESTAMP,
    notas_admin TEXT,
    
    FOREIGN KEY (id_pago) REFERENCES pago(id_pago) ON DELETE RESTRICT,
    FOREIGN KEY (id_venta) REFERENCES venta(id_venta) ON DELETE RESTRICT,
    FOREIGN KEY (id_usuario_solicitante) REFERENCES usuario(id_usuario) ON DELETE SET NULL,
    FOREIGN KEY (id_usuario_aprobador) REFERENCES usuario(id_usuario) ON DELETE SET NULL
);

CREATE INDEX idx_reembolso_pago ON reembolso(id_pago);
CREATE INDEX idx_reembolso_venta ON reembolso(id_venta);
CREATE INDEX idx_reembolso_estado ON reembolso(estado);

-- ============================================================================
-- TABLAS: ENVÍO
-- ============================================================================

CREATE TABLE envio (
    id_envio SERIAL PRIMARY KEY,
    id_venta INTEGER NOT NULL,
    empresa_envio VARCHAR(100),  -- DHL, Olva Courier, Shalom, etc
    metodo_envio VARCHAR(100),
    numero_tracking VARCHAR(100),
    costo DECIMAL(10,2) DEFAULT 0,
    estado estado_envio DEFAULT 'pendiente',
    -- Dirección (snapshot)
    direccion_completa TEXT,
    ciudad VARCHAR(100),
    departamento VARCHAR(100),
    codigo_postal VARCHAR(20),
    telefono_contacto VARCHAR(20),
    -- Fechas
    fecha_estimada DATE,
    fecha_envio TIMESTAMP,
    fecha_entrega TIMESTAMP,
    fecha_creacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    fecha_actualizacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    -- Tracking
    historial_tracking JSONB,  -- [{fecha, ubicacion, estado}]
    notas TEXT,
    
    FOREIGN KEY (id_venta) REFERENCES venta(id_venta) ON DELETE RESTRICT
);

CREATE INDEX idx_envio_venta ON envio(id_venta);
CREATE INDEX idx_envio_tracking ON envio(numero_tracking);
CREATE INDEX idx_envio_estado ON envio(estado);

-- ============================================================================
-- TABLAS: VALORACIONES Y REVIEWS
-- ============================================================================

CREATE TABLE valoracion (
    id_valoracion SERIAL PRIMARY KEY,
    id_producto INTEGER NOT NULL,  -- Se valora el tipo
    id_usuario INTEGER NOT NULL,
    id_producto_detalle INTEGER,  -- Qué marca/modelo compró
    id_venta INTEGER,  -- Verificación de compra
    calificacion INTEGER NOT NULL CHECK (calificacion BETWEEN 1 AND 5),
    titulo VARCHAR(200),
    comentario TEXT,
    -- Verificación
    compra_verificada BOOLEAN DEFAULT FALSE,
    -- Utilidad
    votos_util INTEGER DEFAULT 0,
    votos_no_util INTEGER DEFAULT 0,
    -- Estado
    aprobado BOOLEAN DEFAULT FALSE,
    fecha_creacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    fecha_actualizacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (id_producto) REFERENCES producto(id_producto) ON DELETE CASCADE,
    FOREIGN KEY (id_usuario) REFERENCES usuario(id_usuario) ON DELETE CASCADE,
    FOREIGN KEY (id_producto_detalle) REFERENCES producto_detalle(id_producto_detalle) ON DELETE SET NULL,
    FOREIGN KEY (id_venta) REFERENCES venta(id_venta) ON DELETE SET NULL,
    UNIQUE(id_usuario, id_producto)
);

CREATE INDEX idx_valoracion_producto ON valoracion(id_producto);
CREATE INDEX idx_valoracion_usuario ON valoracion(id_usuario);
CREATE INDEX idx_valoracion_calificacion ON valoracion(calificacion);
CREATE INDEX idx_valoracion_aprobado ON valoracion(aprobado);

-- ============================================================================

CREATE TABLE imagen_valoracion (
    id_imagen_valoracion SERIAL PRIMARY KEY,
    id_valoracion INTEGER NOT NULL,
    url_imagen VARCHAR(255) NOT NULL,
    orden INTEGER DEFAULT 0,
    fecha_subida TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (id_valoracion) REFERENCES valoracion(id_valoracion) ON DELETE CASCADE
);

CREATE INDEX idx_imagen_valoracion ON imagen_valoracion(id_valoracion);

-- ============================================================================
-- TABLAS: NOTIFICACIONES
-- ============================================================================

CREATE TABLE notificacion (
    id_notificacion SERIAL PRIMARY KEY,
    id_usuario INTEGER NOT NULL,
    tipo VARCHAR(50) NOT NULL,  -- pedido_confirmado, pedido_enviado, etc
    titulo VARCHAR(200) NOT NULL,
    mensaje TEXT NOT NULL,
    url VARCHAR(255),  -- Link a donde debe ir
    leida BOOLEAN DEFAULT FALSE,
    fecha_creacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    fecha_leida TIMESTAMP,
    
    FOREIGN KEY (id_usuario) REFERENCES usuario(id_usuario) ON DELETE CASCADE
);

CREATE INDEX idx_notificacion_usuario ON notificacion(id_usuario);
CREATE INDEX idx_notificacion_leida ON notificacion(id_usuario, leida) WHERE leida = FALSE;
CREATE INDEX idx_notificacion_fecha ON notificacion(fecha_creacion DESC);

-- ============================================================================
-- TABLAS: WISHLIST / FAVORITOS
-- ============================================================================

CREATE TABLE lista_deseos (
    id_lista_deseos SERIAL PRIMARY KEY,
    id_usuario INTEGER NOT NULL,
    id_producto_detalle INTEGER NOT NULL,
    fecha_agregado TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (id_usuario) REFERENCES usuario(id_usuario) ON DELETE CASCADE,
    FOREIGN KEY (id_producto_detalle) REFERENCES producto_detalle(id_producto_detalle) ON DELETE CASCADE,
    UNIQUE(id_usuario, id_producto_detalle)
);

CREATE INDEX idx_lista_deseos_usuario ON lista_deseos(id_usuario);
CREATE INDEX idx_lista_deseos_producto ON lista_deseos(id_producto_detalle);
