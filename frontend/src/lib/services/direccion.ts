import { apiAuth } from './auth';

// ==================== INTERFACES ====================

export interface Direccion {
	id_direccion: number;
	tipo: 'envio' | 'facturacion' | 'ambos';
	nombre_completo?: string;
	direccion_linea1: string;
	direccion_linea2?: string;
	ciudad: string;
	departamento: string;
	codigo_postal?: string;
	pais: string;
	telefono_contacto?: string;
	referencia?: string;
	es_predeterminada: boolean;
	activo: boolean;
	fecha_creacion: string;
}

export interface CrearDireccionRequest {
	tipo: 'envio' | 'facturacion' | 'ambos';
	nombre_completo?: string;
	direccion_linea1: string;
	direccion_linea2?: string;
	ciudad: string;
	departamento: string;
	codigo_postal?: string;
	pais?: string; // Default: "Perú"
	telefono_contacto?: string;
	referencia?: string;
	es_predeterminada?: boolean;
}

export interface ActualizarDireccionRequest {
	tipo?: 'envio' | 'facturacion' | 'ambos';
	nombre_completo?: string;
	direccion_linea1?: string;
	direccion_linea2?: string;
	ciudad?: string;
	departamento?: string;
	codigo_postal?: string;
	pais?: string;
	telefono_contacto?: string;
	referencia?: string;
	es_predeterminada?: boolean;
	activo?: boolean;
}

interface ApiResponse<T> {
	success: boolean;
	data?: T;
	message?: string;
}

// ==================== SERVICIO ====================

class DireccionService {
	/**
	 * Obtener todas las direcciones del usuario
	 */
	async getDirecciones(): Promise<Direccion[]> {
		try {
			const { data } = await apiAuth.get<ApiResponse<Direccion[]>>('/direcciones');
			if (data.success && data.data) {
				return data.data;
			}
			throw new Error(data.message || 'Error al obtener direcciones');
		} catch (error: any) {
			console.error('Error en getDirecciones:', error);
			throw new Error(error.response?.data?.message || 'Error al obtener direcciones');
		}
	}

	/**
	 * Crear una nueva dirección
	 */
	async crearDireccion(request: CrearDireccionRequest): Promise<Direccion> {
		try {
			const { data } = await apiAuth.post<ApiResponse<Direccion>>('/direcciones', request);
			if (data.success && data.data) {
				return data.data;
			}
			throw new Error(data.message || 'Error al crear dirección');
		} catch (error: any) {
			console.error('Error en crearDireccion:', error);
			throw new Error(error.response?.data?.message || 'Error al crear dirección');
		}
	}

	/**
	 * Actualizar una dirección existente
	 */
	async actualizarDireccion(id: number, request: ActualizarDireccionRequest): Promise<Direccion> {
		try {
			const { data } = await apiAuth.put<ApiResponse<Direccion>>(
				`/direcciones/${id}`,
				request
			);
			if (data.success && data.data) {
				return data.data;
			}
			throw new Error(data.message || 'Error al actualizar dirección');
		} catch (error: any) {
			console.error('Error en actualizarDireccion:', error);
			throw new Error(error.response?.data?.message || 'Error al actualizar dirección');
		}
	}

	/**
	 * Eliminar una dirección
	 */
	async eliminarDireccion(id: number): Promise<void> {
		try {
			const { data } = await apiAuth.delete<ApiResponse<null>>(`/direcciones/${id}`);
			if (!data.success) {
				throw new Error(data.message || 'Error al eliminar dirección');
			}
		} catch (error: any) {
			console.error('Error en eliminarDireccion:', error);
			throw new Error(error.response?.data?.message || 'Error al eliminar dirección');
		}
	}
}

export const direccionService = new DireccionService();
