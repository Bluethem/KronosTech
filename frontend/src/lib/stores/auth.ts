import { writable } from 'svelte/store';
import type { AuthUser } from '$lib/services/auth';

type AuthState = {
  user: AuthUser | null;
  token: string | null;
};

type PersistStrategy = 'local' | 'session';

const STORAGE_KEY = 'kronostech_auth_state';

const isBrowser = typeof window !== 'undefined';

function persistState(state: AuthState | null, strategy: PersistStrategy = 'local') {
  if (!isBrowser) return;

  if (!state || !state.token) {
    window.localStorage?.removeItem(STORAGE_KEY);
    window.sessionStorage?.removeItem(STORAGE_KEY);
    return;
  }

  const serialized = JSON.stringify(state);

  if (strategy === 'session') {
    window.sessionStorage?.setItem(STORAGE_KEY, serialized);
    window.localStorage?.removeItem(STORAGE_KEY);
  } else {
    window.localStorage?.setItem(STORAGE_KEY, serialized);
    window.sessionStorage?.removeItem(STORAGE_KEY);
  }
}

function loadPersistedState(): AuthState | null {
  if (!isBrowser) return null;

  const sessionValue = window.sessionStorage?.getItem(STORAGE_KEY);
  const localValue = window.localStorage?.getItem(STORAGE_KEY);
  const raw = sessionValue || localValue;

  if (!raw) return null;

  try {
    return JSON.parse(raw) as AuthState;
  } catch (error) {
    console.error('Error al leer el estado de autenticación almacenado', error);
    return null;
  }
}

function createAuthStore() {
  const initialState: AuthState = {
    user: null,
    token: null
  };

  const { subscribe, set } = writable<AuthState>(initialState);

  return {
    subscribe,
    setAuth(token: string, user: AuthUser, options?: { remember?: boolean }) {
      const state: AuthState = { token, user };
      set(state);
      persistState(state, options?.remember === false ? 'session' : 'local');
    },
    logout() {
      set({ user: null, token: null });
      persistState(null);
    },
    hydrate() {
      const saved = loadPersistedState();
      if (saved?.token && saved?.user) {
        set(saved);
      }
    }
  };
}

const internalAuthStore = createAuthStore();

export const authStore = {
  subscribe: internalAuthStore.subscribe
};

export const setAuth = (token: string, user: AuthUser, options?: { remember?: boolean }) =>
  internalAuthStore.setAuth(token, user, options);

export const logout = () => internalAuthStore.logout();
export const hydrateAuth = () => internalAuthStore.hydrate();

if (isBrowser) {
  hydrateAuth();
}
