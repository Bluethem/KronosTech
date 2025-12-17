-- ============================================================================
-- SCRIPT DE POBLAMIENTO DE DATOS (DML)
-- E-COMMERCE DE COMPONENTES DE PC - KRONOSDB
-- ============================================================================

-- ============================================================================
-- 1. USUARIOS
-- ============================================================================
--
-- CREDENCIALES DE PRUEBA (Las contraseñas están hasheadas con bcrypt):
--
-- | Email                          | Contraseña | Rol          |
-- |--------------------------------|------------|--------------|
-- | admin@kronostech.pe            | admin123   | super_admin  |
-- | maria.lopez@kronostech.pe      | maria123   | administrador|
-- | juan.perez@gmail.com           | juan123    | cliente      |
-- | ana.garcia@outlook.com         | ana123     | cliente      |
-- | pedro.martinez@hotmail.com     | pedro123   | cliente      |
--
-- ============================================================================

INSERT INTO usuario (nombre, apellido, email, contrasena, telefono, dni, rol, email_verificado, activo) VALUES
-- Contraseña: admin123
('Carlos', 'Ramirez', 'admin@kronostech.pe', '$2b$12$vmVSHJ0ULIo44CkKg1J9V.WCg7MggsvonGmqmCFNfKM3siORkMsZu', '987654321', '12345678', 'super_admin', TRUE, TRUE),
-- Contraseña: maria123
('Maria', 'Lopez', 'maria.lopez@kronostech.pe', '$2b$12$VOWveI/2YEPg0lVR3b0tVOBoL4.5pEedN3mVuvc4IsGXqLuxl6rmy', '987654322', '23456789', 'administrador', TRUE, TRUE),
-- Contraseña: juan123
('Juan', 'Pérez', 'juan.perez@gmail.com', '$2b$12$WwspyHspjLCe3z4/d.Tapu1AP7yCD7d6rvr1kjSHBurl2HHXAks7K', '987654324', '45678901', 'cliente', TRUE, TRUE),
-- Contraseña: ana123
('Ana', 'García', 'ana.garcia@outlook.com', '$2b$12$MEYGAGiB/H7Vb5bD8ug9meAJAfsPbGFfcv1sf55IkoASdvIUvkc3i', '987654325', '56789012', 'cliente', TRUE, TRUE),
-- Contraseña: pedro123
('Pedro', 'Martínez', 'pedro.martinez@hotmail.com', '$2b$12$26vnJXmFtZWSIfGAnLZM8OVV6GhKmYQaHw1hwLOdZgPR3UrWHkzVm', '987654326', '67890123', 'cliente', TRUE, TRUE);

INSERT INTO administrador (id_usuario, es_super_admin, permisos) VALUES
(1, TRUE, '{"productos": true, "ventas": true, "usuarios": true, "reportes": true}'),
(2, FALSE, '{"productos": true, "ventas": true, "inventario": true}');

-- ============================================================================
-- 2. DIRECCIONES
-- ============================================================================

INSERT INTO direccion (id_usuario, tipo, nombre_completo, direccion_linea1, ciudad, departamento, codigo_postal, telefono_contacto, referencia, es_predeterminada) VALUES
(3, 'ambos', 'Juan Pérez', 'Av. Javier Prado Este 4567', 'Lima', 'Lima', '15036', '987654324', 'Edificio azul', TRUE),
(4, 'envio', 'Ana García', 'Calle Los Olivos 234', 'San Isidro', 'Lima', '15073', '987654325', 'Casa blanca', TRUE),
(5, 'ambos', 'Pedro Martínez', 'Av. Benavides 1890', 'Miraflores', 'Lima', '15074', '987654326', 'Torre B', TRUE);

-- ============================================================================
-- 3. FAMILIAS, CATEGORÍAS Y SUBCATEGORÍAS
-- ============================================================================

INSERT INTO familia (nombre, descripcion, orden, estado) VALUES
('Componentes Internos', 'Componentes principales para PC', 1, 'activo'),
('Almacenamiento', 'SSD, HDD y NVMe', 2, 'activo'),
('Periféricos', 'Teclados, mouse, audífonos', 3, 'activo'),
('Monitores', 'Monitores gaming y profesionales', 4, 'activo');

INSERT INTO categoria (nombre, descripcion, id_familia, orden, estado) VALUES
('Procesadores', 'CPUs Intel y AMD', 1, 1, 'activo'),
('Tarjetas Gráficas', 'GPUs NVIDIA y AMD', 1, 2, 'activo'),
('Memoria RAM', 'DDR4 y DDR5', 1, 4, 'activo'),
('SSD', 'Unidades de estado sólido', 2, 1, 'activo'),
('Teclados', 'Teclados mecánicos y gaming', 3, 1, 'activo'),
('Mouse', 'Mouse gaming', 3, 2, 'activo');

INSERT INTO subcategoria (nombre, id_categoria, orden, estado) VALUES
('Intel Core i5', 1, 1, 'activo'),
('Intel Core i7', 1, 2, 'activo'),
('AMD Ryzen 5', 1, 4, 'activo'),
('AMD Ryzen 7', 1, 5, 'activo'),
('NVIDIA RTX 4060', 2, 1, 'activo'),
('NVIDIA RTX 4070', 2, 2, 'activo');

-- ============================================================================
-- 4. MARCAS
-- ============================================================================

INSERT INTO marca (nombre, slug, pais_origen, estado) VALUES
('Intel', 'intel', 'Estados Unidos', 'activo'),
('AMD', 'amd', 'Estados Unidos', 'activo'),
('NVIDIA', 'nvidia', 'Estados Unidos', 'activo'),
('ASUS', 'asus', 'Taiwán', 'activo'),
('MSI', 'msi', 'Taiwán', 'activo'),
('Corsair', 'corsair', 'Estados Unidos', 'activo'),
('Samsung', 'samsung', 'Corea del Sur', 'activo'),
('Logitech', 'logitech', 'Suiza', 'activo'),
('Razer', 'razer', 'Estados Unidos', 'activo');

-- ============================================================================
-- 5. PRODUCTOS
-- ============================================================================

INSERT INTO producto (nombre, descripcion, id_categoria, id_subcategoria, especificaciones_base, valoracion_promedio, total_valoraciones, estado) VALUES
('Intel Core i5-13600K', 'Procesador Intel Core i5 13ª gen con 14 núcleos', 1, 1, '{"socket": "LGA1700", "nucleos": 14, "hilos": 20}', 4.5, 32, 'activo'),
('Intel Core i7-13700K', 'Procesador Intel Core i7 13ª gen con 16 núcleos', 1, 2, '{"socket": "LGA1700", "nucleos": 16, "hilos": 24}', 4.7, 45, 'activo'),
('AMD Ryzen 5 7600X', 'Procesador AMD Ryzen 5 Zen 4', 1, 3, '{"socket": "AM5", "nucleos": 6, "hilos": 12}', 4.6, 38, 'activo'),
('AMD Ryzen 7 7700X', 'Procesador AMD Ryzen 7 Zen 4', 1, 4, '{"socket": "AM5", "nucleos": 8, "hilos": 16}', 4.7, 52, 'activo'),
('NVIDIA RTX 4060', 'GPU RTX 4060 8GB GDDR6', 2, 5, '{"memoria": "8GB GDDR6", "cuda_cores": 3072}', 4.4, 67, 'activo'),
('NVIDIA RTX 4070', 'GPU RTX 4070 12GB GDDR6X', 2, 6, '{"memoria": "12GB GDDR6X", "cuda_cores": 5888}', 4.7, 89, 'activo'),
('Corsair Vengeance DDR5', 'Memoria RAM DDR5 alto rendimiento', 3, NULL, '{"tipo": "DDR5", "frecuencia": "5600 MHz"}', 4.6, 78, 'activo'),
('Samsung 980 PRO', 'SSD NVMe PCIe 4.0', 4, NULL, '{"interfaz": "NVMe PCIe 4.0", "lectura": "7000 MB/s"}', 4.8, 124, 'activo'),
('Logitech G Pro X', 'Teclado mecánico gaming', 5, NULL, '{"tipo_switch": "GX", "iluminacion": "RGB"}', 4.6, 89, 'activo'),
('Razer DeathAdder V3', 'Mouse gaming ergonómico', 6, NULL, '{"sensor": "Focus Pro 30K", "dpi_max": 30000}', 4.7, 112, 'activo');

-- ============================================================================
-- 6. PRODUCTOS DETALLE
-- ============================================================================

INSERT INTO producto_detalle (nombre, modelo, sku, id_producto, id_marca, precio_base, precio_venta, costo, peso, garantia_meses, es_destacado, es_nuevo, estado, imagen_principal) VALUES
-- CPUs Intel (imagen de procesador)
('Intel Core i5-13600K Box', 'BX8071513600K', 'CPU-INTL-I5-13600K', 1, 1, 1599.00, 1499.00, 1200.00, 0.5, 36, TRUE, FALSE, 'activo', 'https://images.unsplash.com/photo-1591799264318-7e6ef8ddb7ea?w=400&h=300&fit=crop'),
('Intel Core i7-13700K Box', 'BX8071513700K', 'CPU-INTL-I7-13700K', 2, 1, 2199.00, 2099.00, 1700.00, 0.5, 36, TRUE, FALSE, 'activo', 'https://images.unsplash.com/photo-1555617981-dac3880eac6e?w=400&h=300&fit=crop'),
-- CPUs AMD (imagen de procesador AMD)
('AMD Ryzen 5 7600X Box', '100-100000593WOF', 'CPU-AMD-R5-7600X', 3, 2, 1399.00, 1299.00, 1000.00, 0.5, 36, FALSE, FALSE, 'activo', 'https://images.unsplash.com/photo-1558618666-fcd25c85cd64?w=400&h=300&fit=crop'),
('AMD Ryzen 7 7700X Box', '100-100000591WOF', 'CPU-AMD-R7-7700X', 4, 2, 1899.00, 1799.00, 1450.00, 0.5, 36, TRUE, FALSE, 'activo', 'https://images.unsplash.com/photo-1591799264318-7e6ef8ddb7ea?w=400&h=300&fit=crop'),
-- GPUs (imágenes de tarjetas gráficas)
('ASUS RTX 4060 TUF Gaming', 'TUF-RTX4060-O8G-GAMING', 'GPU-ASUS-RTX4060-TUF', 5, 4, 1899.00, 1799.00, 1400.00, 1.2, 36, TRUE, FALSE, 'activo', 'https://images.unsplash.com/photo-1591488320449-011701bb6704?w=400&h=300&fit=crop'),
('ASUS RTX 4070 ROG Strix', 'ROG-STRIX-RTX4070-O12G', 'GPU-ASUS-RTX4070-STRIX', 6, 4, 3299.00, 3199.00, 2600.00, 1.5, 36, TRUE, TRUE, 'activo', 'https://images.unsplash.com/photo-1587202372775-e229f172b9d7?w=400&h=300&fit=crop'),
('MSI RTX 4060 Gaming X', 'RTX 4060 GAMING X 8G', 'GPU-MSI-RTX4060-GX', 5, 5, 1849.00, 1749.00, 1350.00, 1.1, 36, FALSE, FALSE, 'activo', 'https://images.unsplash.com/photo-1623820919239-0d0ff10797a1?w=400&h=300&fit=crop'),
('MSI RTX 4070 Gaming X Trio', 'RTX 4070 GAMING X TRIO 12G', 'GPU-MSI-RTX4070-GXT', 6, 5, 3199.00, 3099.00, 2500.00, 1.4, 36, TRUE, FALSE, 'activo', 'https://images.unsplash.com/photo-1591488320449-011701bb6704?w=400&h=300&fit=crop'),
-- RAM Corsair (imágenes de memoria RAM)
('Corsair Vengeance DDR5 32GB', 'CMK32GX5M2B5600C36', 'RAM-CORS-DDR5-32GB', 7, 6, 699.00, 649.00, 500.00, 0.2, 24, FALSE, FALSE, 'activo', 'https://images.unsplash.com/photo-1562976540-1502c2145186?w=400&h=300&fit=crop'),
('Corsair Vengeance DDR5 16GB', 'CMK16GX5M2B5600C36', 'RAM-CORS-DDR5-16GB', 7, 6, 399.00, 349.00, 270.00, 0.1, 24, TRUE, FALSE, 'activo', 'https://images.unsplash.com/photo-1562976540-1502c2145186?w=400&h=300&fit=crop'),
-- SSD Samsung (imágenes de SSD/almacenamiento)
('Samsung 980 PRO 1TB', 'MZ-V8P1T0BW', 'SSD-SAM-980PRO-1TB', 8, 7, 699.00, 649.00, 500.00, 0.08, 60, TRUE, FALSE, 'activo', 'https://images.unsplash.com/photo-1597872200969-2b65d56bd16b?w=400&h=300&fit=crop'),
('Samsung 980 PRO 2TB', 'MZ-V8P2T0BW', 'SSD-SAM-980PRO-2TB', 8, 7, 1199.00, 1149.00, 900.00, 0.08, 60, TRUE, FALSE, 'activo', 'https://images.unsplash.com/photo-1597872200969-2b65d56bd16b?w=400&h=300&fit=crop'),
-- Teclado gaming
('Logitech G Pro X TKL', 'G-PKB-003', 'KBD-LOGI-GPROX-TKL', 9, 8, 799.00, 749.00, 600.00, 0.8, 24, TRUE, FALSE, 'activo', 'https://images.unsplash.com/photo-1587829741301-dc798b83add3?w=400&h=300&fit=crop'),
-- Mouse gaming
('Razer DeathAdder V3 Pro', 'RZ01-04630100-R3U1', 'MOU-RAZ-DAV3-PRO', 10, 9, 649.00, 599.00, 450.00, 0.06, 24, TRUE, FALSE, 'activo', 'https://images.unsplash.com/photo-1527864550417-7fd91fc51a46?w=400&h=300&fit=crop');

-- ============================================================================
-- 7. INVENTARIO
-- ============================================================================

INSERT INTO inventario (id_producto_detalle, cantidad_disponible, cantidad_minima, cantidad_maxima, ubicacion_fisica) VALUES
(1, 25, 5, 100, 'A-01-CPU'),
(2, 18, 5, 100, 'A-01-CPU'),
(3, 30, 5, 100, 'A-02-CPU'),
(4, 22, 5, 100, 'A-02-CPU'),
(5, 15, 3, 50, 'B-01-GPU'),
(6, 12, 3, 50, 'B-01-GPU'),
(7, 8, 2, 30, 'B-02-GPU'),
(8, 10, 2, 30, 'B-02-GPU'),
(9, 45, 10, 200, 'C-01-RAM'),
(10, 60, 10, 200, 'C-01-RAM'),
(11, 35, 8, 150, 'D-01-SSD'),
(12, 20, 5, 100, 'D-01-SSD'),
(13, 28, 5, 100, 'E-01-PER'),
(14, 32, 5, 100, 'E-02-PER');

-- ============================================================================
-- 8. DESCUENTOS
-- ============================================================================

INSERT INTO descuento (nombre, descripcion, tipo_descuento, valor, aplica_a, id_referencia, fecha_inicio, fecha_fin, activo) VALUES
('Black Friday GPUs', 'Descuento Black Friday en tarjetas gráficas', 'porcentaje', 15.00, 'categoria', 2, '2024-11-15 00:00:00', '2024-12-01 23:59:59', TRUE),
('Cyber Monday RAM', 'Descuento Cyber Monday en memorias', 'porcentaje', 10.00, 'categoria', 3, '2024-12-02 00:00:00', '2024-12-09 23:59:59', TRUE),
('Descuento AMD', 'Promoción en procesadores AMD', 'porcentaje', 8.00, 'marca', 2, '2024-11-01 00:00:00', '2024-12-31 23:59:59', TRUE);

-- ============================================================================
-- 9. CUPONES
-- ============================================================================

INSERT INTO cupon (codigo, nombre, descripcion, tipo_cupon, valor, aplica_a, compra_minima, usos_maximos, usos_maximos_por_usuario, fecha_inicio, fecha_fin, activo) VALUES
('BIENVENIDO10', 'Bienvenida nuevos clientes', '10% descuento primera compra', 'porcentaje', 10.00, 'todo', 500.00, NULL, 1, '2024-01-01 00:00:00', '2025-12-31 23:59:59', TRUE),
('NAVIDAD2024', 'Promoción Navidad', 'S/. 50 descuento compras +S/.1000', 'monto_fijo', 50.00, 'todo', 1000.00, 100, 1, '2024-12-01 00:00:00', '2024-12-26 23:59:59', TRUE),
('GAMING100', 'Gaming Premium', 'S/. 100 descuento componentes gaming', 'monto_fijo', 100.00, 'familia', 2000.00, 50, 1, '2024-11-01 00:00:00', '2024-12-31 23:59:59', TRUE),
('ENVIOGRATIS', 'Envío gratis Lima', 'Envío gratis compras +S/.500', 'envio_gratis', 0.00, 'todo', 500.00, NULL, 3, '2024-01-01 00:00:00', '2025-12-31 23:59:59', TRUE);

-- ============================================================================
-- 10. CARRITOS DE EJEMPLO
-- ============================================================================

INSERT INTO carrito (id_usuario, estado, fecha_expiracion) VALUES
(3, 'activo', NOW() + INTERVAL '7 days'),
(4, 'activo', NOW() + INTERVAL '7 days');

INSERT INTO carrito_detalle (id_carrito, id_producto_detalle, cantidad, precio_unitario) VALUES
(1, 1, 1, 1499.00),
(1, 5, 1, 1799.00),
(1, 10, 2, 349.00),
(2, 7, 1, 3199.00),
(2, 11, 1, 649.00);

-- ============================================================================
-- 11. VENTAS DE EJEMPLO
-- ============================================================================

-- Ventas con fechas dinámicas (recientes) para que aparezcan en reportes
INSERT INTO venta (numero_pedido, id_usuario, subtotal, descuento_total, costo_envio, total, estado, estado_pago, direccion_envio, ciudad, departamento, telefono_contacto, metodo_envio, fecha_pedido, fecha_pago, fecha_confirmacion) VALUES
('PED-20241110-00000001', 3, 2599.00, 0.00, 25.00, 2624.00, 'entregado', 'completado', 'Av. Javier Prado Este 4567', 'Lima', 'Lima', '987654324', 'Envío Express', CURRENT_TIMESTAMP - INTERVAL '15 days', CURRENT_TIMESTAMP - INTERVAL '15 days' + INTERVAL '5 minutes', CURRENT_TIMESTAMP - INTERVAL '15 days' + INTERVAL '10 minutes'),
('PED-20241112-00000002', 4, 3848.00, 192.40, 25.00, 3680.60, 'procesando', 'completado', 'Calle Los Olivos 234', 'San Isidro', 'Lima', '987654325', 'Envío Estándar', CURRENT_TIMESTAMP - INTERVAL '10 days', CURRENT_TIMESTAMP - INTERVAL '10 days' + INTERVAL '5 minutes', CURRENT_TIMESTAMP - INTERVAL '10 days' + INTERVAL '10 minutes'),
('PED-20241114-00000003', 5, 1499.00, 0.00, 25.00, 1524.00, 'pendiente', 'pendiente', 'Av. Benavides 1890', 'Miraflores', 'Lima', '987654326', 'Envío Express', CURRENT_TIMESTAMP - INTERVAL '5 days', NULL, NULL),
('PED-20241215-00000004', 3, 1799.00, 0.00, 25.00, 1824.00, 'confirmado', 'completado', 'Av. Javier Prado Este 4567', 'Lima', 'Lima', '987654324', 'Envío Express', CURRENT_TIMESTAMP - INTERVAL '2 days', CURRENT_TIMESTAMP - INTERVAL '2 days' + INTERVAL '5 minutes', CURRENT_TIMESTAMP - INTERVAL '2 days' + INTERVAL '10 minutes'),
('PED-20241216-00000005', 4, 649.00, 0.00, 15.00, 664.00, 'enviado', 'completado', 'Calle Los Olivos 234', 'San Isidro', 'Lima', '987654325', 'Envío Estándar', CURRENT_TIMESTAMP - INTERVAL '1 day', CURRENT_TIMESTAMP - INTERVAL '1 day' + INTERVAL '5 minutes', CURRENT_TIMESTAMP - INTERVAL '1 day' + INTERVAL '10 minutes');

INSERT INTO detalle_venta (id_venta, id_producto_detalle, id_producto, cantidad, precio_unitario, descuento_unitario, precio_final, subtotal) VALUES
-- Venta 1: Procesador + RAM + SSD
(1, 3, 3, 1, 1299.00, 0.00, 1299.00, 1299.00),
(1, 10, 7, 2, 349.00, 0.00, 349.00, 698.00),
(1, 14, 10, 1, 599.00, 0.00, 599.00, 599.00),
-- Venta 2: GPU + RAM
(2, 7, 6, 1, 3199.00, 159.95, 3039.05, 3039.05),
(2, 11, 8, 1, 649.00, 32.45, 616.55, 616.55),
-- Venta 3: Procesador
(3, 1, 1, 1, 1499.00, 0.00, 1499.00, 1499.00),
-- Venta 4: Placa madre
(4, 5, 4, 1, 1799.00, 0.00, 1799.00, 1799.00),
-- Venta 5: RAM
(5, 11, 8, 1, 649.00, 0.00, 649.00, 649.00);

-- ============================================================================
-- 12. VALORACIONES
-- ============================================================================

INSERT INTO valoracion (id_producto, id_usuario, id_producto_detalle, id_venta, calificacion, titulo, comentario, compra_verificada, aprobado) VALUES
(3, 3, 3, 1, 5, 'Excelente procesador', 'Muy buen rendimiento para gaming y trabajo. Recomendado al 100%.', TRUE, TRUE),
(7, 3, 10, 1, 4, 'Buena RAM', 'Funciona perfecto, buen precio calidad.', TRUE, TRUE),
(6, 4, 7, 2, 5, 'Increíble GPU', 'Corre todos los juegos en ultra a 1440p sin problemas. Vale cada sol.', TRUE, TRUE);

-- ============================================================================
-- 13. NOTIFICACIONES
-- ============================================================================

INSERT INTO notificacion (id_usuario, tipo, titulo, mensaje, leida) VALUES
(3, 'pedido_entregado', 'Pedido entregado', 'Tu pedido PED-20241110-00000001 ha sido entregado exitosamente', TRUE),
(4, 'pedido_confirmado', 'Pedido confirmado', 'Tu pedido PED-20241112-00000002 ha sido confirmado y está en proceso', FALSE),
(5, 'pedido_pendiente_pago', 'Completa tu pago', 'Tu pedido PED-20241114-00000003 está pendiente de pago', FALSE);

INSERT INTO metodo_pago (nombre, tipo, proveedor, descripcion, comision_porcentaje, tiempo_procesamiento, orden) VALUES
('Tarjeta de Crédito Visa', 'tarjeta_credito', 'Visa', 'Pago con tarjeta Visa', 2.9, 'Inmediato', 1),
('Tarjeta de Crédito Mastercard', 'tarjeta_credito', 'Mastercard', 'Pago con tarjeta Mastercard', 2.9, 'Inmediato', 2),
('Tarjeta de Débito', 'tarjeta_debito', 'Visa/Mastercard', 'Pago con tarjeta de débito', 2.5, 'Inmediato', 3),
('Yape', 'billetera_digital', 'Yape', 'Pago mediante Yape', 0, 'Inmediato', 4),
('Plin', 'billetera_digital', 'Plin', 'Pago mediante Plin', 0, 'Inmediato', 5),
('Transferencia Bancaria', 'transferencia', NULL, 'Transferencia desde cualquier banco', 0, '1-2 días', 6),
('Contra Reembolso', 'contrareembolso', NULL, 'Pago al recibir el producto', 0, 'Al entregar', 7);

-- ============================================================================
-- 14. CONFIGURACIÓN DEL SISTEMA
-- ============================================================================

INSERT INTO configuracion_sistema (clave, valor, tipo, descripcion, categoria) VALUES
-- General
('site_name', 'KronosTech', 'string', 'Nombre del sitio', 'general'),
('site_description', 'Tu tienda de tecnología de confianza', 'string', 'Descripción del sitio', 'general'),
('support_email', 'soporte@kronostech.com', 'string', 'Email de soporte', 'general'),
('contact_phone', '+51 999 999 999', 'string', 'Teléfono de contacto', 'general'),

-- E-commerce
('currency', 'PEN', 'string', 'Moneda del sistema', 'ecommerce'),
('tax_rate', '18', 'number', 'Tasa de impuesto (IGV)', 'ecommerce'),
('free_shipping_threshold', '100', 'number', 'Umbral para envío gratis (S/.)', 'ecommerce'),
('low_stock_threshold', '10', 'number', 'Umbral de alerta de stock bajo', 'ecommerce'),

-- Envío
('default_shipping_cost', '15', 'number', 'Costo de envío estándar (S/.)', 'envio'),
('express_shipping_cost', '35', 'number', 'Costo de envío express (S/.)', 'envio'),
('estimated_delivery_days', '3', 'number', 'Días estimados de entrega', 'envio'),

-- Seguridad
('session_timeout', '24', 'number', 'Duración de sesión en horas', 'seguridad'),
('max_login_attempts', '5', 'number', 'Máximo intentos de login antes de bloqueo', 'seguridad'),
('password_min_length', '6', 'number', 'Longitud mínima de contraseña', 'seguridad'),

-- Funcionalidades
('allow_guest_checkout', 'true', 'boolean', 'Permitir compra sin registro', 'funcionalidades'),
('require_email_verification', 'false', 'boolean', 'Requerir verificación de email', 'funcionalidades'),
('enable_reviews', 'true', 'boolean', 'Habilitar valoraciones', 'funcionalidades'),
('enable_wishlist', 'true', 'boolean', 'Habilitar lista de deseos', 'funcionalidades'),
('enable_coupons', 'true', 'boolean', 'Habilitar cupones', 'funcionalidades'),

-- Email
('email_enabled', 'true', 'boolean', 'Habilitar envío de emails', 'email'),
('smtp_host', 'smtp.gmail.com', 'string', 'Servidor SMTP', 'email'),
('smtp_port', '587', 'number', 'Puerto SMTP', 'email'),
('smtp_user', 'noreply@kronostech.com', 'string', 'Usuario SMTP', 'email');

-- ============================================================================
-- 15. LOGS DE AUDITORÍA (DATOS DE PRUEBA)
-- ============================================================================

INSERT INTO log_auditoria (nivel, accion, detalles, modulo, id_usuario, email_usuario, ip_cliente, fecha_creacion) VALUES
-- Logs de autenticación
('success', 'Login exitoso', 'Usuario admin@kronostech.pe (super_admin) inició sesión', 'Autenticación', 1, 'admin@kronostech.pe', '192.168.1.100', CURRENT_TIMESTAMP - INTERVAL '2 hours'),
('success', 'Login exitoso', 'Usuario maria.lopez@kronostech.pe (administrador) inició sesión', 'Autenticación', 2, 'maria.lopez@kronostech.pe', '192.168.1.101', CURRENT_TIMESTAMP - INTERVAL '1 hour 45 minutes'),
('success', 'Login exitoso', 'Usuario juan.perez@gmail.com (cliente) inició sesión', 'Autenticación', 3, 'juan.perez@gmail.com', '201.234.56.78', CURRENT_TIMESTAMP - INTERVAL '1 hour 30 minutes'),
('warning', 'Intento de login fallido', 'Intento fallido de login para usuario@inexistente.com: Usuario no encontrado', 'Seguridad', NULL, 'usuario@inexistente.com', '45.67.89.10', CURRENT_TIMESTAMP - INTERVAL '1 hour 15 minutes'),
('success', 'Nuevo registro', 'Nuevo usuario registrado: ana.garcia@outlook.com (Ana García)', 'Autenticación', 4, 'ana.garcia@outlook.com', '189.123.45.67', CURRENT_TIMESTAMP - INTERVAL '1 hour'),

-- Logs de sistema
('info', 'Sistema iniciado', 'Panel de administración cargado correctamente', 'Sistema', 1, 'admin@kronostech.pe', '192.168.1.100', CURRENT_TIMESTAMP - INTERVAL '55 minutes'),
('info', 'Configuración actualizada', 'Parámetros de envío actualizados: costo estándar S/. 15, express S/. 30', 'Configuración', 1, 'admin@kronostech.pe', '192.168.1.100', CURRENT_TIMESTAMP - INTERVAL '50 minutes'),
('success', 'Backup completado', 'Backup de base de datos completado exitosamente (45 MB)', 'Sistema', NULL, 'Sistema', NULL, CURRENT_TIMESTAMP - INTERVAL '45 minutes'),

-- Logs de usuarios
('info', 'Usuario actualizado', 'Se actualizó el rol del usuario juan.perez@gmail.com a cliente', 'Usuarios', 1, 'admin@kronostech.pe', '192.168.1.100', CURRENT_TIMESTAMP - INTERVAL '40 minutes'),
('warning', 'Usuario desactivado', 'Se desactivó temporalmente la cuenta de pedro.martinez@hotmail.com', 'Usuarios', 1, 'admin@kronostech.pe', '192.168.1.100', CURRENT_TIMESTAMP - INTERVAL '35 minutes'),

-- Logs de ventas
('success', 'Venta completada', 'Pedido PED-20241215-00000001 completado por S/. 1,250.00', 'Ventas', 3, 'juan.perez@gmail.com', '201.234.56.78', CURRENT_TIMESTAMP - INTERVAL '30 minutes'),
('info', 'Pedido enviado', 'Pedido PED-20241215-00000001 marcado como enviado. Tracking: TRK123456', 'Ventas', 2, 'maria.lopez@kronostech.pe', '192.168.1.101', CURRENT_TIMESTAMP - INTERVAL '25 minutes'),

-- Logs de inventario
('warning', 'Stock bajo', 'Producto SKU-AMD-R5-5600X tiene stock bajo: 3 unidades disponibles', 'Inventario', NULL, 'Sistema', NULL, CURRENT_TIMESTAMP - INTERVAL '20 minutes'),
('info', 'Movimiento de inventario', 'Entrada de 50 unidades de SKU-NVIDIA-RTX4070. Stock actualizado: 75', 'Inventario', 2, 'maria.lopez@kronostech.pe', '192.168.1.101', CURRENT_TIMESTAMP - INTERVAL '15 minutes'),

-- Logs de productos
('success', 'Producto creado', 'Nuevo producto agregado: AMD Ryzen 7 7800X3D (SKU: SKU-AMD-R7-7800X3D)', 'Productos', 2, 'maria.lopez@kronostech.pe', '192.168.1.101', CURRENT_TIMESTAMP - INTERVAL '10 minutes'),
('info', 'Precio actualizado', 'Precio del producto SKU-NVIDIA-RTX4090 actualizado de S/. 8,999.00 a S/. 8,499.00', 'Productos', 2, 'maria.lopez@kronostech.pe', '192.168.1.101', CURRENT_TIMESTAMP - INTERVAL '5 minutes'),

-- Logs de seguridad
('security', 'Múltiples intentos fallidos', 'Se detectaron 5 intentos fallidos de login desde IP 45.67.89.10', 'Seguridad', NULL, 'Sistema', '45.67.89.10', CURRENT_TIMESTAMP - INTERVAL '3 minutes'),
('error', 'Error de conexión', 'Error temporal de conexión con pasarela de pago. Reintentos: 3', 'Sistema', NULL, 'Sistema', NULL, CURRENT_TIMESTAMP - INTERVAL '1 minute');

-- ============================================================================
-- 16. PAGOS DE EJEMPLO
-- ============================================================================

INSERT INTO pago (id_venta, id_metodo_pago, numero_transaccion, estado, monto, moneda, comision, monto_neto, proveedor_pago, id_transaccion_proveedor, fecha_pago) VALUES
(1, 4, 'TXN-YAPE-20241110-001', 'completado', 2624.00, 'PEN', 0.00, 2624.00, 'Yape', 'YAPE_TXN_001', CURRENT_TIMESTAMP - INTERVAL '15 days'),
(2, 1, 'TXN-VISA-20241112-001', 'completado', 3680.60, 'PEN', 106.74, 3573.86, 'Culqi', 'CULQI_TXN_001', CURRENT_TIMESTAMP - INTERVAL '10 days'),
(3, 5, 'TXN-PLIN-20241114-001', 'pendiente', 1524.00, 'PEN', 0.00, 1524.00, 'Plin', NULL, NULL),
(4, 4, 'TXN-YAPE-20241215-001', 'completado', 1824.00, 'PEN', 0.00, 1824.00, 'Yape', 'YAPE_TXN_002', CURRENT_TIMESTAMP - INTERVAL '2 days'),
(5, 2, 'TXN-MC-20241216-001', 'completado', 664.00, 'PEN', 19.26, 644.74, 'Culqi', 'CULQI_TXN_002', CURRENT_TIMESTAMP - INTERVAL '1 day');

-- ============================================================================
-- 17. REEMBOLSOS DE EJEMPLO
-- ============================================================================

INSERT INTO reembolso (id_pago, id_venta, tipo_reembolso, monto_reembolsado, motivo, estado, id_usuario_solicitante, fecha_solicitado, fecha_aprobado, fecha_completado, notas_admin) VALUES
-- Reembolso completado
(1, 1, 'parcial', 599.00, 'El producto SSD Kingston llegó defectuoso, no enciende.', 'completado', 3, CURRENT_TIMESTAMP - INTERVAL '12 days', CURRENT_TIMESTAMP - INTERVAL '11 days', CURRENT_TIMESTAMP - INTERVAL '11 days' + INTERVAL '2 hours', 'Reembolso aprobado. Se verificó defecto con soporte técnico.'),
-- Reembolso solicitado (pendiente)
(2, 2, 'total', 3680.60, 'El pedido llegó incompleto, falta la memoria RAM.', 'solicitado', 4, CURRENT_TIMESTAMP - INTERVAL '2 days', NULL, NULL, NULL),
-- Otro reembolso solicitado
(2, 2, 'parcial', 616.55, 'La RAM llegó con un módulo dañado.', 'solicitado', 4, CURRENT_TIMESTAMP - INTERVAL '1 day', NULL, NULL, NULL);

-- ============================================================================
-- FIN DEL SCRIPT DML
-- ============================================================================