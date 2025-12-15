import { apiAuth } from './auth';

interface ApiResponse<T> {
	success: boolean;
	data?: T;
	message?: string;
}

export interface CuponUsuario {
	id_cupon: number;
	codigo: string;
	nombre: string;
	descripcion?: string;
	tipo_cupon: 'porcentaje' | 'monto_fijo' | 'envio_gratis' | string;
	valor: number;
	fecha_inicio: string;
	fecha_fin: string;
	usos_maximos_por_usuario?: number;
	usos_maximos_totales?: number;
	usos_actuales?: number;
	usos_usuario: number;
	usado: boolean;
	fecha_asignacion: string;
}

class CuponService {
	async getMisCupones(): Promise<CuponUsuario[]> {
		try {
			const { data } = await apiAuth.get<ApiResponse<CuponUsuario[]>>('/cupones/mis');
			if (data.success && data.data) {
				return data.data;
			}
			throw new Error(data.message || 'Error al obtener tus cupones');
		} catch (error: any) {
			console.error('Error en getMisCupones:', error);
			throw new Error(error.response?.data?.message || 'Error al obtener tus cupones');
		}
	}
}

export const cuponService = new CuponService();
