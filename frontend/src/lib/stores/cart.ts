import { writable, derived } from 'svelte/store';
import type { CarritoResponse, CarritoItemResponse } from '$lib/services/cart';

// Store del carrito
export const cart = writable<CarritoResponse | null>(null);
export const cartLoading = writable(false);
export const cartError = writable<string | null>(null);

// Derivados útiles
export const cartItemCount = derived(cart, ($cart) => $cart?.total_items || 0);

export const cartSubtotal = derived(cart, ($cart) => $cart?.subtotal || 0);

export const cartItems = derived(cart, ($cart) => $cart?.items || []);

// Acciones del store
export function setCart(data: CarritoResponse | null) {
	cart.set(data);
}

export function setCartLoading(loading: boolean) {
	cartLoading.set(loading);
}

export function setCartError(error: string | null) {
	cartError.set(error);
}

export function clearCart() {
	cart.set(null);
	cartError.set(null);
}
