import { apiAuth } from './auth';
import { setCart, setCartLoading, setCartError, clearCart as clearCartStore } from '$lib/stores/cart';
import type { AxiosError } from 'axios';

// ==================== INTERFACES ====================

export interface CarritoItemResponse {
	id_carrito_detalle: number;
	id_producto_detalle: number;
	nombre: string;
	imagen_principal: string | null;
	precio_unitario: number;
	cantidad: number;
	subtotal: number;
	stock_disponible: number;
}

export interface CarritoResponse {
	id_carrito: number;
	id_usuario: number | null;
	items: CarritoItemResponse[];
	total_items: number;
	subtotal: number;
	fecha_actualizacion: string;
}

export interface ApiResponse<T> {
	success: boolean;
	data?: T;
	message?: string;
}

export interface ErrorResponse {
	success: false;
	message: string;
}

export interface AgregarAlCarritoRequest {
	id_producto_detalle: number;
	cantidad: number;
}

export interface ActualizarCantidadRequest {
	cantidad: number;
}

// ==================== SERVICIOS ====================

export const cartService = {
	// Obtener carrito actual
	async getCart(): Promise<CarritoResponse> {
		try {
			setCartLoading(true);
			setCartError(null);

			const { data } = await apiAuth.get<ApiResponse<CarritoResponse>>('/carrito');

			if (data.success && data.data) {
				setCart(data.data);
				return data.data;
			}

			throw new Error('Error al obtener el carrito');
		} catch (error) {
			const axiosError = error as AxiosError<ErrorResponse>;
			const message = axiosError.response?.data?.message || 'Error al obtener el carrito';
			setCartError(message);
			throw new Error(message);
		} finally {
			setCartLoading(false);
		}
	},

	// Agregar producto al carrito
	async addItem(request: AgregarAlCarritoRequest): Promise<CarritoResponse> {
		try {
			setCartLoading(true);
			setCartError(null);

			const { data } = await apiAuth.post<ApiResponse<CarritoResponse>>('/carrito/items', request);

			if (data.success && data.data) {
				setCart(data.data);
				return data.data;
			}

			throw new Error(data.message || 'Error al agregar producto');
		} catch (error) {
			const axiosError = error as AxiosError<ErrorResponse>;
			const message = axiosError.response?.data?.message || 'Error al agregar producto al carrito';
			setCartError(message);
			throw new Error(message);
		} finally {
			setCartLoading(false);
		}
	},

	// Actualizar cantidad de un item
	async updateQuantity(
		id_carrito_detalle: number,
		request: ActualizarCantidadRequest
	): Promise<CarritoResponse> {
		try {
			setCartLoading(true);
			setCartError(null);

			const { data } = await apiAuth.patch<ApiResponse<CarritoResponse>>(
				`/carrito/items/${id_carrito_detalle}`,
				request
			);

			if (data.success && data.data) {
				setCart(data.data);
				return data.data;
			}

			throw new Error(data.message || 'Error al actualizar cantidad');
		} catch (error) {
			const axiosError = error as AxiosError<ErrorResponse>;
			const message = axiosError.response?.data?.message || 'Error al actualizar la cantidad';
			setCartError(message);
			throw new Error(message);
		} finally {
			setCartLoading(false);
		}
	},

	// Eliminar item del carrito
	async removeItem(id_carrito_detalle: number): Promise<CarritoResponse> {
		try {
			setCartLoading(true);
			setCartError(null);

			const { data } = await apiAuth.delete<ApiResponse<CarritoResponse>>(
				`/carrito/items/${id_carrito_detalle}`
			);

			if (data.success && data.data) {
				setCart(data.data);
				return data.data;
			}

			throw new Error(data.message || 'Error al eliminar item');
		} catch (error) {
			const axiosError = error as AxiosError<ErrorResponse>;
			const message = axiosError.response?.data?.message || 'Error al eliminar el producto';
			setCartError(message);
			throw new Error(message);
		} finally {
			setCartLoading(false);
		}
	},

	// Limpiar carrito
	async clearCart(): Promise<CarritoResponse> {
		try {
			setCartLoading(true);
			setCartError(null);

			const { data } = await apiAuth.delete<ApiResponse<CarritoResponse>>('/carrito');

			if (data.success && data.data) {
				setCart(data.data);
				return data.data;
			}

			throw new Error(data.message || 'Error al limpiar carrito');
		} catch (error) {
			const axiosError = error as AxiosError<ErrorResponse>;
			const message = axiosError.response?.data?.message || 'Error al limpiar el carrito';
			setCartError(message);
			throw new Error(message);
		} finally {
			setCartLoading(false);
		}
	},

	// Inicializar carrito (cargar al iniciar sesión)
	async initCart(): Promise<void> {
		try {
			await this.getCart();
		} catch (error) {
			// Si falla, simplemente limpiar el store
			clearCartStore();
		}
	},

	// Limpiar store (al cerrar sesión)
	clearStore() {
		clearCartStore();
	}
};
