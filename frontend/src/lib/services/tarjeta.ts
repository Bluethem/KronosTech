import { apiAuth } from './auth';

// ==================== INTERFACES ====================

export interface MetodoPagoCliente {
	id_metodo_pago_cliente: number;
	id_usuario: number;
	id_metodo_pago: number;
	tipo: string;
	token_pago?: string;
	ultimos_4_digitos?: string;
	marca?: string;
	fecha_expiracion?: string;
	nombre_titular?: string;
	es_predeterminado?: boolean;
	activo?: boolean;
	fecha_creacion?: string;
	fecha_actualizacion?: string;
}

export interface CrearMetodoPagoRequest {
	id_metodo_pago: number;
	tipo: string;
	token_pago?: string;
	ultimos_4_digitos?: string;
	marca?: string;
	fecha_expiracion?: string;
	nombre_titular?: string;
	es_predeterminado?: boolean;
}

export interface ActualizarMetodoPagoRequest {
	ultimos_4_digitos?: string;
	marca?: string;
	fecha_expiracion?: string;
	nombre_titular?: string;
	es_predeterminado?: boolean;
}

interface ApiResponse<T> {
	success: boolean;
	data?: T;
	message?: string;
}

// ==================== SERVICIO ====================

class TarjetaService {
	/**
	 * Obtener todos los métodos de pago del usuario
	 */
	async getMetodosPago(): Promise<MetodoPagoCliente[]> {
		try {
			const { data } = await apiAuth.get<ApiResponse<MetodoPagoCliente[]>>(
				'/metodos-pago-cliente'
			);
			if (data.success && data.data) {
				return data.data;
			}
			throw new Error(data.message || 'Error al obtener métodos de pago');
		} catch (error: any) {
			console.error('Error en getMetodosPago:', error);
			throw new Error(error.response?.data?.message || 'Error al obtener métodos de pago');
		}
	}

	/**
	 * Crear un nuevo método de pago
	 */
	async crearMetodoPago(request: CrearMetodoPagoRequest): Promise<MetodoPagoCliente> {
		try {
			const { data } = await apiAuth.post<ApiResponse<MetodoPagoCliente>>(
				'/metodos-pago-cliente',
				request
			);
			if (data.success && data.data) {
				return data.data;
			}
			throw new Error(data.message || 'Error al crear método de pago');
		} catch (error: any) {
			console.error('Error en crearMetodoPago:', error);
			throw new Error(error.response?.data?.message || 'Error al crear método de pago');
		}
	}

	/**
	 * Actualizar un método de pago existente
	 */
	async actualizarMetodoPago(
		id: number,
		request: ActualizarMetodoPagoRequest
	): Promise<MetodoPagoCliente> {
		try {
			const { data } = await apiAuth.put<ApiResponse<MetodoPagoCliente>>(
				`/metodos-pago-cliente/${id}`,
				request
			);
			if (data.success && data.data) {
				return data.data;
			}
			throw new Error(data.message || 'Error al actualizar método de pago');
		} catch (error: any) {
			console.error('Error en actualizarMetodoPago:', error);
			throw new Error(error.response?.data?.message || 'Error al actualizar método de pago');
		}
	}

	/**
	 * Eliminar un método de pago
	 */
	async eliminarMetodoPago(id: number): Promise<void> {
		try {
			const { data } = await apiAuth.delete<ApiResponse<null>>(
				`/metodos-pago-cliente/${id}`
			);
			if (!data.success) {
				throw new Error(data.message || 'Error al eliminar método de pago');
			}
		} catch (error: any) {
			console.error('Error en eliminarMetodoPago:', error);
			throw new Error(error.response?.data?.message || 'Error al eliminar método de pago');
		}
	}
}

export const tarjetaService = new TarjetaService();
