import { apiAuth } from './auth';

// ==================== INTERFACES ====================

export interface UsuarioAdmin {
	id_usuario: number;
	nombre: string;
	apellido: string;
	email: string;
	rol: 'cliente' | 'administrador' | 'super_admin';
	activo: boolean;
	email_verificado: boolean;
	telefono?: string;
	dni?: string;
	fecha_registro: string;
	ultima_conexion?: string;
}

export interface ListarUsuariosParams {
	rol?: string;
	activo?: boolean;
	limit?: number;
	offset?: number;
}

export interface ActualizarUsuarioAdminRequest {
	nombre?: string;
	apellido?: string;
	rol?: 'cliente' | 'administrador' | 'super_admin';
	activo?: boolean;
	email_verificado?: boolean;
}

export interface CrearAdministradorRequest {
	nombre: string;
	apellido: string;
	email: string;
	password: string;
	rol: 'administrador' | 'super_admin';
}

interface ApiResponse<T> {
	success: boolean;
	data?: T;
	message?: string;
}

// ==================== SERVICIO ====================

class AdminService {
	/**
	 * Listar todos los usuarios (solo super_admin)
	 */
	async listarUsuarios(params?: ListarUsuariosParams): Promise<UsuarioAdmin[]> {
		try {
			const queryParams = new URLSearchParams();
			if (params?.rol) queryParams.append('rol', params.rol);
			if (params?.activo !== undefined) queryParams.append('activo', params.activo.toString());
			if (params?.limit) queryParams.append('limit', params.limit.toString());
			if (params?.offset) queryParams.append('offset', params.offset.toString());

			const query = queryParams.toString();
			const url = `/admin/usuarios${query ? `?${query}` : ''}`;

			const { data } = await apiAuth.get<ApiResponse<UsuarioAdmin[]>>(url);
			if (data.success && data.data) {
				return data.data;
			}
			throw new Error(data.message || 'Error al listar usuarios');
		} catch (error: any) {
			console.error('Error en listarUsuarios:', error);
			throw new Error(error.response?.data?.message || 'Error al listar usuarios');
		}
	}

	/**
	 * Actualizar usuario (solo super_admin)
	 */
	async actualizarUsuario(
		id: number,
		request: ActualizarUsuarioAdminRequest
	): Promise<UsuarioAdmin> {
		try {
			const { data } = await apiAuth.put<ApiResponse<UsuarioAdmin>>(
				`/admin/usuarios/${id}`,
				request
			);
			if (data.success && data.data) {
				return data.data;
			}
			throw new Error(data.message || 'Error al actualizar usuario');
		} catch (error: any) {
			console.error('Error en actualizarUsuario:', error);
			throw new Error(error.response?.data?.message || 'Error al actualizar usuario');
		}
	}

	/**
	 * Crear nuevo administrador (solo super_admin)
	 */
	async crearAdministrador(request: CrearAdministradorRequest): Promise<UsuarioAdmin> {
		try {
			const { data } = await apiAuth.post<ApiResponse<UsuarioAdmin>>(
				'/admin/administradores',
				request
			);
			if (data.success && data.data) {
				return data.data;
			}
			throw new Error(data.message || 'Error al crear administrador');
		} catch (error: any) {
			console.error('Error en crearAdministrador:', error);
			throw new Error(error.response?.data?.message || 'Error al crear administrador');
		}
	}
}

export const adminService = new AdminService();
