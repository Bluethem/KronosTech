import { apiAuth } from './auth';
import type { Usuario } from './auth';

// ==================== INTERFACES ====================

export interface ActualizarPerfilRequest {
	nombre?: string;
	apellido?: string;
	telefono?: string;
	dni?: string;
}

export interface CambiarPasswordRequest {
	password_actual: string;
	password_nuevo: string;
}

interface ApiResponse<T> {
	success: boolean;
	data?: T;
	message?: string;
}

// ==================== SERVICIO ====================

class PerfilService {
	/**
	 * Actualizar perfil de usuario
	 */
	async actualizarPerfil(request: ActualizarPerfilRequest): Promise<Usuario> {
		try {
			const { data } = await apiAuth.put<ApiResponse<Usuario>>('/auth/perfil', request);
			if (data.success && data.data) {
				return data.data;
			}
			throw new Error(data.message || 'Error al actualizar perfil');
		} catch (error: any) {
			console.error('Error en actualizarPerfil:', error);
			throw new Error(error.response?.data?.message || 'Error al actualizar perfil');
		}
	}

	/**
	 * Cambiar contraseña
	 */
	async cambiarPassword(request: CambiarPasswordRequest): Promise<void> {
		try {
			const { data } = await apiAuth.put<ApiResponse<null>>('/auth/cambiar-password', request);
			if (!data.success) {
				throw new Error(data.message || 'Error al cambiar contraseña');
			}
		} catch (error: any) {
			console.error('Error en cambiarPassword:', error);
			throw new Error(error.response?.data?.message || 'Error al cambiar contraseña');
		}
	}
}

export const perfilService = new PerfilService();
