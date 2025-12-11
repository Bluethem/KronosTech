import axios, { type AxiosError } from 'axios';

const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000/api';

// Cliente axios con interceptores para token
export const apiAuth = axios.create({
  baseURL: API_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Interceptor para agregar token a las peticiones
apiAuth.interceptors.request.use((config) => {
  const token = getToken();
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

// ==================== INTERFACES ====================

export interface LoginPayload {
  email: string;
  password: string;
  remember_me?: boolean;
}

export interface RegisterPayload {
  nombre: string;
  apellido: string;
  email: string;
  telefono?: string;
  dni?: string;
  password: string;
}

export interface Usuario {
  id_usuario: number;
  nombre: string;
  apellido: string;
  email: string;
  telefono?: string | null;
  dni?: string | null;
  rol: string;
  email_verificado: boolean;
  activo: boolean;
  fecha_registro: string;
}

export interface LoginResponse {
  success: boolean;
  data: {
    token: string;
    usuario: Usuario;
  };
}

export interface RegisterResponse {
  success: boolean;
  data: {
    message: string;
    usuario: Usuario;
  };
}

export interface UsuarioResponse {
  success: boolean;
  data: Usuario;
}

export interface ErrorResponse {
  success: false;
  message: string;
}

// ==================== GESTIÓN DE TOKEN ====================

export function setToken(token: string): void {
  if (typeof window !== 'undefined') {
    localStorage.setItem('auth_token', token);
  }
}

export function getToken(): string | null {
  if (typeof window !== 'undefined') {
    return localStorage.getItem('auth_token');
  }
  return null;
}

export function removeToken(): void {
  if (typeof window !== 'undefined') {
    localStorage.removeItem('auth_token');
  }
}

export function isAuthenticated(): boolean {
  return getToken() !== null;
}

// ==================== API CALLS ====================

export async function login(payload: LoginPayload): Promise<LoginResponse> {
  try {
    const { data } = await apiAuth.post<LoginResponse>('/auth/login', payload);
    if (data.success && data.data.token) {
      setToken(data.data.token);
    }
    return data;
  } catch (error) {
    const axiosError = error as AxiosError<ErrorResponse>;
    throw new Error(axiosError.response?.data?.message || 'Error al iniciar sesión');
  }
}

export async function register(payload: RegisterPayload): Promise<RegisterResponse> {
  try {
    const { data } = await apiAuth.post<RegisterResponse>('/auth/register', payload);
    return data;
  } catch (error) {
    const axiosError = error as AxiosError<ErrorResponse>;
    throw new Error(axiosError.response?.data?.message || 'Error al registrar usuario');
  }
}

export async function getCurrentUser(): Promise<Usuario> {
  try {
    const { data } = await apiAuth.get<UsuarioResponse>('/auth/me');
    return data.data;
  } catch (error) {
    removeToken();
    const axiosError = error as AxiosError<ErrorResponse>;
    throw new Error(axiosError.response?.data?.message || 'Error al obtener usuario');
  }
}

export async function logout(): Promise<void> {
  try {
    await apiAuth.post('/auth/logout');
  } catch (error) {
    // Ignorar errores de logout
  } finally {
    removeToken();
  }
}
