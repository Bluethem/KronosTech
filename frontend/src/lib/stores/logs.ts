import { writable, derived } from 'svelte/store';
import { browser } from '$app/environment';
import * as logsService from '$lib/services/logs';

// ==================== TIPOS ====================

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

// ==================== STORE ====================

function createLogsStore() {
	const { subscribe, set, update } = writable<LogEntry[]>([]);
	const loading = writable<boolean>(false);
	const error = writable<string | null>(null);

	return {
		subscribe,
		loading,
		error,
		
		// Cargar logs desde la API
		load: async (params?: logsService.FiltrarLogsParams) => {
			if (!browser) return;
			
			loading.set(true);
			error.set(null);
			
			try {
				const logs = await logsService.listarLogs(params);
				set(logs);
			} catch (e) {
				error.set('Error al cargar logs');
				console.error('Error loading logs:', e);
			} finally {
				loading.set(false);
			}
		},
		
		// Agregar un nuevo log (envía a la API)
		add: async (entry: Omit<LogEntry, 'id' | 'timestamp' | 'ip'>) => {
			if (!browser) return;
			
			const newLog = await logsService.crearLog({
				nivel: entry.level,
				accion: entry.action,
				detalles: entry.details,
				modulo: entry.module,
				email_usuario: entry.user
			});
			
			if (newLog) {
				update(logs => [newLog, ...logs]);
			}
		},
		
		// Agregar log de info
		info: async (action: string, details: string, module: string, user: string = 'Sistema') => {
			const newLog = await logsService.crearLog({
				nivel: 'info',
				accion: action,
				detalles: details,
				modulo: module,
				email_usuario: user
			});
			
			if (newLog) {
				update(logs => [newLog, ...logs]);
			}
		},
		
		// Agregar log de éxito
		success: async (action: string, details: string, module: string, user: string = 'Sistema') => {
			const newLog = await logsService.crearLog({
				nivel: 'success',
				accion: action,
				detalles: details,
				modulo: module,
				email_usuario: user
			});
			
			if (newLog) {
				update(logs => [newLog, ...logs]);
			}
		},
		
		// Agregar log de advertencia
		warning: async (action: string, details: string, module: string, user: string = 'Sistema') => {
			const newLog = await logsService.crearLog({
				nivel: 'warning',
				accion: action,
				detalles: details,
				modulo: module,
				email_usuario: user
			});
			
			if (newLog) {
				update(logs => [newLog, ...logs]);
			}
		},
		
		// Agregar log de error
		error: async (action: string, details: string, module: string, user: string = 'Sistema') => {
			const newLog = await logsService.crearLog({
				nivel: 'error',
				accion: action,
				detalles: details,
				modulo: module,
				email_usuario: user
			});
			
			if (newLog) {
				update(logs => [newLog, ...logs]);
			}
		},
		
		// Limpiar todos los logs (llama a la API)
		clear: async () => {
			if (!browser) return;
			
			loading.set(true);
			const success = await logsService.limpiarLogs();
			
			if (success) {
				set([]);
			}
			
			loading.set(false);
			return success;
		},
		
		// Refrescar logs desde la API
		refresh: async () => {
			if (!browser) return;
			
			loading.set(true);
			try {
				const logs = await logsService.listarLogs({ limit: 500 });
				set(logs);
			} catch (e) {
				console.error('Error refreshing logs:', e);
			} finally {
				loading.set(false);
			}
		}
	};
}

export const logsStore = createLogsStore();

// ==================== STORES DERIVADOS ====================

// Contadores por nivel
export const logStats = derived(logsStore, ($logs) => ({
	total: $logs.length,
	info: $logs.filter(l => l.level === 'info').length,
	success: $logs.filter(l => l.level === 'success').length,
	warning: $logs.filter(l => l.level === 'warning').length,
	error: $logs.filter(l => l.level === 'error').length,
	security: $logs.filter(l => l.level === 'security').length
}));

// Logs ordenados por fecha (más recientes primero)
export const logsOrdenados = derived(logsStore, ($logs) => 
	[...$logs].sort((a, b) => new Date(b.timestamp).getTime() - new Date(a.timestamp).getTime())
);

// Módulos únicos
export const modulosUnicos = derived(logsStore, ($logs) => 
	Array.from(new Set($logs.map(l => l.module))).sort()
);

// ==================== RE-EXPORTAR FUNCIONES DE SERVICIO ====================

// Para mantener compatibilidad con código existente que usa las funciones del store
export const logAuth = logsService.logAuth;
export const logSecurity = logsService.logSecurity;
export const logUser = logsService.logUser;
export const logConfig = logsService.logConfig;
export const logSystem = logsService.logSystem;
export const logSale = logsService.logSale;
export const logInventory = logsService.logInventory;
export const logProduct = logsService.logProduct;
