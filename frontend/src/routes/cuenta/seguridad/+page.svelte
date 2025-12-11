<script lang="ts">
	import { perfilService, type CambiarPasswordRequest } from '$lib/services/perfil';
	import { Lock, CheckCircle, AlertCircle, Eye, EyeOff } from 'lucide-svelte';

	let loading = false;
	let error = '';
	let success = '';

	// Formulario
	let passwordActual = '';
	let passwordNuevo = '';
	let passwordConfirmar = '';

	// Mostrar/ocultar contraseñas
	let showPasswordActual = false;
	let showPasswordNuevo = false;
	let showPasswordConfirmar = false;

	function togglePasswordActual() {
		showPasswordActual = !showPasswordActual;
	}

	function togglePasswordNuevo() {
		showPasswordNuevo = !showPasswordNuevo;
	}

	function togglePasswordConfirmar() {
		showPasswordConfirmar = !showPasswordConfirmar;
	}

	async function handleSubmit() {
		error = '';
		success = '';

		// Validaciones
		if (!passwordActual.trim()) {
			error = 'La contraseña actual es obligatoria';
			return;
		}

		if (!passwordNuevo.trim()) {
			error = 'La nueva contraseña es obligatoria';
			return;
		}

		if (passwordNuevo.length < 6) {
			error = 'La nueva contraseña debe tener al menos 6 caracteres';
			return;
		}

		if (passwordNuevo !== passwordConfirmar) {
			error = 'Las contraseñas nuevas no coinciden';
			return;
		}

		if (passwordActual === passwordNuevo) {
			error = 'La nueva contraseña debe ser diferente a la actual';
			return;
		}

		loading = true;

		try {
			const request: CambiarPasswordRequest = {
				password_actual: passwordActual,
				password_nuevo: passwordNuevo
			};

			await perfilService.cambiarPassword(request);

			success = 'Contraseña actualizada exitosamente';

			// Limpiar formulario
			passwordActual = '';
			passwordNuevo = '';
			passwordConfirmar = '';
			showPasswordActual = false;
			showPasswordNuevo = false;
			showPasswordConfirmar = false;

			// Limpiar mensaje después de 5 segundos
			setTimeout(() => {
				success = '';
			}, 5000);
		} catch (err: any) {
			error = err.message || 'Error al cambiar contraseña';
		} finally {
			loading = false;
		}
	}

	function cancelar() {
		passwordActual = '';
		passwordNuevo = '';
		passwordConfirmar = '';
		error = '';
		success = '';
		showPasswordActual = false;
		showPasswordNuevo = false;
		showPasswordConfirmar = false;
	}
</script>

<svelte:head>
	<title>Seguridad | KronosTech</title>
</svelte:head>

<div class="space-y-6">
	<!-- Header -->
	<div>
		<h1 class="text-3xl font-bold tracking-tight text-text-light dark:text-text-dark">
			Seguridad
		</h1>
		<p class="text-sm text-slate-600 dark:text-slate-400 mt-1">
			Cambia tu contraseña para mantener tu cuenta segura
		</p>
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

	<!-- Contenido -->
	<div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
		<!-- Formulario -->
		<div class="lg:col-span-2">
			<form on:submit|preventDefault={handleSubmit} class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 shadow-sm overflow-hidden">
				<div class="px-6 py-4 border-b border-border-light dark:border-border-dark">
					<h2 class="text-lg font-semibold text-text-light dark:text-text-dark">
						Cambiar Contraseña
					</h2>
				</div>

				<div class="p-6 space-y-5">
					<!-- Contraseña Actual -->
					<div>
						<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
							Contraseña Actual *
						</label>
						<div class="relative">
							<input
								type={showPasswordActual ? 'text' : 'password'}
								bind:value={passwordActual}
								class="form-input w-full pr-10"
								placeholder="Tu contraseña actual"
								required
								disabled={loading}
							/>
							<button
								type="button"
								on:click={togglePasswordActual}
								class="absolute right-3 top-1/2 -translate-y-1/2 text-slate-400 hover:text-slate-600 dark:hover:text-slate-300 transition-colors"
								tabindex="-1"
							>
								{#if showPasswordActual}
									<EyeOff size={20} />
								{:else}
									<Eye size={20} />
								{/if}
							</button>
						</div>
					</div>

					<!-- Nueva Contraseña -->
					<div>
						<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
							Nueva Contraseña *
						</label>
						<div class="relative">
							<input
								type={showPasswordNuevo ? 'text' : 'password'}
								bind:value={passwordNuevo}
								class="form-input w-full pr-10"
								placeholder="Tu nueva contraseña"
								required
								disabled={loading}
							/>
							<button
								type="button"
								on:click={togglePasswordNuevo}
								class="absolute right-3 top-1/2 -translate-y-1/2 text-slate-400 hover:text-slate-600 dark:hover:text-slate-300 transition-colors"
								tabindex="-1"
							>
								{#if showPasswordNuevo}
									<EyeOff size={20} />
								{:else}
									<Eye size={20} />
								{/if}
							</button>
						</div>
						<p class="text-xs text-slate-600 dark:text-slate-400 mt-1">
							Mínimo 6 caracteres
						</p>
					</div>

					<!-- Confirmar Contraseña -->
					<div>
						<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
							Confirmar Nueva Contraseña *
						</label>
						<div class="relative">
							<input
								type={showPasswordConfirmar ? 'text' : 'password'}
								bind:value={passwordConfirmar}
								class="form-input w-full pr-10"
								placeholder="Confirma tu nueva contraseña"
								required
								disabled={loading}
							/>
							<button
								type="button"
								on:click={togglePasswordConfirmar}
								class="absolute right-3 top-1/2 -translate-y-1/2 text-slate-400 hover:text-slate-600 dark:hover:text-slate-300 transition-colors"
								tabindex="-1"
							>
								{#if showPasswordConfirmar}
									<EyeOff size={20} />
								{:else}
									<Eye size={20} />
								{/if}
							</button>
						</div>
					</div>
				</div>

				<!-- Footer -->
				<div class="px-6 py-4 border-t border-border-light dark:border-border-dark flex gap-3 justify-end bg-slate-50 dark:bg-slate-800/30">
					<button
						type="button"
						on:click={cancelar}
						class="px-4 py-2 rounded-lg border border-border-light dark:border-border-dark hover:bg-slate-100 dark:hover:bg-slate-700 transition-colors"
						disabled={loading}
					>
						Cancelar
					</button>
					<button
						type="submit"
						class="px-4 py-2 rounded-lg bg-primary text-white hover:bg-primary/90 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
						disabled={loading}
					>
						{#if loading}
							Cambiando...
						{:else}
							Cambiar Contraseña
						{/if}
					</button>
				</div>
			</form>
		</div>

		<!-- Consejos de Seguridad -->
		<div class="space-y-6">
			<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 shadow-sm p-6">
				<div class="flex items-center gap-3 mb-4">
					<div class="w-10 h-10 rounded-lg bg-primary/10 dark:bg-primary/20 flex items-center justify-center">
						<Lock class="text-primary" size={20} />
					</div>
					<h3 class="text-sm font-semibold text-text-light dark:text-text-dark">
						Consejos de Seguridad
					</h3>
				</div>

				<ul class="space-y-3 text-sm text-slate-600 dark:text-slate-400">
					<li class="flex items-start gap-2">
						<CheckCircle size={16} class="text-green-600 dark:text-green-400 flex-shrink-0 mt-0.5" />
						<span>Usa una contraseña única y compleja</span>
					</li>
					<li class="flex items-start gap-2">
						<CheckCircle size={16} class="text-green-600 dark:text-green-400 flex-shrink-0 mt-0.5" />
						<span>Combina letras mayúsculas, minúsculas, números y símbolos</span>
					</li>
					<li class="flex items-start gap-2">
						<CheckCircle size={16} class="text-green-600 dark:text-green-400 flex-shrink-0 mt-0.5" />
						<span>Evita usar información personal obvia</span>
					</li>
					<li class="flex items-start gap-2">
						<CheckCircle size={16} class="text-green-600 dark:text-green-400 flex-shrink-0 mt-0.5" />
						<span>Cambia tu contraseña periódicamente</span>
					</li>
					<li class="flex items-start gap-2">
						<CheckCircle size={16} class="text-green-600 dark:text-green-400 flex-shrink-0 mt-0.5" />
						<span>No compartas tu contraseña con nadie</span>
					</li>
				</ul>
			</div>
		</div>
	</div>
</div>
