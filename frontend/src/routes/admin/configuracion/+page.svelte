<script lang="ts">
	import { onMount } from 'svelte';
	import { get } from 'svelte/store';
	import { goto } from '$app/navigation';
	import { authUser, isAuthenticated } from '$lib/stores/auth';
	import { siteConfig, type SiteConfig } from '$lib/stores/config';
	import { logConfig, logSystem } from '$lib/services/logs';
	import { Settings, Save, RotateCcw, CheckCircle, AlertCircle, Mail, Globe, DollarSign, Truck, RefreshCw, Shield, Database, AlertTriangle } from 'lucide-svelte';

	let loading = false;
	let saving = false;
	let error = '';
	let success = '';
	let usingLocalStorage = false; // Indica si está usando localStorage como fallback

	// Configuración del Sistema - se sincroniza con el store
	let config: SiteConfig = {
		siteName: 'KronosTech',
		siteDescription: 'Tu tienda de tecnología de confianza',
		supportEmail: 'soporte@kronostech.com',
		contactPhone: '+51 999 999 999',
		currency: 'PEN',
		taxRate: 18,
		freeShippingThreshold: 100,
		lowStockThreshold: 10,
		emailEnabled: true,
		smtpHost: 'smtp.gmail.com',
		smtpPort: 587,
		smtpUser: 'noreply@kronostech.com',
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
		enableCoupons: true
	};

	// Configuración inicial para comparar cambios
	let initialConfig: SiteConfig;

	onMount(async () => {
		if (!$isAuthenticated) {
			goto('/login?redirect=/admin/configuracion');
			return;
		}

		if ($authUser?.rol !== 'super_admin' && $authUser?.rol !== 'administrador') {
			goto('/admin');
			return;
		}

		await cargarConfiguracion();
	});

	async function cargarConfiguracion() {
		loading = true;
		error = '';

		try {
			// Cargar configuración desde la API
			await siteConfig.load();
			config = { ...$siteConfig };
			initialConfig = { ...$siteConfig };
			
			// Verificar si está usando API o localStorage
			usingLocalStorage = !get(siteConfig.useApi);
		} catch (err: any) {
			error = 'Error al cargar la configuración desde el servidor';
			console.error(err);
			usingLocalStorage = true;
		} finally {
			loading = false;
		}
	}

	async function handleSave() {
		error = '';
		success = '';
		saving = true;

		// Validaciones
		if (!config.siteName.trim()) {
			error = 'El nombre del sitio es obligatorio';
			saving = false;
			return;
		}

		if (config.taxRate < 0 || config.taxRate > 100) {
			error = 'La tasa de impuesto debe estar entre 0 y 100';
			saving = false;
			return;
		}

		if (config.sessionTimeout < 1) {
			error = 'El tiempo de sesión debe ser al menos 1 hora';
			saving = false;
			return;
		}

		if (config.defaultShippingCost < 0 || config.expressShippingCost < 0) {
			error = 'Los costos de envío no pueden ser negativos';
			saving = false;
			return;
		}

		if (config.freeShippingThreshold < 0) {
			error = 'El umbral de envío gratis no puede ser negativo';
			saving = false;
			return;
		}

		try {
			// Guardar en la API
			const saved = await siteConfig.save(config);

			if (saved) {
				initialConfig = { ...config };
				
				// Registrar en logs
				await logConfig(
					'Configuración actualizada', 
					`Parámetros del sistema actualizados: Sesión ${config.sessionTimeout}h, Envío S/.${config.defaultShippingCost}`,
					$authUser?.email || 'admin'
				);

				success = 'Configuración guardada exitosamente en la base de datos. Los cambios se aplicarán en todo el sistema.';

				setTimeout(() => {
					success = '';
				}, 5000);
			} else {
				error = 'Error al guardar la configuración';
			}
		} catch (err: any) {
			error = err.message || 'Error al guardar configuración';
		} finally {
			saving = false;
		}
	}

	function handleReset() {
		config = { ...initialConfig };
		success = 'Configuración restaurada a los valores anteriores';
		setTimeout(() => {
			success = '';
		}, 2000);
	}

	async function handleResetToDefaults() {
		if (!confirm('¿Estás seguro de restaurar toda la configuración a valores por defecto?')) {
			return;
		}

		siteConfig.reset();
		config = { ...$siteConfig };
		
		// Guardar los defaults en la BD también
		await siteConfig.save(config);
		initialConfig = { ...config };
		
		await logSystem(
			'Configuración reseteada',
			`Usuario ${$authUser?.email} restauró la configuración a valores por defecto`,
			'warning'
		);

		success = 'Configuración restaurada a valores por defecto';
		setTimeout(() => {
			success = '';
		}, 2000);
	}

	// Verificar si hay cambios sin guardar
	$: hasChanges = JSON.stringify(config) !== JSON.stringify(initialConfig);
</script>

<svelte:head>
	<title>Configuración del Sistema | KronosTech Admin</title>
</svelte:head>

<div class="space-y-6 p-4 md:p-6">
	<!-- Header -->
	<div class="flex items-center justify-between flex-wrap gap-4">
		<div>
			<h1 class="text-3xl font-bold tracking-tight text-text-light dark:text-text-dark">
				Configuración del Sistema
			</h1>
			<p class="text-sm text-slate-600 dark:text-slate-400 mt-1 flex items-center gap-2">
				<Database size={14} />
				Gestiona los parámetros globales. Los cambios se guardan en la base de datos.
			</p>
		</div>
		<div class="flex gap-3">
			<button
				on:click={cargarConfiguracion}
				class="flex items-center gap-2 px-4 py-2 rounded-lg border border-border-light dark:border-border-dark hover:bg-slate-100 dark:hover:bg-slate-700 transition-colors"
				disabled={loading || saving}
				title="Recargar configuración desde la base de datos"
			>
				<RefreshCw size={18} class={loading ? 'animate-spin' : ''} />
				Recargar
			</button>
			<button
				on:click={handleResetToDefaults}
				class="flex items-center gap-2 px-4 py-2 rounded-lg border border-orange-300 dark:border-orange-700 text-orange-600 dark:text-orange-400 hover:bg-orange-50 dark:hover:bg-orange-900/20 transition-colors"
				disabled={loading || saving}
				title="Restaurar a valores de fábrica"
			>
				<RotateCcw size={18} />
				Por defecto
			</button>
			<button
				on:click={handleReset}
				class="flex items-center gap-2 px-4 py-2 rounded-lg border border-border-light dark:border-border-dark hover:bg-slate-100 dark:hover:bg-slate-700 transition-colors"
				disabled={loading || saving || !hasChanges}
			>
				<RotateCcw size={18} />
				Deshacer
			</button>
			<button
				on:click={handleSave}
				class="flex items-center gap-2 px-4 py-2 rounded-lg bg-primary text-white hover:bg-primary/90 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
				disabled={loading || saving || !hasChanges}
			>
				<Save size={18} />
				{saving ? 'Guardando...' : 'Guardar Cambios'}
			</button>
		</div>
	</div>

	<!-- Indicador de fallback a localStorage -->
	{#if usingLocalStorage}
		<div class="rounded-lg bg-orange-50 dark:bg-orange-900/20 border border-orange-200 dark:border-orange-800 p-4 flex items-start gap-3">
			<AlertTriangle class="text-orange-600 dark:text-orange-400 flex-shrink-0 mt-0.5" size={20} />
			<div>
				<p class="text-sm font-medium text-orange-800 dark:text-orange-200">
					Modo local activo
				</p>
				<p class="text-xs text-orange-700 dark:text-orange-300 mt-1">
					La tabla de configuración no existe en la BD. Los cambios se guardan localmente en el navegador.
					Para guardar en la BD, ejecuta los scripts DDL/DML actualizados.
				</p>
			</div>
		</div>
	{/if}

	<!-- Indicador de cambios -->
	{#if hasChanges}
		<div class="rounded-lg bg-amber-50 dark:bg-amber-900/20 border border-amber-200 dark:border-amber-800 p-3 flex items-center gap-2">
			<AlertCircle class="text-amber-600 dark:text-amber-400" size={18} />
			<p class="text-sm text-amber-800 dark:text-amber-200">
				Tienes cambios sin guardar. Haz clic en "Guardar Cambios" para aplicarlos.
			</p>
		</div>
	{/if}

	<!-- Mensajes -->
	{#if error}
		<div class="rounded-lg bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 p-4 flex items-start gap-3">
			<AlertCircle class="text-red-600 dark:text-red-400 flex-shrink-0 mt-0.5" size={20} />
			<p class="text-sm text-red-800 dark:text-red-200">{error}</p>
		</div>
	{/if}

	{#if success}
		<div class="rounded-lg bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 p-4 flex items-start gap-3">
			<CheckCircle class="text-green-600 dark:text-green-400 flex-shrink-0 mt-0.5" size={20} />
			<p class="text-sm text-green-800 dark:text-green-200">{success}</p>
		</div>
	{/if}

	<!-- Loading -->
	{#if loading}
		<div class="flex items-center justify-center py-12">
			<div class="text-center">
				<div class="animate-spin rounded-full h-12 w-12 border-b-2 border-primary mx-auto mb-4"></div>
				<p class="text-slate-600 dark:text-slate-400">Cargando configuración...</p>
			</div>
		</div>
	{:else}
		<!-- Formulario de Configuración -->
		<div class="space-y-6">
			<!-- Configuración General -->
			<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 overflow-hidden">
				<div class="px-6 py-4 border-b border-border-light dark:border-border-dark flex items-center gap-3">
					<div class="w-10 h-10 rounded-lg bg-blue-500/10 dark:bg-blue-500/20 flex items-center justify-center">
						<Globe class="text-blue-600 dark:text-blue-400" size={20} />
					</div>
					<h2 class="text-lg font-semibold text-text-light dark:text-text-dark">
						Configuración General
					</h2>
				</div>
				<div class="p-6 grid grid-cols-1 md:grid-cols-2 gap-6">
					<div>
						<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
							Nombre del Sitio *
						</label>
						<input
							type="text"
							bind:value={config.siteName}
							class="form-input w-full"
							placeholder="KronosTech"
						/>
					</div>

					<div>
						<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
							Email de Soporte *
						</label>
						<input
							type="email"
							bind:value={config.supportEmail}
							class="form-input w-full"
							placeholder="soporte@kronostech.com"
						/>
					</div>

					<div class="md:col-span-2">
						<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
							Descripción del Sitio
						</label>
						<textarea
							bind:value={config.siteDescription}
							class="form-input w-full"
							rows="3"
							placeholder="Descripción..."
						></textarea>
					</div>

					<div>
						<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
							Teléfono de Contacto
						</label>
						<input
							type="tel"
							bind:value={config.contactPhone}
							class="form-input w-full"
							placeholder="+51 999 999 999"
						/>
					</div>
				</div>
			</div>

			<!-- Configuración E-commerce -->
			<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 overflow-hidden">
				<div class="px-6 py-4 border-b border-border-light dark:border-border-dark flex items-center gap-3">
					<div class="w-10 h-10 rounded-lg bg-green-500/10 dark:bg-green-500/20 flex items-center justify-center">
						<DollarSign class="text-green-600 dark:text-green-400" size={20} />
					</div>
					<h2 class="text-lg font-semibold text-text-light dark:text-text-dark">
						E-commerce
					</h2>
				</div>
				<div class="p-6 grid grid-cols-1 md:grid-cols-2 gap-6">
					<div>
						<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
							Moneda
						</label>
						<select bind:value={config.currency} class="form-input w-full">
							<option value="PEN">PEN - Sol Peruano</option>
							<option value="USD">USD - Dólar</option>
							<option value="EUR">EUR - Euro</option>
						</select>
					</div>

					<div>
						<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
							Tasa de Impuesto (%)
						</label>
						<input
							type="number"
							bind:value={config.taxRate}
							class="form-input w-full"
							min="0"
							max="100"
							step="0.01"
						/>
						<p class="text-xs text-slate-600 dark:text-slate-400 mt-1">
							IGV en Perú: 18%
						</p>
					</div>

					<div>
						<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
							Envío Gratis desde (S/.)
						</label>
						<input
							type="number"
							bind:value={config.freeShippingThreshold}
							class="form-input w-full"
							min="0"
							step="10"
						/>
						<p class="text-xs text-slate-600 dark:text-slate-400 mt-1">
							Pedidos mayores a este monto tienen envío gratis
						</p>
					</div>

					<div>
						<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
							Alerta de Stock Bajo
						</label>
						<input
							type="number"
							bind:value={config.lowStockThreshold}
							class="form-input w-full"
							min="1"
						/>
						<p class="text-xs text-slate-600 dark:text-slate-400 mt-1">
							Notificar cuando el stock sea menor a este valor
						</p>
					</div>
				</div>
			</div>

			<!-- Configuración de Envío -->
			<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 overflow-hidden">
				<div class="px-6 py-4 border-b border-border-light dark:border-border-dark flex items-center gap-3">
					<div class="w-10 h-10 rounded-lg bg-purple-500/10 dark:bg-purple-500/20 flex items-center justify-center">
						<Truck class="text-purple-600 dark:text-purple-400" size={20} />
					</div>
					<h2 class="text-lg font-semibold text-text-light dark:text-text-dark">
						Envío y Logística
					</h2>
				</div>
				<div class="p-6 grid grid-cols-1 md:grid-cols-3 gap-6">
					<div>
						<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
							Costo Envío Estándar (S/.)
						</label>
						<input
							type="number"
							bind:value={config.defaultShippingCost}
							class="form-input w-full"
							min="0"
							step="1"
						/>
						<p class="text-xs text-slate-600 dark:text-slate-400 mt-1">
							Se muestra en checkout (3-5 días)
						</p>
					</div>

					<div>
						<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
							Costo Envío Express (S/.)
						</label>
						<input
							type="number"
							bind:value={config.expressShippingCost}
							class="form-input w-full"
							min="0"
							step="1"
						/>
						<p class="text-xs text-slate-600 dark:text-slate-400 mt-1">
							Se muestra en checkout (1-2 días)
						</p>
					</div>

					<div>
						<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
							Días de Entrega Estimados
						</label>
						<input
							type="number"
							bind:value={config.estimatedDeliveryDays}
							class="form-input w-full"
							min="1"
						/>
						<p class="text-xs text-slate-600 dark:text-slate-400 mt-1">
							Días hábiles para envío estándar
						</p>
					</div>
				</div>
			</div>

			<!-- Configuración de Email -->
			<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 overflow-hidden">
				<div class="px-6 py-4 border-b border-border-light dark:border-border-dark flex items-center gap-3">
					<div class="w-10 h-10 rounded-lg bg-orange-500/10 dark:bg-orange-500/20 flex items-center justify-center">
						<Mail class="text-orange-600 dark:text-orange-400" size={20} />
					</div>
					<h2 class="text-lg font-semibold text-text-light dark:text-text-dark">
						Configuración de Email
					</h2>
				</div>
				<div class="p-6 space-y-4">
					<div class="flex items-center gap-2">
						<input
							type="checkbox"
							bind:checked={config.emailEnabled}
							id="emailEnabled"
							class="w-4 h-4 text-primary border-gray-300 rounded focus:ring-primary"
						/>
						<label for="emailEnabled" class="text-sm text-text-light dark:text-text-dark">
							Habilitar envío de emails
						</label>
					</div>

					{#if config.emailEnabled}
						<div class="grid grid-cols-1 md:grid-cols-2 gap-4 pt-4 border-t border-border-light dark:border-border-dark">
							<div>
								<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
									SMTP Host
								</label>
								<input
									type="text"
									bind:value={config.smtpHost}
									class="form-input w-full"
									placeholder="smtp.gmail.com"
								/>
							</div>

							<div>
								<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
									SMTP Port
								</label>
								<input
									type="number"
									bind:value={config.smtpPort}
									class="form-input w-full"
									placeholder="587"
								/>
							</div>

							<div class="md:col-span-2">
								<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
									SMTP User
								</label>
								<input
									type="email"
									bind:value={config.smtpUser}
									class="form-input w-full"
									placeholder="noreply@kronostech.com"
								/>
							</div>
						</div>
					{/if}
				</div>
			</div>

			<!-- Configuración de Seguridad -->
			<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 overflow-hidden">
				<div class="px-6 py-4 border-b border-border-light dark:border-border-dark flex items-center gap-3">
					<div class="w-10 h-10 rounded-lg bg-red-500/10 dark:bg-red-500/20 flex items-center justify-center">
						<Shield class="text-red-600 dark:text-red-400" size={20} />
					</div>
					<div>
						<h2 class="text-lg font-semibold text-text-light dark:text-text-dark">
							Seguridad
						</h2>
						<p class="text-xs text-slate-500 dark:text-slate-400">
							Estos parámetros se aplican al sistema de autenticación
						</p>
					</div>
				</div>
				<div class="p-6 space-y-6">
					<!-- Nota informativa -->
					<div class="rounded-lg bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 p-4">
						<p class="text-sm text-blue-800 dark:text-blue-200">
							<strong>✓ Configuración activa:</strong> La duración de la sesión se aplica al generar nuevos tokens JWT.
							Los usuarios deberán iniciar sesión nuevamente para que el nuevo tiempo de sesión tome efecto.
						</p>
					</div>

					<div class="grid grid-cols-1 md:grid-cols-3 gap-6">
						<div>
							<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
								Duración de Sesión (horas)
							</label>
							<input
								type="number"
								bind:value={config.sessionTimeout}
								class="form-input w-full"
								min="1"
								max="168"
							/>
							<p class="text-xs text-slate-500 dark:text-slate-400 mt-1">
								Tiempo antes de requerir login (1-168h)
							</p>
						</div>

						<div>
							<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
								Máx. Intentos de Login
							</label>
							<input
								type="number"
								bind:value={config.maxLoginAttempts}
								class="form-input w-full"
								min="1"
								max="10"
							/>
							<p class="text-xs text-slate-500 dark:text-slate-400 mt-1">
								Bloquear cuenta tras N intentos fallidos
							</p>
						</div>

						<div>
							<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
								Long. Mín. Contraseña
							</label>
							<input
								type="number"
								bind:value={config.passwordMinLength}
								class="form-input w-full"
								min="6"
								max="20"
							/>
							<p class="text-xs text-slate-500 dark:text-slate-400 mt-1">
								Mínimo de caracteres requeridos
							</p>
						</div>
					</div>
				</div>
			</div>

			<!-- Funcionalidades -->
			<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 overflow-hidden">
				<div class="px-6 py-4 border-b border-border-light dark:border-border-dark">
					<h2 class="text-lg font-semibold text-text-light dark:text-text-dark">
						Funcionalidades
					</h2>
					<p class="text-xs text-slate-600 dark:text-slate-400 mt-1">
						Activa o desactiva funciones del sistema
					</p>
				</div>
				<div class="p-6 grid grid-cols-1 md:grid-cols-2 gap-4">
					<div class="flex items-center gap-2">
						<input
							type="checkbox"
							bind:checked={config.allowGuestCheckout}
							id="allowGuestCheckout"
							class="w-4 h-4 text-primary border-gray-300 rounded focus:ring-primary"
						/>
						<label for="allowGuestCheckout" class="text-sm text-text-light dark:text-text-dark">
							Permitir compra sin registro
						</label>
					</div>

					<div class="flex items-center gap-2">
						<input
							type="checkbox"
							bind:checked={config.requireEmailVerification}
							id="requireEmailVerification"
							class="w-4 h-4 text-primary border-gray-300 rounded focus:ring-primary"
						/>
						<label for="requireEmailVerification" class="text-sm text-text-light dark:text-text-dark">
							Requerir verificación de email
						</label>
					</div>

					<div class="flex items-center gap-2">
						<input
							type="checkbox"
							bind:checked={config.enableReviews}
							id="enableReviews"
							class="w-4 h-4 text-primary border-gray-300 rounded focus:ring-primary"
						/>
						<label for="enableReviews" class="text-sm text-text-light dark:text-text-dark">
							Habilitar valoraciones de productos
						</label>
					</div>

					<div class="flex items-center gap-2">
						<input
							type="checkbox"
							bind:checked={config.enableWishlist}
							id="enableWishlist"
							class="w-4 h-4 text-primary border-gray-300 rounded focus:ring-primary"
						/>
						<label for="enableWishlist" class="text-sm text-text-light dark:text-text-dark">
							Habilitar lista de deseos
						</label>
					</div>

					<div class="flex items-center gap-2">
						<input
							type="checkbox"
							bind:checked={config.enableCoupons}
							id="enableCoupons"
							class="w-4 h-4 text-primary border-gray-300 rounded focus:ring-primary"
						/>
						<label for="enableCoupons" class="text-sm text-text-light dark:text-text-dark">
							Habilitar cupones de descuento
						</label>
					</div>
				</div>
			</div>
		</div>

		<!-- Botones de acción (fijos abajo) -->
		<div class="sticky bottom-0 bg-white dark:bg-slate-900 border-t border-border-light dark:border-border-dark p-4 -mx-6 flex gap-3 justify-end">
			<button
				on:click={handleReset}
				class="px-6 py-2 rounded-lg border border-border-light dark:border-border-dark hover:bg-slate-100 dark:hover:bg-slate-700 transition-colors"
				disabled={saving || !hasChanges}
			>
				Deshacer cambios
			</button>
			<button
				on:click={handleSave}
				class="px-6 py-2 rounded-lg bg-primary text-white hover:bg-primary/90 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
				disabled={saving || !hasChanges}
			>
				{saving ? 'Guardando...' : 'Guardar Cambios'}
			</button>
		</div>
	{/if}
</div>
