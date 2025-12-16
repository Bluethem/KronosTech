<script lang="ts">
	import { onMount } from 'svelte';
	import { authUser } from '$lib/stores/auth';
	import { perfilService, type ActualizarPerfilRequest } from '$lib/services/perfil';
	import { User, Mail, Phone, CreditCard, CheckCircle, AlertCircle, Edit2, X } from 'lucide-svelte';

	$: user = $authUser;

	let editMode = false;
	let loading = false;
	let error = '';
	let success = '';

	// Formulario
	let nombre = '';
	let apellido = '';
	let telefono = '';
	let dni = '';

	onMount(() => {
		if (user) {
			nombre = user.nombre;
			apellido = user.apellido;
			telefono = user.telefono || '';
			dni = user.dni || '';
		}
	});

	function toggleEditMode() {
		if (!editMode && user) {
			// Entrar en modo edición - cargar datos actuales
			nombre = user.nombre;
			apellido = user.apellido;
			telefono = user.telefono || '';
			dni = user.dni || '';
		}
		editMode = !editMode;
		error = '';
		success = '';
	}

	function cancelEdit() {
		editMode = false;
		error = '';
		success = '';
	}

	async function handleSubmit() {
		error = '';
		success = '';

		// Validaciones
		if (!nombre.trim() || !apellido.trim()) {
			error = 'El nombre y apellido son obligatorios';
			return;
		}

		if (telefono && telefono.length < 9) {
			error = 'El teléfono debe tener al menos 9 dígitos';
			return;
		}

		if (dni && dni.length !== 8) {
			error = 'El DNI debe tener 8 dígitos';
			return;
		}

		loading = true;

		try {
			const request: ActualizarPerfilRequest = {
				nombre: nombre.trim(),
				apellido: apellido.trim(),
				telefono: telefono.trim() || undefined,
				dni: dni.trim() || undefined
			};

			const updatedUser = await perfilService.actualizarPerfil(request);

			// Actualizar el store de auth
			authUser.set(updatedUser);

			success = 'Perfil actualizado exitosamente';
			editMode = false;

			// Limpiar mensaje después de 3 segundos
			setTimeout(() => {
				success = '';
			}, 3000);
		} catch (err: any) {
			error = err.message || 'Error al actualizar perfil';
		} finally {
			loading = false;
		}
	}

	function formatDate(dateString?: string): string {
		if (!dateString) return 'N/A';
		const date = new Date(dateString);
		return date.toLocaleDateString('es-ES', {
			year: 'numeric',
			month: 'long',
			day: 'numeric'
		});
	}
</script>

<svelte:head>
	<title>Mi Perfil | KronosTech</title>
</svelte:head>

<div class="space-y-4">
	<!-- Header -->
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-2xl font-bold tracking-tight text-text-light dark:text-text-dark">
				Mi Perfil
			</h1>
			<p class="text-sm text-slate-600 dark:text-slate-400">
				Gestiona tu información personal
			</p>
		</div>
		{#if !editMode}
			<button
				type="button"
				on:click={toggleEditMode}
				class="flex items-center gap-2 px-4 py-2 rounded-lg bg-primary text-white hover:bg-primary/90 transition-colors text-sm"
			>
				<Edit2 size={16} />
				Editar Perfil
			</button>
		{/if}
	</div>

	<!-- Mensajes -->
	{#if error}
		<div class="rounded-lg bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 p-3 flex items-start gap-3">
			<AlertCircle class="text-red-600 dark:text-red-400 flex-shrink-0" size={18} />
			<p class="text-sm text-red-800 dark:text-red-200">{error}</p>
		</div>
	{/if}

	{#if success}
		<div class="rounded-lg bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 p-3 flex items-start gap-3">
			<CheckCircle class="text-green-600 dark:text-green-400 flex-shrink-0" size={18} />
			<p class="text-sm text-green-800 dark:text-green-200">{success}</p>
		</div>
	{/if}

	<!-- Contenido - Layout horizontal -->
	<div class="grid grid-cols-1 lg:grid-cols-[1fr,280px] gap-4">
		<!-- Información del Usuario (Vista) -->
		{#if !editMode}
			<!-- Card Principal - Más compacto -->
			<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 shadow-sm overflow-hidden">
				<div class="px-5 py-3 border-b border-border-light dark:border-border-dark">
					<h2 class="text-base font-semibold text-text-light dark:text-text-dark">
						Información Personal
					</h2>
				</div>

				<div class="p-5 grid grid-cols-1 sm:grid-cols-2 gap-4">
					<!-- Nombre Completo -->
					<div class="flex items-center gap-3">
						<div class="w-9 h-9 rounded-lg bg-primary/10 dark:bg-primary/20 flex items-center justify-center flex-shrink-0">
							<User class="text-primary" size={18} />
						</div>
						<div class="flex-1 min-w-0">
							<p class="text-xs text-slate-600 dark:text-slate-400 uppercase tracking-wide">
								Nombre Completo
							</p>
							<p class="text-sm font-medium text-text-light dark:text-text-dark truncate">
								{user?.nombre} {user?.apellido}
							</p>
						</div>
					</div>

					<!-- Email -->
					<div class="flex items-center gap-3">
						<div class="w-9 h-9 rounded-lg bg-blue-500/10 dark:bg-blue-500/20 flex items-center justify-center flex-shrink-0">
							<Mail class="text-blue-600 dark:text-blue-400" size={18} />
						</div>
						<div class="flex-1 min-w-0">
							<p class="text-xs text-slate-600 dark:text-slate-400 uppercase tracking-wide">
								Correo Electrónico
							</p>
							<div class="flex items-center gap-2">
								<p class="text-sm font-medium text-text-light dark:text-text-dark truncate">
									{user?.email}
								</p>
								{#if user?.email_verificado}
									<span class="inline-flex items-center gap-1 px-1.5 py-0.5 rounded text-xs font-medium bg-green-100 dark:bg-green-900/40 text-green-700 dark:text-green-300">
										<CheckCircle size={10} />
										Verificado
									</span>
								{/if}
							</div>
						</div>
					</div>

					<!-- Teléfono -->
					<div class="flex items-center gap-3">
						<div class="w-9 h-9 rounded-lg bg-emerald-500/10 dark:bg-emerald-500/20 flex items-center justify-center flex-shrink-0">
							<Phone class="text-emerald-600 dark:text-emerald-400" size={18} />
						</div>
						<div class="flex-1 min-w-0">
							<p class="text-xs text-slate-600 dark:text-slate-400 uppercase tracking-wide">
								Teléfono
							</p>
							<p class="text-sm font-medium text-text-light dark:text-text-dark">
								{user?.telefono || 'No especificado'}
							</p>
						</div>
					</div>

					<!-- DNI -->
					<div class="flex items-center gap-3">
						<div class="w-9 h-9 rounded-lg bg-purple-500/10 dark:bg-purple-500/20 flex items-center justify-center flex-shrink-0">
							<CreditCard class="text-purple-600 dark:text-purple-400" size={18} />
						</div>
						<div class="flex-1 min-w-0">
							<p class="text-xs text-slate-600 dark:text-slate-400 uppercase tracking-wide">
								DNI
							</p>
							<p class="text-sm font-medium text-text-light dark:text-text-dark">
								{user?.dni || 'No especificado'}
							</p>
						</div>
					</div>
				</div>
			</div>

			<!-- Info Adicional - Sidebar derecho -->
			<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 shadow-sm p-4 h-fit">
				<h3 class="text-sm font-semibold text-text-light dark:text-text-dark mb-3">
					Estado de Cuenta
				</h3>
				<div class="space-y-2.5">
					<div class="flex items-center justify-between">
						<span class="text-sm text-slate-600 dark:text-slate-400">Rol</span>
						<span class="px-2 py-0.5 rounded text-xs font-medium bg-primary/10 dark:bg-primary/20 text-primary capitalize">
							{user?.rol}
						</span>
					</div>
					<div class="flex items-center justify-between">
						<span class="text-sm text-slate-600 dark:text-slate-400">Estado</span>
						<span class="px-2 py-0.5 rounded text-xs font-medium bg-green-100 dark:bg-green-900/40 text-green-700 dark:text-green-300">
							{user?.activo ? 'Activo' : 'Inactivo'}
						</span>
					</div>
					<div class="flex items-center justify-between">
						<span class="text-sm text-slate-600 dark:text-slate-400">Miembro desde</span>
						<span class="text-xs font-medium text-text-light dark:text-text-dark">
							{formatDate(user?.fecha_registro)}
						</span>
					</div>
				</div>
			</div>
		{:else}
			<!-- Formulario de Edición - Más compacto -->
			<div class="lg:col-span-2">
				<form on:submit|preventDefault={handleSubmit} class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 shadow-sm overflow-hidden">
					<div class="px-5 py-3 border-b border-border-light dark:border-border-dark flex items-center justify-between">
						<h2 class="text-base font-semibold text-text-light dark:text-text-dark">
							Editar Información
						</h2>
						<button
							type="button"
							on:click={cancelEdit}
							class="p-1.5 hover:bg-slate-100 dark:hover:bg-slate-700 rounded-lg transition-colors"
						>
							<X size={18} />
						</button>
					</div>

					<div class="p-5 grid grid-cols-1 sm:grid-cols-2 gap-4">
						<!-- Nombre -->
						<div>
							<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-1.5">
								Nombre *
							</label>
							<input
								type="text"
								bind:value={nombre}
								class="form-input w-full text-sm py-2"
								placeholder="Tu nombre"
								required
							/>
						</div>

						<!-- Apellido -->
						<div>
							<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-1.5">
								Apellido *
							</label>
							<input
								type="text"
								bind:value={apellido}
								class="form-input w-full text-sm py-2"
								placeholder="Tu apellido"
								required
							/>
						</div>

						<!-- Teléfono -->
						<div>
							<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-1.5">
								Teléfono <span class="text-xs text-slate-500">(opcional)</span>
							</label>
							<input
								type="tel"
								bind:value={telefono}
								class="form-input w-full text-sm py-2"
								placeholder="999999999"
								maxlength="15"
							/>
						</div>

						<!-- DNI -->
						<div>
							<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-1.5">
								DNI <span class="text-xs text-slate-500">(opcional)</span>
							</label>
							<input
								type="text"
								bind:value={dni}
								class="form-input w-full text-sm py-2"
								placeholder="12345678"
								maxlength="8"
								pattern="[0-9]*"
							/>
						</div>

						<!-- Email (no editable) - Full width -->
						<div class="sm:col-span-2">
							<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-1.5">
								Correo Electrónico <span class="text-xs text-slate-500">(no editable)</span>
							</label>
							<input
								type="email"
								value={user?.email}
								class="form-input w-full text-sm py-2 bg-slate-100 dark:bg-slate-800 cursor-not-allowed"
								disabled
							/>
						</div>
					</div>

					<!-- Footer -->
					<div class="px-5 py-3 border-t border-border-light dark:border-border-dark flex gap-3 justify-end bg-slate-50 dark:bg-slate-800/30">
						<button
							type="button"
							on:click={cancelEdit}
							class="px-4 py-2 text-sm rounded-lg border border-border-light dark:border-border-dark hover:bg-slate-100 dark:hover:bg-slate-700 transition-colors"
							disabled={loading}
						>
							Cancelar
						</button>
						<button
							type="submit"
							class="px-4 py-2 text-sm rounded-lg bg-primary text-white hover:bg-primary/90 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
							disabled={loading}
						>
							{#if loading}
								Guardando...
							{:else}
								Guardar Cambios
							{/if}
						</button>
					</div>
				</form>
			</div>
		{/if}
	</div>
</div>
