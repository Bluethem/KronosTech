use sqlx::PgPool;
use crate::models::MetodoPagoCliente;
use crate::repositories::MetodoPagoClienteRepository;

pub struct MetodoPagoClienteService;

impl MetodoPagoClienteService {
    /// Obtener todos los métodos de pago del cliente
    pub async fn get_user_payment_methods(
        pool: &PgPool,
        id_usuario: i32,
    ) -> Result<Vec<MetodoPagoCliente>, String> {
        MetodoPagoClienteRepository::get_by_user(pool, id_usuario)
            .await
            .map_err(|e| format!("Error al obtener métodos de pago: {}", e))
    }

    /// Crear un nuevo método de pago
    pub async fn create_payment_method(
        pool: &PgPool,
        id_usuario: i32,
        id_metodo_pago: i32,
        tipo: &str,
        token_pago: Option<&str>,
        ultimos_4_digitos: Option<&str>,
        marca: Option<&str>,
        fecha_expiracion: Option<&str>,
        nombre_titular: Option<&str>,
        es_predeterminado: bool,
    ) -> Result<MetodoPagoCliente, String> {
        // Validaciones
        if let Some(digitos) = ultimos_4_digitos {
            if digitos.len() != 4 {
                return Err("Los últimos 4 dígitos deben ser exactamente 4 caracteres".to_string());
            }
        }

        if let Some(expiracion) = fecha_expiracion {
            // Validar formato MM/YY
            if !expiracion.contains('/') || expiracion.len() != 5 {
                return Err("La fecha de expiración debe tener formato MM/YY".to_string());
            }
        }

        MetodoPagoClienteRepository::create(
            pool,
            id_usuario,
            id_metodo_pago,
            tipo,
            token_pago,
            ultimos_4_digitos,
            marca,
            fecha_expiracion,
            nombre_titular,
            es_predeterminado,
        )
        .await
        .map_err(|e| format!("Error al crear método de pago: {}", e))
    }

    /// Actualizar un método de pago
    pub async fn update_payment_method(
        pool: &PgPool,
        id_metodo_pago_cliente: i32,
        id_usuario: i32,
        ultimos_4_digitos: Option<&str>,
        marca: Option<&str>,
        fecha_expiracion: Option<&str>,
        nombre_titular: Option<&str>,
        es_predeterminado: Option<bool>,
    ) -> Result<MetodoPagoCliente, String> {
        // Verificar que el método de pago pertenece al usuario
        let belongs = MetodoPagoClienteRepository::belongs_to_user(pool, id_metodo_pago_cliente, id_usuario)
            .await
            .map_err(|e| format!("Error al verificar pertenencia: {}", e))?;

        if !belongs {
            return Err("Método de pago no encontrado".to_string());
        }

        // Validaciones
        if let Some(digitos) = ultimos_4_digitos {
            if digitos.len() != 4 {
                return Err("Los últimos 4 dígitos deben ser exactamente 4 caracteres".to_string());
            }
        }

        if let Some(expiracion) = fecha_expiracion {
            // Validar formato MM/YY
            if !expiracion.contains('/') || expiracion.len() != 5 {
                return Err("La fecha de expiración debe tener formato MM/YY".to_string());
            }
        }

        MetodoPagoClienteRepository::update(
            pool,
            id_metodo_pago_cliente,
            id_usuario,
            ultimos_4_digitos,
            marca,
            fecha_expiracion,
            nombre_titular,
            es_predeterminado,
        )
        .await
        .map_err(|e| format!("Error al actualizar método de pago: {}", e))
    }

    /// Eliminar un método de pago
    pub async fn delete_payment_method(
        pool: &PgPool,
        id_metodo_pago_cliente: i32,
        id_usuario: i32,
    ) -> Result<(), String> {
        // Verificar que el método de pago pertenece al usuario
        let belongs = MetodoPagoClienteRepository::belongs_to_user(pool, id_metodo_pago_cliente, id_usuario)
            .await
            .map_err(|e| format!("Error al verificar pertenencia: {}", e))?;

        if !belongs {
            return Err("Método de pago no encontrado".to_string());
        }

        MetodoPagoClienteRepository::delete(pool, id_metodo_pago_cliente, id_usuario)
            .await
            .map_err(|e| format!("Error al eliminar método de pago: {}", e))
    }
}
