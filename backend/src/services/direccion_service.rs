use sqlx::PgPool;
use crate::models::{DireccionResponse, CrearDireccionRequest, ActualizarDireccionRequest};
use crate::repositories::DireccionRepository;

pub struct DireccionService;

impl DireccionService {
    /// Obtener direcciones del usuario
    pub async fn get_direcciones(
        pool: &PgPool,
        id_usuario: i32,
    ) -> Result<Vec<DireccionResponse>, String> {
        let direcciones = DireccionRepository::get_direcciones_usuario(pool, id_usuario)
            .await
            .map_err(|e| format!("Error al obtener direcciones: {}", e))?;

        Ok(direcciones.into_iter().map(DireccionResponse::from).collect())
    }

    /// Crear nueva dirección
    pub async fn crear_direccion(
        pool: &PgPool,
        id_usuario: i32,
        request: CrearDireccionRequest,
    ) -> Result<DireccionResponse, String> {
        // Validar tipo de dirección
        let tipo_str = request.tipo.as_deref().unwrap_or("envio");
        let tipo_valido = matches!(
            tipo_str.to_lowercase().as_str(),
            "envio" | "facturacion" | "ambos"
        );
        if !tipo_valido {
            return Err("Tipo de dirección inválido. Debe ser: envio, facturacion o ambos".to_string());
        }

        // Validar campos requeridos
        if request.direccion_linea1.trim().is_empty() {
            return Err("La dirección es requerida".to_string());
        }
        if request.ciudad.trim().is_empty() {
            return Err("La ciudad es requerida".to_string());
        }
        if request.departamento.trim().is_empty() {
            return Err("El departamento es requerido".to_string());
        }

        let direccion = DireccionRepository::crear_direccion(pool, id_usuario, &request)
            .await
            .map_err(|e| format!("Error al crear dirección: {}", e))?;

        Ok(DireccionResponse::from(direccion))
    }

    /// Actualizar dirección
    pub async fn actualizar_direccion(
        pool: &PgPool,
        id_usuario: i32,
        id_direccion: i32,
        request: ActualizarDireccionRequest,
    ) -> Result<DireccionResponse, String> {
        // Validar que la dirección pertenezca al usuario
        let direccion_existente = DireccionRepository::get_direccion_by_id(pool, id_direccion, id_usuario)
            .await
            .map_err(|e| format!("Error al buscar dirección: {}", e))?;

        if direccion_existente.is_none() {
            return Err("Dirección no encontrada".to_string());
        }

        // Validar tipo si se proporciona
        if let Some(ref tipo) = request.tipo {
            let tipo_valido = matches!(tipo.to_lowercase().as_str(), "envio" | "facturacion" | "ambos");
            if !tipo_valido {
                return Err("Tipo de dirección inválido".to_string());
            }
        }

        let direccion = DireccionRepository::actualizar_direccion(pool, id_direccion, id_usuario, &request)
            .await
            .map_err(|e| format!("Error al actualizar dirección: {}", e))?;

        Ok(DireccionResponse::from(direccion))
    }

    /// Eliminar dirección
    pub async fn eliminar_direccion(
        pool: &PgPool,
        id_usuario: i32,
        id_direccion: i32,
    ) -> Result<(), String> {
        // Validar que la dirección pertenezca al usuario
        let direccion_existente = DireccionRepository::get_direccion_by_id(pool, id_direccion, id_usuario)
            .await
            .map_err(|e| format!("Error al buscar dirección: {}", e))?;

        if direccion_existente.is_none() {
            return Err("Dirección no encontrada".to_string());
        }

        DireccionRepository::eliminar_direccion(pool, id_direccion, id_usuario)
            .await
            .map_err(|e| format!("Error al eliminar dirección: {}", e))?;

        Ok(())
    }
}
