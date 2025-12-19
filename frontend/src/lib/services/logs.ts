import axios, { type AxiosError } from 'axios';
import { getToken } from './auth';

const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000/api';

// Cliente axios para logs
const apiLogs = axios.create({
  baseURL: `${API_URL}/logs`,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Interceptor para agregar token
apiLogs.interceptors.request.use((config) => {
  const token = getToken();
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

// ==================== INTERFACES ====================

export type LogLevel = 'info' | 'warning' | 'error' | 'success' | 'security';

export interface LogEntry {
  id: string;
  timestamp: string;
  level: LogLevel;
  action: string;
  user: string;
  ip: string;
  details: string;
  module: string;
}

export interface CrearLogRequest {
  nivel: LogLevel;
  accion: string;
  detalles?: string;
  modulo: string;
  email_usuario?: string;
}

export interface FiltrarLogsParams {
  nivel?: LogLevel;
  modulo?: string;
  fecha_inicio?: string;
  fecha_fin?: string;
  limit?: number;
  offset?: number;
}

export interface LogsResponse {
  success: boolean;
  data: LogEntry[];
  message?: string;
}

export interface LogResponse {
  success: boolean;
  data: LogEntry;
  message?: string;
}

export interface ErrorResponse {
  success: false;
  message: string;
}

// ==================== API CALLS ====================

/**
 * Listar logs con filtros opcionales
 */
export async function listarLogs(params?: FiltrarLogsParams): Promise<LogEntry[]> {
  try {
    const { data } = await apiLogs.get<LogsResponse>('', { params });
    return data.data || [];
  } catch (error) {
    const axiosError = error as AxiosError<ErrorResponse>;
    console.error('Error al listar logs:', axiosError.response?.data?.message);
    return [];
  }
}

/**
 * Crear un nuevo log
 */
export async function crearLog(log: CrearLogRequest): Promise<LogEntry | null> {
  try {
    const { data } = await apiLogs.post<LogResponse>('', log);
    return data.data;
  } catch (error) {
    const axiosError = error as AxiosError<ErrorResponse>;
    console.error('Error al crear log:', axiosError.response?.data?.message);
    return null;
  }
}

/**
 * Limpiar todos los logs (solo super_admin)
 */
export async function limpiarLogs(): Promise<boolean> {
  try {
    await apiLogs.delete('/limpiar');
    return true;
  } catch (error) {
    const axiosError = error as AxiosError<ErrorResponse>;
    console.error('Error al limpiar logs:', axiosError.response?.data?.message);
    return false;
  }
}

// ==================== FUNCIONES HELPER ====================

/**
 * Registrar log de autenticación
 */
export async function logAuth(
  action: string,
  details: string,
  user: string,
  success: boolean = true
): Promise<void> {
  await crearLog({
    nivel: success ? 'success' : 'error',
    accion: action,
    detalles: details,
    modulo: 'Autenticación',
    email_usuario: user
  });
}

/**
 * Registrar log de seguridad
 */
export async function logSecurity(
  action: string,
  details: string,
  user: string = 'Desconocido',
  level: LogLevel = 'warning'
): Promise<void> {
  await crearLog({
    nivel: level,
    accion: action,
    detalles: details,
    modulo: 'Seguridad',
    email_usuario: user
  });
}

/**
 * Registrar log de usuario
 */
export async function logUser(
  action: string,
  details: string,
  user: string,
  level: LogLevel = 'info'
): Promise<void> {
  await crearLog({
    nivel: level,
    accion: action,
    detalles: details,
    modulo: 'Usuarios',
    email_usuario: user
  });
}

/**
 * Registrar log de configuración
 */
export async function logConfig(
  action: string,
  details: string,
  user: string
): Promise<void> {
  await crearLog({
    nivel: 'success',
    accion: action,
    detalles: details,
    modulo: 'Configuración',
    email_usuario: user
  });
}

/**
 * Registrar log de sistema
 */
export async function logSystem(
  action: string,
  details: string,
  level: LogLevel = 'info'
): Promise<void> {
  await crearLog({
    nivel: level,
    accion: action,
    detalles: details,
    modulo: 'Sistema'
  });
}

/**
 * Registrar log de venta
 */
export async function logSale(
  action: string,
  details: string,
  user: string = 'Sistema',
  level: LogLevel = 'success'
): Promise<void> {
  await crearLog({
    nivel: level,
    accion: action,
    detalles: details,
    modulo: 'Ventas',
    email_usuario: user
  });
}

/**
 * Registrar log de inventario
 */
export async function logInventory(
  action: string,
  details: string,
  user: string = 'Sistema',
  level: LogLevel = 'info'
): Promise<void> {
  await crearLog({
    nivel: level,
    accion: action,
    detalles: details,
    modulo: 'Inventario',
    email_usuario: user
  });
}

/**
 * Registrar log de producto
 */
export async function logProduct(
  action: string,
  details: string,
  user: string,
  level: LogLevel = 'info'
): Promise<void> {
  await crearLog({
    nivel: level,
    accion: action,
    detalles: details,
    modulo: 'Productos',
    email_usuario: user
  });
}

