import axios, { type AxiosError } from 'axios';
import { getToken } from './auth';

const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000/api';

// Cliente axios para config
const apiConfig = axios.create({
  baseURL: `${API_URL}/config`,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Interceptor para agregar token
apiConfig.interceptors.request.use((config) => {
  const token = getToken();
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

// ==================== INTERFACES ====================

export interface ConfigValue {
  valor: string;
  tipo: string;
  categoria: string;
}

export interface ConfigMap {
  [key: string]: ConfigValue;
}

export interface ConfigResponse {
  success: boolean;
  data: ConfigMap;
  message?: string;
}

export interface UpdateConfigRequest {
  clave: string;
  valor: string;
}

export interface UpdateConfigBatchRequest {
  configuraciones: UpdateConfigRequest[];
}

export interface ErrorResponse {
  success: false;
  message: string;
}

// ==================== HELPERS ====================

/**
 * Convierte el valor según su tipo
 */
export function parseConfigValue(valor: string, tipo: string): string | number | boolean {
  switch (tipo) {
    case 'number':
      return parseFloat(valor) || 0;
    case 'boolean':
      return valor.toLowerCase() === 'true' || valor === '1';
    default:
      return valor;
  }
}

/**
 * Convierte un valor a string para guardar
 */
export function stringifyConfigValue(valor: string | number | boolean): string {
  if (typeof valor === 'boolean') {
    return valor ? 'true' : 'false';
  }
  return String(valor);
}

// ==================== API CALLS ====================

/**
 * Obtener toda la configuración
 */
export async function getAllConfig(): Promise<ConfigMap> {
  try {
    const { data } = await apiConfig.get<ConfigResponse>('');
    return data.data || {};
  } catch (error) {
    const axiosError = error as AxiosError<ErrorResponse>;
    console.error('Error al obtener configuración:', axiosError.response?.data?.message);
    return {};
  }
}

/**
 * Obtener una configuración específica
 */
export async function getConfig(clave: string): Promise<string | null> {
  try {
    const { data } = await apiConfig.get(`/${clave}`);
    return data.data?.valor || null;
  } catch (error) {
    console.error('Error al obtener configuración:', clave);
    return null;
  }
}

/**
 * Actualizar una configuración
 */
export async function updateConfig(clave: string, valor: string): Promise<boolean> {
  try {
    await apiConfig.put(`/${clave}`, { clave, valor });
    return true;
  } catch (error) {
    const axiosError = error as AxiosError<ErrorResponse>;
    console.error('Error al actualizar configuración:', axiosError.response?.data?.message);
    return false;
  }
}

/**
 * Actualizar múltiples configuraciones
 */
export async function updateConfigBatch(configuraciones: UpdateConfigRequest[]): Promise<boolean> {
  try {
    await apiConfig.put('', { configuraciones });
    return true;
  } catch (error) {
    const axiosError = error as AxiosError<ErrorResponse>;
    console.error('Error al actualizar configuraciones:', axiosError.response?.data?.message);
    return false;
  }
}

/**
 * Obtener el timeout de sesión
 */
export async function getSessionTimeout(): Promise<number> {
  try {
    const { data } = await apiConfig.get('/session-timeout');
    return data.data || 24;
  } catch (error) {
    return 24; // Default
  }
}

