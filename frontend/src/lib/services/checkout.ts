import { apiAuth } from './api';

// ==================== INTERFACES ====================

export interface MetodoPago {
	id_metodo_pago: number;
	nombre: string;
	tipo: 'tarjeta_credito' | 'tarjeta_debito' | 'billetera_digital' | 'transferencia' | 'efectivo' | 'contrareembolso';
	proveedor?: string;
	descripcion?: string;
	icono?: string;
	comision_porcentaje: number;
	comision_fija: number;
	tiempo_procesamiento?: string;
	instrucciones?: string;
}

export interface CalcularTotalResponse {
	subtotal: number;
	descuento_total: number;
	descuento_cupon: number;
	costo_envio: number;
	total: number;
	items_count: number;
	cupon_aplicado?: string;
}

export interface ProcesarCheckoutRequest {
	id_direccion: number;
	id_metodo_pago: number;
	notas_cliente?: string;
	codigo_cupon?: string;
}

export interface DetalleVenta {
	id_detalle_venta: number;
	id_producto_detalle: number;
	nombre_producto: string;
	sku: string;
	imagen?: string;
	cantidad: number;
	precio_unitario: number;
	descuento_unitario: number;
	precio_final: number;
	subtotal: number;
}

export interface Venta {
	id_venta: number;
	numero_pedido: string;
	id_usuario: number;
	subtotal: number;
	descuento_total: number;
	costo_envio: number;
	total: number;
	moneda: string;
	estado: 'pendiente' | 'confirmado' | 'procesando' | 'enviado' | 'entregado' | 'cancelado' | 'devuelto';
	estado_pago: 'pendiente' | 'procesando' | 'completado' | 'fallido' | 'rechazado' | 'cancelado' | 'reembolsado' | 'parcialmente_reembolsado';
	// Dirección
	direccion_envio?: string;
	ciudad?: string;
	departamento?: string;
	codigo_postal?: string;
	telefono_contacto?: string;
	metodo_envio?: string;
	numero_tracking?: string;
	// Fechas
	fecha_pedido: string;
	fecha_pago?: string;
	fecha_entrega_estimada?: string;
	notas_cliente?: string;
	// Items
	items: DetalleVenta[];
}

interface ApiResponse<T> {
	success: boolean;
	data?: T;
	message?: string;
}

// ==================== SERVICIO ====================

class CheckoutService {
	/**
	 * Obtener métodos de pago disponibles
	 */
	async getMetodosPago(): Promise<MetodoPago[]> {
		try {
			const { data } = await apiAuth.get<ApiResponse<MetodoPago[]>>('/metodos-pago');
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
	 * Calcular total del checkout
	 * @param idDireccion - ID de dirección para calcular costo de envío (opcional)
	 * @param codigoCupon - Código de cupón para aplicar descuento (opcional)
	 */
	async calcularTotal(idDireccion?: number, codigoCupon?: string): Promise<CalcularTotalResponse> {
		try {
			const params: any = {};
			if (idDireccion) {
				params.id_direccion = idDireccion;
			}
			if (codigoCupon && codigoCupon.trim()) {
				params.codigo_cupon = codigoCupon.trim().toUpperCase();
			}

			const { data } = await apiAuth.get<ApiResponse<CalcularTotalResponse>>(
				'/checkout/calcular-total',
				{ params }
			);

			if (data.success && data.data) {
				return data.data;
			}
			throw new Error(data.message || 'Error al calcular total');
		} catch (error: any) {
			console.error('Error en calcularTotal:', error);
			throw new Error(error.response?.data?.message || 'Error al calcular total');
		}
	}

	/**
	 * Procesar checkout y crear pedido
	 */
	async procesarCheckout(request: ProcesarCheckoutRequest): Promise<Venta> {
		try {
			const { data } = await apiAuth.post<ApiResponse<Venta>>(
				'/checkout/procesar',
				request
			);

			if (data.success && data.data) {
				return data.data;
			}
			throw new Error(data.message || 'Error al procesar checkout');
		} catch (error: any) {
			console.error('Error en procesarCheckout:', error);
			throw new Error(error.response?.data?.message || 'Error al procesar checkout');
		}
	}

	/**
	 * Obtener pedido por ID
	 */
	async getPedido(idVenta: number): Promise<Venta> {
		try {
			const { data } = await apiAuth.get<ApiResponse<Venta>>(`/pedidos/${idVenta}`);

			if (data.success && data.data) {
				return data.data;
			}
			throw new Error(data.message || 'Error al obtener pedido');
		} catch (error: any) {
			console.error('Error en getPedido:', error);
			throw new Error(error.response?.data?.message || 'Error al obtener pedido');
		}
	}

	/**
	 * Listar pedidos del usuario
	 */
	async getPedidos(limit?: number, offset?: number): Promise<Venta[]> {
		try {
			const params: any = {};
			if (limit) params.limit = limit;
			if (offset) params.offset = offset;

			const { data} = await apiAuth.get<ApiResponse<Venta[]>>('/pedidos', { params });

			if (data.success && data.data) {
				return data.data;
			}
			throw new Error(data.message || 'Error al obtener pedidos');
		} catch (error: any) {
			console.error('Error en getPedidos:', error);
			throw new Error(error.response?.data?.message || 'Error al obtener pedidos');
		}
	}

	// ==================== FUTURO: CUPONES ====================
	// TODO: Implementar cuando se habiliten los cupones
	//
	// /**
	//  * Validar código de cupón
	//  */
	// async validarCupon(codigo: string): Promise<CuponInfo> {
	//   try {
	//     const { data } = await apiAuth.post<ApiResponse<CuponInfo>>(
	//       '/cupones/validar',
	//       { codigo }
	//     );
	//
	//     if (data.success && data.data) {
	//       return data.data;
	//     }
	//     throw new Error(data.message || 'Cupón inválido');
	//   } catch (error: any) {
	//     console.error('Error en validarCupon:', error);
	//     throw new Error(error.response?.data?.message || 'Error al validar cupón');
	//   }
	// }
}

export const checkoutService = new CheckoutService();
