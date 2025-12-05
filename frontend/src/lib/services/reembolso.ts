import { apiAuth } from './auth';

// ==================== INTERFACES ====================

export interface Reembolso {
	id_reembolso: number;
	numero_pedido: string;
	id_venta: number;
	nombre_cliente?: string;
	email_cliente?: string;
	tipo_reembolso: string;
	monto_reembolsado: number;
	estado?: string;
	fecha_solicitado?: string;
}

export interface SolicitarReembolsoRequest {
	id_venta: number;
	tipo_reembolso: 'total' | 'parcial';
	monto_reembolsado: number;
	motivo: string;
}

export interface SolicitarReembolsoResponse {
	success: boolean;
	message: string;
	id_reembolso?: number;
}

interface ApiResponse<T> {
	success: boolean;
	data?: T;
	message?: string;
}

// ==================== SERVICIO ====================

class ReembolsoService {
	/**
	 * Obtener mis solicitudes de reembolso
	 */
	async getMisReembolsos(): Promise<Reembolso[]> {
		try {
			const { data } = await apiAuth.get<Reembolso[]>('/mis-reembolsos');
			return data;
		} catch (error: any) {
			console.error('Error en getMisReembolsos:', error);
			throw new Error(error.response?.data?.message || 'Error al obtener reembolsos');
		}
	}

	/**
	 * Solicitar un reembolso
	 */
	async solicitarReembolso(request: SolicitarReembolsoRequest): Promise<SolicitarReembolsoResponse> {
		try {
			const { data } = await apiAuth.post<SolicitarReembolsoResponse>(
				'/mis-reembolsos/solicitar',
				request
			);
			return data;
		} catch (error: any) {
			console.error('Error en solicitarReembolso:', error);
			// Si la respuesta tiene el formato esperado, retornarla
			if (error.response?.data?.success === false) {
				throw error.response.data;
			}
			throw new Error(error.response?.data?.message || 'Error al solicitar reembolso');
		}
	}
}

export const reembolsoService = new ReembolsoService();
