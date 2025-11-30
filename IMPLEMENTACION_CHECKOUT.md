# 📦 Implementación del Sistema de Checkout

## 🎯 Resumen

Se ha implementado un sistema completo de checkout transaccional y robusto para KronosTech E-Commerce, siguiendo las mejores prácticas de arquitectura en capas y manteniendo la coherencia con la estructura existente del proyecto.

---

## 🏗️ Arquitectura Implementada

### Backend (Rust + Axum + PostgreSQL)

```
├── models/
│   ├── direccion.rs          # Direcciones de envío
│   ├── venta.rs              # Pedidos y detalles de venta
│   └── metodo_pago.rs        # Métodos de pago y pagos
│
├── repositories/
│   ├── direccion_repository.rs   # Queries de direcciones
│   └── checkout_repository.rs    # Queries de checkout/ventas
│
├── services/
│   ├── direccion_service.rs      # Lógica de negocio direcciones
│   └── checkout_service.rs       # Lógica de negocio checkout
│
├── handlers/
│   ├── direccion_handler.rs      # HTTP handlers direcciones
│   └── checkout_handler.rs       # HTTP handlers checkout
│
└── routes/
    ├── direccion_routes.rs        # Rutas de direcciones
    └── checkout_routes.rs         # Rutas de checkout
```

### Frontend (SvelteKit + TypeScript)

```
frontend/src/lib/services/
├── direccion.ts      # API service para direcciones
└── checkout.ts       # API service para checkout y pedidos
```

---

## 📡 API Endpoints Implementados

### Direcciones

```
GET    /api/direcciones          # Listar direcciones del usuario
POST   /api/direcciones          # Crear nueva dirección
PUT    /api/direcciones/{id}     # Actualizar dirección
DELETE /api/direcciones/{id}     # Eliminar dirección (soft delete)
```

### Checkout y Pedidos

```
GET    /api/metodos-pago                 # Listar métodos de pago disponibles
GET    /api/checkout/calcular-total      # Calcular total con descuentos y envío
POST   /api/checkout/procesar            # Procesar checkout completo
GET    /api/pedidos                      # Listar pedidos del usuario
GET    /api/pedidos/{id}                 # Obtener detalle de pedido
```

---

## 🔐 Seguridad Implementada

### Autenticación y Autorización

- ✅ **JWT Token obligatorio** en todos los endpoints
- ✅ **Verificación de propiedad**: El usuario solo puede ver/modificar sus propias direcciones y pedidos
- ✅ **Validación de estado**: Direcciones y métodos de pago activos
- ✅ **Sanitización de datos**: Validación de tipos y campos requeridos

### Datos Sensibles

- ✅ **NUNCA se guardan datos reales de tarjeta**
- ✅ **Solo tokens de procesadores de pago** (preparado para Stripe, Culqi, etc.)
- ✅ **Direcciones son snapshot**: Se copian completas a la venta (no solo FK)

---

## ⚡ Lógica Transaccional (ACID)

### Proceso de Checkout

El método `procesar_checkout` es **completamente transaccional**:

```rust
// 1. Iniciar transacción
let mut tx = pool.begin().await?;

// 2. VALIDACIONES
//    - Dirección existe y está activa
//    - Método de pago existe y está activo
//    - Carrito no está vacío
//    - Stock disponible para TODOS los productos

// 3. CREAR VENTA
//    - Generar número de pedido único (PED-20251129-0001)
//    - Crear venta con snapshot de dirección
//    - Crear detalles de venta (snapshot de precios)

// 4. ACTUALIZAR INVENTARIO
//    - Restar stock de cada producto
//    - Incrementar total_vendidos

// 5. PROCESAR PAGO
//    - Crear registro de pago (simulado por ahora)
//    - Actualizar estado de venta y pago

// 6. CONVERTIR CARRITO
//    - Marcar carrito como 'convertido'

// 7. COMMIT o ROLLBACK
//    - Si todo OK: commit (todo se guarda)
//    - Si error: rollback (nada se guarda)
tx.commit().await?;
```

### Garantías ACID

- ✅ **Atomicidad**: Todo se guarda o nada se guarda
- ✅ **Consistencia**: Stock nunca negativo, datos coherentes
- ✅ **Aislamiento**: Transacciones no interfieren entre sí
- ✅ **Durabilidad**: Una vez confirmado, persiste en DB

---

## 💰 Cálculo de Totales

```typescript
subtotal = suma(item.precio_unitario * item.cantidad)
descuento_total = descuento_cupon + descuento_productos
costo_envio = (subtotal >= 100) ? 0 : 15
total = subtotal - descuento_total + costo_envio
```

### Reglas de Negocio Actuales

- 🚚 **Envío gratis** si compra >= S/. 100
- 🚚 **Envío S/. 15** si compra < S/. 100
- 💳 **Comisión de pago** calculada según método (porcentaje + fijo)

> **NOTA**: Costo de envío puede configurarse por admin (pendiente implementar panel admin)

---

## 🎟️ Sistema de Cupones (FUTURO)

Se ha dejado preparado el espacio para cupones con comentarios `TODO`:

### Backend

```rust
// En models/venta.rs
pub struct ProcesarCheckoutRequest {
    pub id_direccion: i32,
    pub id_metodo_pago: i32,
    pub notas_cliente: Option<String>,
    // FUTURO: Para cupones
    // pub codigo_cupon: Option<String>,
}

// En services/checkout_service.rs
// FUTURO: Validar y aplicar cupón
// TODO: Implementar validación de cupón (fecha vigencia, usos, compra mínima)
let descuento_total = Decimal::ZERO;

// FUTURO: Registrar uso de cupón si se aplicó
// TODO: Crear registro en tabla uso_cupon
```

### Frontend

```typescript
// En services/checkout.ts
export interface ProcesarCheckoutRequest {
    id_direccion: number;
    id_metodo_pago: number;
    notas_cliente?: string;
    // FUTURO: Para cupones
    // codigo_cupon?: string;
}

// Método preparado para validar cupón
// async validarCupon(codigo: string): Promise<CuponInfo> { ... }
```

### Pasos para Habilitar Cupones

1. **Backend**: Descomentar campos `codigo_cupon` en DTOs
2. **Backend**: Implementar `validar_y_calcular_descuento_cupon()` en service
3. **Backend**: Crear registro en tabla `uso_cupon` al procesar checkout
4. **Frontend**: Descomentar campo en request
5. **Frontend**: Agregar UI para ingresar código
6. **Frontend**: Llamar a `validarCupon()` antes de procesar

---

## 📊 Estados del Sistema

### Estado de Pedido (estado_pedido)

```
pendiente → confirmado → procesando → enviado → entregado

Casos especiales:
pendiente → cancelado
confirmado → cancelado (requiere reembolso)
```

### Estado de Pago (estado_pago)

```
pendiente → procesando → completado

Errores:
pendiente → fallido
completado → reembolsado
completado → parcialmente_reembolsado
```

---

## 🔄 Flujo Completo del Usuario

### 1. Carrito
- Usuario agrega productos al carrito
- Se valida stock disponible

### 2. Checkout - Dirección
- Usuario selecciona o crea dirección de envío
- Se calcula costo de envío basado en ubicación/monto

### 3. Checkout - Método de Pago
- Usuario selecciona método de pago
- Se muestra comisión si aplica

### 4. Checkout - Revisión
- Usuario revisa:
  - Productos y cantidades
  - Dirección de envío
  - Método de pago
  - Totales (subtotal, descuentos, envío, total)
- Puede aplicar cupón (futuro)

### 5. Procesamiento
- Se valida stock nuevamente
- Se crea pedido con snapshot de precios
- Se procesa pago (simulado)
- Se actualiza inventario
- Se convierte carrito

### 6. Confirmación
- Usuario recibe número de pedido
- Puede ver detalles del pedido
- Puede ver historial de pedidos

---

## 🎨 Diseño Visual (Pendiente)

**NOTA**: El usuario proporcionará el diseño de las pantallas. La implementación actual se enfoca en la lógica robusta.

Pantallas necesarias:
1. `/checkout/direccion` - Seleccionar/crear dirección
2. `/checkout/pago` - Seleccionar método de pago
3. `/checkout/revision` - Resumen y confirmación
4. `/pedido/[id]/confirmacion` - Confirmación de pedido
5. `/pedidos` - Lista de pedidos
6. `/pedido/[id]` - Detalle de pedido

---

## 🧪 Validaciones Implementadas

### Direcciones

- ✅ Tipo válido: "envio", "facturacion", "ambos"
- ✅ Campos requeridos: direccion_linea1, ciudad, departamento
- ✅ Solo una dirección predeterminada por usuario
- ✅ Usuario solo puede modificar sus propias direcciones

### Checkout

- ✅ Carrito no vacío
- ✅ Stock suficiente para TODOS los productos
- ✅ Dirección existe y está activa
- ✅ Método de pago existe y está activo
- ✅ Usuario autenticado

### Inventario

- ✅ Stock no puede ser negativo
- ✅ Actualización atómica del inventario
- ✅ Registro de movimientos de inventario

---

## 🚀 Próximos Pasos

### Backend

1. **Integrar procesador de pagos real** (Stripe, Culqi, MercadoPago)
   - Actualizar `crear_pago()` en repository
   - Manejar callbacks/webhooks del procesador
   - Implementar reintentos en caso de fallo

2. **Sistema de cupones**
   - Implementar validación de cupones
   - Registrar uso de cupones
   - Límites de uso por usuario/global

3. **Configuración de envío por admin**
   - Panel para configurar costos de envío
   - Reglas por departamento/ciudad
   - Opciones de envío (express, normal, etc.)

4. **Sistema de notificaciones**
   - Email de confirmación de pedido
   - Email de tracking de envío
   - Notificaciones push (opcional)

### Frontend

1. **Crear pantallas de checkout**
   - Wizard multi-paso o página única
   - Validación en tiempo real
   - Loading states

2. **Dashboard de pedidos**
   - Lista de pedidos con filtros
   - Detalle de pedido
   - Tracking de envío

3. **Gestión de direcciones**
   - Formulario de dirección
   - Lista de direcciones guardadas
   - Marcar como predeterminada

---

## 📝 Notas Importantes

### Para el Desarrollador

- **Todas las transacciones usan BEGIN/COMMIT/ROLLBACK**
- **Todos los precios se guardan como snapshot** (no se afectan por cambios posteriores)
- **La dirección se copia completa** a la venta (no solo FK)
- **Stock se valida 2 veces**: antes de calcular total y antes de confirmar
- **Carrito se marca como 'convertido'**, no se elimina (para histórico)

### Pruebas Recomendadas

1. ✅ Probar checkout con stock insuficiente
2. ✅ Probar checkout con carrito vacío
3. ✅ Probar con dirección inactiva
4. ✅ Probar con método de pago inactivo
5. ✅ Probar sin autenticación
6. ✅ Probar transacción con error (debe hacer rollback)
7. ✅ Verificar que el stock se actualiza correctamente
8. ✅ Verificar que el carrito se convierte
9. ✅ Verificar snapshot de precios (cambiar precio de producto después)
10. ✅ Verificar cálculo de envío

---

## 📞 Soporte

Para dudas sobre la implementación, revisar:
- Código fuente con comentarios detallados
- Documentación de la base de datos (`documentacion/assets/scripts/ddl.sql`)
- Este archivo `IMPLEMENTACION_CHECKOUT.md`

---

**✨ Implementación completada con éxito - Lista para integrarse con el diseño visual**
