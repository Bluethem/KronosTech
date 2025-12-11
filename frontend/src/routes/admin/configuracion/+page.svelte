<script lang="ts">
	import { Settings, Save, RotateCcw, CheckCircle, AlertCircle, Mail, Globe, DollarSign, Truck } from 'lucide-svelte';

	let loading = false;
	let error = '';
	let success = '';

	// Configuración del Sistema
	let config = {
		// General
		siteName: 'KronosTech',
		siteDescription: 'Tu tienda de tecnología de confianza',
		supportEmail: 'soporte@kronostech.com',
		contactPhone: '+51 999 999 999',

		// E-commerce
		currency: 'PEN',
		taxRate: 18, // IGV en Perú
		freeShippingThreshold: 100,
		lowStockThreshold: 10,

		// Email
		emailEnabled: true,
		smtpHost: 'smtp.gmail.com',
		smtpPort: 587,
		smtpUser: 'noreply@kronostech.com',

		// Envío
		defaultShippingCost: 15,
		expressShippingCost: 30,
		estimatedDeliveryDays: 3,

		// Seguridad
		sessionTimeout: 24, // horas
		maxLoginAttempts: 5,
		passwordMinLength: 6,

		// Funcionalidades
		allowGuestCheckout: true,
		requireEmailVerification: false,
		enableReviews: true,
		enableWishlist: true,
		enableCoupons: true
	};

	// Guardar estado inicial para resetear
	const initialConfig = { ...config };

	async function handleSave() {
		error = '';
		success = '';
		loading = true;

		// Validaciones
		if (!config.siteName.trim()) {
			error = 'El nombre del sitio es obligatorio';
			loading = false;
			return;
		}

		if (config.taxRate < 0 || config.taxRate > 100) {
			error = 'La tasa de impuesto debe estar entre 0 y 100';
			loading = false;
			return;
		}

		if (config.sessionTimeout < 1) {
			error = 'El tiempo de sesión debe ser al menos 1 hora';
			loading = false;
			return;
		}

		// Simular guardado (en producción, llamar a un endpoint)
		try {
			await new Promise((resolve) => setTimeout(resolve, 1000));

			// Aquí se haría la llamada al backend
			// await configService.updateConfig(config);

			success = 'Configuración guardada exitosamente';

			setTimeout(() => {
				success = '';
			}, 3000);
		} catch (err: any) {
			error = err.message || 'Error al guardar configuración';
		} finally {
			loading = false;
		}
	}

	function handleReset() {
		config = { ...initialConfig };
		success = 'Configuración restaurada';
		setTimeout(() => {
			success = '';
		}, 2000);
	}
</script>

<svelte:head>
	<title>Configuración del Sistema | KronosTech Admin</title>
</svelte:head>

<div class="space-y-6">
	<!-- Header -->
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-3xl font-bold tracking-tight text-text-light dark:text-text-dark">
				Configuración del Sistema
			</h1>
			<p class="text-sm text-slate-600 dark:text-slate-400 mt-1">
				Gestiona los parámetros globales de la plataforma
			</p>
		</div>
		<div class="flex gap-3">
			<button
				on:click={handleReset}
				class="flex items-center gap-2 px-4 py-2 rounded-lg border border-border-light dark:border-border-dark hover:bg-slate-100 dark:hover:bg-slate-700 transition-colors"
				disabled={loading}
			>
				<RotateCcw size={18} />
				Restaurar
			</button>
			<button
				on:click={handleSave}
				class="flex items-center gap-2 px-4 py-2 rounded-lg bg-primary text-white hover:bg-primary/90 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
				disabled={loading}
			>
				<Save size={18} />
				{loading ? 'Guardando...' : 'Guardar Cambios'}
			</button>
		</div>
	</div>

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
						step="5"
					/>
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
						step="5"
					/>
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
					<Settings class="text-red-600 dark:text-red-400" size={20} />
				</div>
				<h2 class="text-lg font-semibold text-text-light dark:text-text-dark">
					Seguridad
				</h2>
			</div>
			<div class="p-6 grid grid-cols-1 md:grid-cols-3 gap-6">
				<div>
					<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
						Timeout de Sesión (horas)
					</label>
					<input
						type="number"
						bind:value={config.sessionTimeout}
						class="form-input w-full"
						min="1"
						max="168"
					/>
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
				</div>
			</div>
		</div>

		<!-- Funcionalidades -->
		<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 overflow-hidden">
			<div class="px-6 py-4 border-b border-border-light dark:border-border-dark">
				<h2 class="text-lg font-semibold text-text-light dark:text-text-dark">
					Funcionalidades
				</h2>
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
			disabled={loading}
		>
			Restaurar
		</button>
		<button
			on:click={handleSave}
			class="px-6 py-2 rounded-lg bg-primary text-white hover:bg-primary/90 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
			disabled={loading}
		>
			{loading ? 'Guardando...' : 'Guardar Cambios'}
		</button>
	</div>
</div>
