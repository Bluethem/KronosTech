const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000/api';

export type CartItem = {
  id_carrito_detalle: number;
  id_producto_detalle: number;
  nombre: string;
  sku: string;
  imagen_principal: string | null;
  precio_unitario: number;
  cantidad: number;
  stock_disponible?: number | null;
};

export type CartResponse = {
  id_carrito: number;
  items: CartItem[];
};

async function handleResponse<T>(res: Response): Promise<T> {
  if (!res.ok) {
    const text = await res.text();
    throw new Error(text || `Error HTTP ${res.status}`);
  }
  return res.json();
}

export async function getCurrentCart(): Promise<CartResponse> {
  const res = await fetch(`${API_BASE_URL}/carrito`, {
    credentials: 'include'
  });
  return handleResponse<CartResponse>(res);
}

export async function addItemToCart(id_producto_detalle: number, cantidad: number): Promise<CartResponse> {
  const res = await fetch(`${API_BASE_URL}/carrito/items`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    credentials: 'include',
    body: JSON.stringify({ id_producto_detalle, cantidad })
  });
  return handleResponse<CartResponse>(res);
}

export async function updateItemQuantity(id_carrito_detalle: number, cantidad: number): Promise<CartResponse> {
  const res = await fetch(`${API_BASE_URL}/carrito/items/${id_carrito_detalle}`, {
    method: 'PATCH',
    headers: { 'Content-Type': 'application/json' },
    credentials: 'include',
    body: JSON.stringify({ cantidad })
  });
  return handleResponse<CartResponse>(res);
}

export async function removeItem(id_carrito_detalle: number): Promise<CartResponse> {
  const res = await fetch(`${API_BASE_URL}/carrito/items/${id_carrito_detalle}`, {
    method: 'DELETE',
    credentials: 'include'
  });
  return handleResponse<CartResponse>(res);
}
