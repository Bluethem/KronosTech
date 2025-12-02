import type { PageLoad } from './$types';

export interface OrderDetail {
    id_venta: number;
    numero_pedido: string;
    id_usuario: number;
    nombre_usuario: string | null;
    email_usuario: string | null;
    telefono_usuario: string | null;
    dni_usuario: string | null;
    subtotal: number;
    descuento_total: number | null;
    costo_envio: number | null;
    total: number;
    moneda: string | null;
    estado: string | null;
    estado_pago: string | null;
    direccion_envio: string | null;
    ciudad: string | null;
    departamento: string | null;
    codigo_postal: string | null;
    telefono_contacto: string | null;
    metodo_envio: string | null;
    numero_tracking: string | null;
    fecha_pedido: string | null;
    fecha_entrega_estimada: string | null;
    fecha_actualizacion: string | null;
}

export interface ProductDetail {
    id_detalle_venta: number;
    id_producto: number;
    nombre_producto: string;
    sku: string;
    cantidad: number;
    precio_unitario: number;
    subtotal: number;
    imagen_principal: string | null;
}

export interface OrderComplete {
    id_venta: number;
    numero_pedido: string;
    id_usuario: number;
    nombre_usuario: string | null;
    email_usuario: string | null;
    telefono_usuario: string | null;
    dni_usuario: string | null;
    subtotal: number;
    descuento_total: number | null;
    costo_envio: number | null;
    total: number;
    moneda: string | null;
    estado: string | null;
    estado_pago: string | null;
    direccion_envio: string | null;
    ciudad: string | null;
    departamento: string | null;
    codigo_postal: string | null;
    telefono_contacto: string | null;
    metodo_envio: string | null;
    numero_tracking: string | null;
    fecha_pedido: string | null;
    fecha_entrega_estimada: string | null;
    fecha_actualizacion: string | null;
    productos: ProductDetail[];
}

export const load: PageLoad = async ({ params, fetch }) => {
    try {
        const response = await fetch(`http://localhost:3000/api/ventas/${params.id}`);

        if (!response.ok) {
            throw new Error('Failed to fetch order details');
        }

        const data: OrderComplete = await response.json();
        return {
            order: {
                venta: data,
                productos: data.productos
            }
        };
    } catch (error) {
        console.error('Error loading order details:', error);
        return {
            order: null
        };
    }
};
