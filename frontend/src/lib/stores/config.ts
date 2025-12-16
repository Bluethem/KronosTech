import { writable, derived, get } from 'svelte/store';
import { browser } from '$app/environment';
import * as configService from '$lib/services/config';

// ==================== TIPOS ====================

export interface SiteConfig {
	// General
	siteName: string;
	siteDescription: string;
	supportEmail: string;
	contactPhone: string;

	// E-commerce
	currency: string;
	taxRate: number;
	freeShippingThreshold: number;
	lowStockThreshold: number;

	// Envío
	defaultShippingCost: number;
	expressShippingCost: number;
	estimatedDeliveryDays: number;

	// Seguridad
	sessionTimeout: number;
	maxLoginAttempts: number;
	passwordMinLength: number;

	// Funcionalidades
	allowGuestCheckout: boolean;
	requireEmailVerification: boolean;
	enableReviews: boolean;
	enableWishlist: boolean;
	enableCoupons: boolean;

	// Email
	emailEnabled: boolean;
	smtpHost: string;
	smtpPort: number;
	smtpUser: string;
}

// ==================== VALORES POR DEFECTO ====================

const DEFAULT_CONFIG: SiteConfig = {
	siteName: 'KronosTech',
	siteDescription: 'Tu tienda de tecnología de confianza',
	supportEmail: 'soporte@kronostech.com',
	contactPhone: '+51 999 999 999',
	currency: 'PEN',
	taxRate: 18,
	freeShippingThreshold: 100,
	lowStockThreshold: 10,
	defaultShippingCost: 15,
	expressShippingCost: 35,
	estimatedDeliveryDays: 3,
	sessionTimeout: 24,
	maxLoginAttempts: 5,
	passwordMinLength: 6,
	allowGuestCheckout: true,
	requireEmailVerification: false,
	enableReviews: true,
	enableWishlist: true,
	enableCoupons: true,
	emailEnabled: true,
	smtpHost: 'smtp.gmail.com',
	smtpPort: 587,
	smtpUser: 'noreply@kronostech.com'
};

// Mapeo de claves de la BD a propiedades del objeto
const KEY_MAP: Record<string, keyof SiteConfig> = {
	'site_name': 'siteName',
	'site_description': 'siteDescription',
	'support_email': 'supportEmail',
	'contact_phone': 'contactPhone',
	'currency': 'currency',
	'tax_rate': 'taxRate',
	'free_shipping_threshold': 'freeShippingThreshold',
	'low_stock_threshold': 'lowStockThreshold',
	'default_shipping_cost': 'defaultShippingCost',
	'express_shipping_cost': 'expressShippingCost',
	'estimated_delivery_days': 'estimatedDeliveryDays',
	'session_timeout': 'sessionTimeout',
	'max_login_attempts': 'maxLoginAttempts',
	'password_min_length': 'passwordMinLength',
	'allow_guest_checkout': 'allowGuestCheckout',
	'require_email_verification': 'requireEmailVerification',
	'enable_reviews': 'enableReviews',
	'enable_wishlist': 'enableWishlist',
	'enable_coupons': 'enableCoupons',
	'email_enabled': 'emailEnabled',
	'smtp_host': 'smtpHost',
	'smtp_port': 'smtpPort',
	'smtp_user': 'smtpUser'
};

// Mapeo inverso
const REVERSE_KEY_MAP: Record<keyof SiteConfig, string> = Object.fromEntries(
	Object.entries(KEY_MAP).map(([k, v]) => [v, k])
) as Record<keyof SiteConfig, string>;

// ==================== LOCAL STORAGE ====================

const STORAGE_KEY = 'kronostech_config';

function loadFromLocalStorage(): SiteConfig {
	if (!browser) return DEFAULT_CONFIG;
	try {
		const stored = localStorage.getItem(STORAGE_KEY);
		if (stored) {
			return { ...DEFAULT_CONFIG, ...JSON.parse(stored) };
		}
	} catch (e) {
		console.error('Error loading config from localStorage:', e);
	}
	return DEFAULT_CONFIG;
}

function saveToLocalStorage(config: SiteConfig): void {
	if (!browser) return;
	try {
		localStorage.setItem(STORAGE_KEY, JSON.stringify(config));
	} catch (e) {
		console.error('Error saving config to localStorage:', e);
	}
}

// ==================== STORE ====================

function createConfigStore() {
	const { subscribe, set, update } = writable<SiteConfig>(loadFromLocalStorage());
	const loading = writable<boolean>(false);
	const error = writable<string | null>(null);
	const useApi = writable<boolean>(true); // Flag para saber si la API está disponible

	return {
		subscribe,
		loading,
		error,
		useApi,

		// Cargar configuración - intenta API primero, fallback a localStorage
		load: async () => {
			if (!browser) return;

			loading.set(true);
			error.set(null);

			try {
				const configMap = await configService.getAllConfig();
				
				// Si el configMap está vacío, puede ser error de API
				if (Object.keys(configMap).length === 0) {
					throw new Error('No se pudo cargar configuración de la API');
				}
				
				// Convertir el mapa de la BD a nuestro objeto SiteConfig
				const newConfig = { ...DEFAULT_CONFIG };
				
				for (const [dbKey, value] of Object.entries(configMap)) {
					const propKey = KEY_MAP[dbKey];
					if (propKey) {
						const parsed = configService.parseConfigValue(value.valor, value.tipo);
						(newConfig as any)[propKey] = parsed;
					}
				}

				set(newConfig);
				saveToLocalStorage(newConfig); // Guardar copia local
				useApi.set(true);
			} catch (e) {
				console.warn('API de configuración no disponible, usando localStorage:', e);
				// Fallback a localStorage
				const localConfig = loadFromLocalStorage();
				set(localConfig);
				useApi.set(false);
				// No mostrar error al usuario, solo usar localStorage silenciosamente
			} finally {
				loading.set(false);
			}
		},

		// Guardar configuración - intenta API primero, fallback a localStorage
		save: async (config: SiteConfig): Promise<boolean> => {
			if (!browser) return false;

			loading.set(true);
			error.set(null);

			// Siempre guardar en localStorage primero
			saveToLocalStorage(config);
			set(config);

			// Intentar guardar en API
			try {
				const configuraciones: configService.UpdateConfigRequest[] = [];
				
				for (const [propKey, dbKey] of Object.entries(REVERSE_KEY_MAP)) {
					const valor = (config as any)[propKey];
					configuraciones.push({
						clave: dbKey,
						valor: configService.stringifyConfigValue(valor)
					});
				}

				const success = await configService.updateConfigBatch(configuraciones);
				
				if (success) {
					useApi.set(true);
					loading.set(false);
					return true;
				}
			} catch (e) {
				console.warn('No se pudo guardar en API, guardado en localStorage:', e);
				useApi.set(false);
			}

			loading.set(false);
			return true; // Retornar true porque al menos se guardó en localStorage
		},

		// Actualizar localmente
		setLocal: (config: SiteConfig) => {
			set(config);
			saveToLocalStorage(config);
		},

		// Resetear a valores por defecto
		reset: () => {
			set(DEFAULT_CONFIG);
			saveToLocalStorage(DEFAULT_CONFIG);
		},

		// Obtener valor actual
		getConfig: (): SiteConfig => {
			return get({ subscribe });
		}
	};
}

export const siteConfig = createConfigStore();

// ==================== STORES DERIVADOS ====================

export const shippingConfig = derived(siteConfig, ($config) => ({
	defaultCost: $config.defaultShippingCost,
	expressCost: $config.expressShippingCost,
	freeThreshold: $config.freeShippingThreshold,
	estimatedDays: $config.estimatedDeliveryDays
}));

export const ecommerceConfig = derived(siteConfig, ($config) => ({
	currency: $config.currency,
	taxRate: $config.taxRate,
	lowStockThreshold: $config.lowStockThreshold
}));

export const featuresConfig = derived(siteConfig, ($config) => ({
	guestCheckout: $config.allowGuestCheckout,
	emailVerification: $config.requireEmailVerification,
	reviews: $config.enableReviews,
	wishlist: $config.enableWishlist,
	coupons: $config.enableCoupons
}));

export const getCurrency = derived(siteConfig, ($config) => $config.currency);
export const getTaxRate = derived(siteConfig, ($config) => $config.taxRate);
export const getFreeShippingThreshold = derived(siteConfig, ($config) => $config.freeShippingThreshold);
export const getLowStockThreshold = derived(siteConfig, ($config) => $config.lowStockThreshold);
