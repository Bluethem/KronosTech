import type { PageLoad } from './$types';

export interface Order {
    id_venta: number;
    numero_pedido: string;
    id_usuario: number;
    nombre_usuario: string | null;
    email_usuario: string | null;
    subtotal: number;
    descuento_total: number | null;
    costo_envio: number | null;
    total: number;
    moneda: string | null;
    estado: string | null;
    estado_pago: string | null;
    fecha_pedido: string | null;
    fecha_entrega_estimada: string | null;
}

export const load: PageLoad = async ({ fetch, url }) => {
    const estado = url.searchParams.get('estado') || '';
    const estado_pago = url.searchParams.get('estado_pago') || '';
    const search = url.searchParams.get('search') || '';

    const params = new URLSearchParams();
    if (estado && estado !== 'Todos') params.append('estado', estado);
    if (estado_pago && estado_pago !== 'Todos') params.append('estado_pago', estado_pago);
    if (search) params.append('search', search);

    try {
        const response = await fetch(`http://localhost:3000/api/ventas?${params.toString()}`);
        if (!response.ok) {
            throw new Error('Failed to fetch orders');
        }
        const orders: Order[] = await response.json();
        return {
            orders
        };
    } catch (error) {
        console.error('Error loading orders:', error);
        return {
            orders: []
        };
    }
};
