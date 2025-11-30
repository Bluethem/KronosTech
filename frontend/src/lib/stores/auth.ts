import { writable, derived } from 'svelte/store';
import { browser } from '$app/environment';
import type { Usuario } from '$lib/services/auth';
import { getCurrentUser, logout as logoutApi, isAuthenticated as checkAuth } from '$lib/services/auth';
import { cartService } from '$lib/services/cart';

// ==================== STORES ====================

export const authUser = writable<Usuario | null>(null);
export const authLoading = writable<boolean>(false);
export const authError = writable<string | null>(null);

// Store derivado para verificar si está autenticado
export const isAuthenticated = derived(
  authUser,
  ($authUser) => $authUser !== null
);

// ==================== FUNCIONES ====================

export function setUser(user: Usuario | null) {
  authUser.set(user);
  authError.set(null);
}

export function setAuthLoading(value: boolean) {
  authLoading.set(value);
}

export function setAuthError(error: string | null) {
  authError.set(error);
}

// Inicializar usuario desde localStorage
export async function initAuth(): Promise<void> {
  if (!browser) return;

  if (checkAuth()) {
    setAuthLoading(true);
    try {
      const usuario = await getCurrentUser();
      setUser(usuario);
      // Inicializar carrito cuando el usuario está autenticado
      await cartService.initCart();
    } catch (error) {
      setUser(null);
      setAuthError(error instanceof Error ? error.message : 'Error al cargar usuario');
    } finally {
      setAuthLoading(false);
    }
  }
}

// Cerrar sesión
export async function logout(): Promise<void> {
  setAuthLoading(true);
  try {
    await logoutApi();
    setUser(null);
    // Limpiar carrito al cerrar sesión
    cartService.clearStore();
    if (browser) {
      window.location.href = '/';
    }
  } catch (error) {
    setAuthError(error instanceof Error ? error.message : 'Error al cerrar sesión');
  } finally {
    setAuthLoading(false);
  }
}
