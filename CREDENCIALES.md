# Credenciales de Prueba - KronosTech

Este archivo contiene las credenciales de acceso para los usuarios de prueba del sistema.

**IMPORTANTE:** Este archivo es solo para desarrollo/pruebas locales. Nunca subir credenciales reales a repositorios públicos.

---

## Usuarios del Sistema

### 🔐 Administradores

| Nombre | Email | Contraseña | Rol | Acceso |
|--------|-------|------------|-----|--------|
| Carlos Ramirez | `admin@kronostech.pe` | `admin123` | Super Admin | `/admin` |
| Maria Lopez | `maria.lopez@kronostech.pe` | `maria123` | Administrador | `/admin` |

**Permisos Super Admin (Carlos):**
- ✅ Gestión de usuarios del sistema
- ✅ Habilitación/deshabilitación de administradores
- ✅ Configuración del sistema
- ✅ Acceso a logs y auditoría
- ✅ Control de roles y permisos
- 🎨 Badge morado con gradiente "Super Administrador"
- ❌ NO gestiona operaciones del día a día (pedidos, inventario, etc.)

**Permisos Administrador (Maria):**
- ✅ Gestión de pedidos (ver, actualizar estado)
- ✅ Gestión de inventario (stock, movimientos)
- ✅ Gestión de descuentos y cupones
- ✅ Gestión de reembolsos
- ✅ Reportes de ventas e inventario
- 🎨 Badge azul "Administrador"
- ❌ NO puede gestionar usuarios ni administradores
- ❌ NO puede acceder a configuración del sistema

---

### 👤 Clientes

| Nombre | Email | Contraseña | Acceso |
|--------|-------|------------|--------|
| Juan Pérez | `juan.perez@gmail.com` | `juan123` | `/cuenta` |
| Ana García | `ana.garcia@outlook.com` | `ana123` | `/cuenta` |
| Pedro Martínez | `pedro.martinez@hotmail.com` | `pedro123` | `/cuenta` |

**Permisos Cliente:**
- ✅ Comprar productos (carrito y checkout)
- ✅ Gestionar direcciones de envío
- ✅ Gestionar métodos de pago
- ✅ Ver historial de pedidos
- ✅ Editar perfil y contraseña
- 🎨 Badge verde "Cliente"
- ❌ NO puede acceder a funciones administrativas

---

## Cómo usar estas credenciales

### 1. Cargar los datos en la base de datos

```bash
# Asegúrate de que PostgreSQL esté corriendo
# Carga el script DML con las credenciales actualizadas
psql -U postgres -d kronosdb -f documentacion/assets/scripts/dml.sql
```

### 2. Iniciar sesión en la aplicación

**Frontend:**
```
http://localhost:5173
```

**API (login):**
```bash
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "admin@kronostech.pe",
    "password": "admin123"
  }'
```

---

## Seguridad

Las contraseñas están hasheadas usando **bcrypt** (algoritmo de hashing seguro y unidireccional).

- **NO** es posible "desencriptar" un hash bcrypt
- Cada hash incluye un "salt" aleatorio
- Las contraseñas se verifican comparando el hash, no almacenando texto plano

**Para producción:**
- Cambiar todas las contraseñas
- Usar contraseñas fuertes (mínimo 12 caracteres, combinando mayúsculas, minúsculas, números y símbolos)
- Habilitar autenticación de dos factores (2FA)
- Implementar políticas de expiración de contraseñas

---

## Regenerar hashes (si es necesario)

Si necesitas crear un nuevo hash para una contraseña:

```python
import bcrypt

# Genera hash para una nueva contraseña
password = "mi_nueva_contraseña"
hash_bytes = bcrypt.hashpw(password.encode(), bcrypt.gensalt())
print(hash_bytes.decode())
```

O usando el backend de Rust (agregar en `main.rs` temporalmente):

```rust
use bcrypt::{hash, DEFAULT_COST};

fn main() {
    let password = "mi_nueva_contraseña";
    let hashed = hash(password, DEFAULT_COST).unwrap();
    println!("Hash: {}", hashed);
}
```

---

**Última actualización:** 2024-12-05
