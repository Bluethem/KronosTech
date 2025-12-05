<script lang="ts">
	import { onMount } from 'svelte';
	import {
		adminService,
		type UsuarioAdmin,
		type CrearAdministradorRequest
	} from '$lib/services/admin';
	import { UserPlus, Shield, CheckCircle, AlertCircle, Mail, Calendar, Eye, EyeOff, X } from 'lucide-svelte';

	let administradores: UsuarioAdmin[] = [];
	let loading = true;
	let error = '';
	let success = '';

	// Modal de creación
	let showModal = false;
	let formData = {
		nombre: '',
		apellido: '',
		email: '',
		password: '',
		password_confirmar: '',
		rol: 'administrador' as 'administrador' | 'super_admin'
	};

	// Mostrar/ocultar contraseñas
	let showPassword = false;
	let showPasswordConfirmar = false;

	onMount(async () => {
		await cargarAdministradores();
	});

	async function cargarAdministradores() {
		loading = true;
		error = '';

		try {
			// Obtener solo administradores y super_admins
			const todosUsuarios = await adminService.listarUsuarios();
			administradores = todosUsuarios.filter(
				(u) => u.rol === 'administrador' || u.rol === 'super_admin'
			);
		} catch (err: any) {
			console.error('Error al cargar administradores:', err);
			error = err.message || 'Error al cargar administradores';
		} finally {
			loading = false;
		}
	}

	function abrirModalCrear() {
		resetFormulario();
		showModal = true;
		error = '';
		success = '';
	}

	function cerrarModal() {
		showModal = false;
		resetFormulario();
	}

	function resetFormulario() {
		formData = {
			nombre: '',
			apellido: '',
			email: '',
			password: '',
			password_confirmar: '',
			rol: 'administrador'
		};
		showPassword = false;
		showPasswordConfirmar = false;
	}

	async function handleSubmit() {
		error = '';
		success = '';

		// Validaciones
		if (!formData.nombre.trim() || !formData.apellido.trim()) {
			error = 'El nombre y apellido son obligatorios';
			return;
		}

		if (!formData.email.trim()) {
			error = 'El email es obligatorio';
			return;
		}

		// Validar formato de email
		const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
		if (!emailRegex.test(formData.email)) {
			error = 'El formato del email no es válido';
			return;
		}

		if (formData.password.length < 6) {
			error = 'La contraseña debe tener al menos 6 caracteres';
			return;
		}

		if (formData.password !== formData.password_confirmar) {
			error = 'Las contraseñas no coinciden';
			return;
		}

		loading = true;

		try {
			const request: CrearAdministradorRequest = {
				nombre: formData.nombre.trim(),
				apellido: formData.apellido.trim(),
				email: formData.email.trim().toLowerCase(),
				password: formData.password,
				rol: formData.rol
			};

			await adminService.crearAdministrador(request);
			success = `${formData.rol === 'super_admin' ? 'Super Administrador' : 'Administrador'} creado exitosamente`;
			await cargarAdministradores();
			cerrarModal();

			// Limpiar mensaje después de 3 segundos
			setTimeout(() => {
				success = '';
			}, 3000);
		} catch (err: any) {
			error = err.message || 'Error al crear administrador';
		} finally {
			loading = false;
		}
	}

	function getRolBadgeClass(rol: string): string {
		const classes = {
			administrador: 'bg-purple-100 dark:bg-purple-900/40 text-purple-700 dark:text-purple-300',
			super_admin: 'bg-red-100 dark:bg-red-900/40 text-red-700 dark:text-red-300'
		};
		return classes[rol as keyof typeof classes] || classes.administrador;
	}

	function getRolLabel(rol: string): string {
		const labels = {
			administrador: 'Administrador',
			super_admin: 'Super Admin'
		};
		return labels[rol as keyof typeof labels] || rol;
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

	$: administradoresActivos = administradores.filter((a) => a.activo);
	$: administradoresInactivos = administradores.filter((a) => !a.activo);
	$: superAdmins = administradores.filter((a) => a.rol === 'super_admin');
	$: adminRegulares = administradores.filter((a) => a.rol === 'administrador');
</script>

<svelte:head>
	<title>Administradores | KronosTech Admin</title>
</svelte:head>

<div class="space-y-6">
	<!-- Header -->
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-3xl font-bold tracking-tight text-text-light dark:text-text-dark">
				Administradores
			</h1>
			<p class="text-sm text-slate-600 dark:text-slate-400 mt-1">
				Gestiona los administradores y super administradores del sistema
			</p>
		</div>
		<button
			on:click={abrirModalCrear}
			class="flex items-center gap-2 px-4 py-2 rounded-lg bg-primary text-white hover:bg-primary/90 transition-colors"
		>
			<UserPlus size={18} />
			Nuevo Administrador
		</button>
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

	<!-- Estadísticas -->
	<div class="grid grid-cols-1 md:grid-cols-4 gap-4">
		<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 p-6">
			<div class="flex items-center justify-between">
				<div>
					<p class="text-sm text-slate-600 dark:text-slate-400">Total</p>
					<p class="text-3xl font-bold text-text-light dark:text-text-dark mt-1">
						{administradores.length}
					</p>
				</div>
				<div class="w-12 h-12 rounded-lg bg-blue-500/10 dark:bg-blue-500/20 flex items-center justify-center">
					<Shield class="text-blue-600 dark:text-blue-400" size={24} />
				</div>
			</div>
		</div>

		<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 p-6">
			<div class="flex items-center justify-between">
				<div>
					<p class="text-sm text-slate-600 dark:text-slate-400">Activos</p>
					<p class="text-3xl font-bold text-green-600 dark:text-green-400 mt-1">
						{administradoresActivos.length}
					</p>
				</div>
				<div class="w-12 h-12 rounded-lg bg-green-500/10 dark:bg-green-500/20 flex items-center justify-center">
					<CheckCircle class="text-green-600 dark:text-green-400" size={24} />
				</div>
			</div>
		</div>

		<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 p-6">
			<div class="flex items-center justify-between">
				<div>
					<p class="text-sm text-slate-600 dark:text-slate-400">Super Admins</p>
					<p class="text-3xl font-bold text-red-600 dark:text-red-400 mt-1">
						{superAdmins.length}
					</p>
				</div>
				<div class="w-12 h-12 rounded-lg bg-red-500/10 dark:bg-red-500/20 flex items-center justify-center">
					<Shield class="text-red-600 dark:text-red-400" size={24} />
				</div>
			</div>
		</div>

		<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 p-6">
			<div class="flex items-center justify-between">
				<div>
					<p class="text-sm text-slate-600 dark:text-slate-400">Admins</p>
					<p class="text-3xl font-bold text-purple-600 dark:text-purple-400 mt-1">
						{adminRegulares.length}
					</p>
				</div>
				<div class="w-12 h-12 rounded-lg bg-purple-500/10 dark:bg-purple-500/20 flex items-center justify-center">
					<Shield class="text-purple-600 dark:text-purple-400" size={24} />
				</div>
			</div>
		</div>
	</div>

	<!-- Loading -->
	{#if loading && !showModal}
		<div class="flex items-center justify-center py-12">
			<div class="animate-spin rounded-full h-12 w-12 border-b-2 border-primary"></div>
		</div>
	{/if}

	<!-- Lista de Administradores -->
	{#if !loading || showModal}
		{#if administradores.length === 0}
			<div class="text-center py-12 rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50">
				<Shield class="mx-auto text-slate-400 mb-4" size={48} />
				<h3 class="text-lg font-semibold text-text-light dark:text-text-dark mb-2">
					No hay administradores
				</h3>
				<p class="text-sm text-slate-600 dark:text-slate-400 mb-6">
					Crea el primer administrador del sistema
				</p>
				<button
					on:click={abrirModalCrear}
					class="inline-flex items-center gap-2 px-6 py-3 rounded-lg bg-primary text-white hover:bg-primary/90 transition-colors"
				>
					<UserPlus size={18} />
					Crear Administrador
				</button>
			</div>
		{:else}
			<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
				{#each administradores as admin}
					<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 shadow-sm overflow-hidden hover:shadow-md transition-shadow">
						<!-- Header -->
						<div class="p-4 border-b border-border-light dark:border-border-dark bg-slate-50 dark:bg-slate-800/30">
							<div class="flex items-start justify-between gap-3">
								<div class="flex items-start gap-3 flex-1">
									<div class="w-12 h-12 rounded-full bg-primary/10 dark:bg-primary/20 flex items-center justify-center">
										<span class="text-primary font-bold text-lg">
											{admin.nombre.charAt(0)}{admin.apellido.charAt(0)}
										</span>
									</div>
									<div class="flex-1">
										<p class="font-semibold text-text-light dark:text-text-dark">
											{admin.nombre} {admin.apellido}
										</p>
										<div class="flex items-center gap-2 mt-1">
											<Mail size={14} class="text-slate-400" />
											<p class="text-sm text-slate-600 dark:text-slate-400">
												{admin.email}
											</p>
										</div>
									</div>
								</div>

								<span class="px-2 py-1 rounded text-xs font-medium {getRolBadgeClass(admin.rol)}">
									{getRolLabel(admin.rol)}
								</span>
							</div>
						</div>

						<!-- Contenido -->
						<div class="p-4 space-y-3">
							<!-- Estado -->
							<div class="flex items-center justify-between">
								<span class="text-sm text-slate-600 dark:text-slate-400">Estado</span>
								{#if admin.activo}
									<span class="inline-flex items-center gap-1 px-2 py-1 rounded text-xs font-medium bg-green-100 dark:bg-green-900/40 text-green-700 dark:text-green-300">
										<CheckCircle size={12} />
										Activo
									</span>
								{:else}
									<span class="inline-flex items-center gap-1 px-2 py-1 rounded text-xs font-medium bg-red-100 dark:bg-red-900/40 text-red-700 dark:text-red-300">
										<AlertCircle size={12} />
										Inactivo
									</span>
								{/if}
							</div>

							<!-- Email Verificado -->
							<div class="flex items-center justify-between">
								<span class="text-sm text-slate-600 dark:text-slate-400">Email</span>
								{#if admin.email_verificado}
									<span class="inline-flex items-center gap-1 px-2 py-1 rounded text-xs font-medium bg-green-100 dark:bg-green-900/40 text-green-700 dark:text-green-300">
										<CheckCircle size={12} />
										Verificado
									</span>
								{:else}
									<span class="text-xs text-slate-500">No verificado</span>
								{/if}
							</div>

							<!-- Fecha de Registro -->
							<div class="flex items-center justify-between">
								<span class="text-sm text-slate-600 dark:text-slate-400">Registro</span>
								<div class="flex items-center gap-2 text-sm text-text-light dark:text-text-dark">
									<Calendar size={14} />
									{formatDate(admin.fecha_registro)}
								</div>
							</div>

							<!-- Última Conexión -->
							{#if admin.ultima_conexion}
								<div class="flex items-center justify-between">
									<span class="text-sm text-slate-600 dark:text-slate-400">Última conexión</span>
									<span class="text-sm text-text-light dark:text-text-dark">
										{formatDate(admin.ultima_conexion)}
									</span>
								</div>
							{/if}
						</div>
					</div>
				{/each}
			</div>
		{/if}
	{/if}
</div>

<!-- Modal Crear Administrador -->
{#if showModal}
	<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
		<div class="bg-white dark:bg-slate-800 rounded-xl max-w-md w-full max-h-[90vh] overflow-y-auto">
			<!-- Header -->
			<div class="p-6 border-b border-border-light dark:border-border-dark flex items-center justify-between sticky top-0 bg-white dark:bg-slate-800 z-10">
				<h2 class="text-xl font-semibold text-text-light dark:text-text-dark">
					Nuevo Administrador
				</h2>
				<button
					on:click={cerrarModal}
					class="p-2 hover:bg-slate-100 dark:hover:bg-slate-700 rounded-lg transition-colors"
				>
					<X size={20} />
				</button>
			</div>

			<!-- Form -->
			<form on:submit|preventDefault={handleSubmit} class="p-6 space-y-4">
				<!-- Nombre -->
				<div>
					<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
						Nombre *
					</label>
					<input
						type="text"
						bind:value={formData.nombre}
						class="form-input w-full"
						placeholder="Nombre"
						required
						disabled={loading}
					/>
				</div>

				<!-- Apellido -->
				<div>
					<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
						Apellido *
					</label>
					<input
						type="text"
						bind:value={formData.apellido}
						class="form-input w-full"
						placeholder="Apellido"
						required
						disabled={loading}
					/>
				</div>

				<!-- Email -->
				<div>
					<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
						Email *
					</label>
					<input
						type="email"
						bind:value={formData.email}
						class="form-input w-full"
						placeholder="email@ejemplo.com"
						required
						disabled={loading}
					/>
				</div>

				<!-- Rol -->
				<div>
					<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
						Tipo de Administrador *
					</label>
					<select bind:value={formData.rol} class="form-input w-full" required disabled={loading}>
						<option value="administrador">Administrador</option>
						<option value="super_admin">Super Administrador</option>
					</select>
					<p class="text-xs text-slate-600 dark:text-slate-400 mt-1">
						{#if formData.rol === 'super_admin'}
							Super Admin: Gestión completa del sistema y usuarios
						{:else}
							Administrador: Gestión operativa (pedidos, inventario, etc.)
						{/if}
					</p>
				</div>

				<!-- Contraseña -->
				<div>
					<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
						Contraseña *
					</label>
					<div class="relative">
						<input
							type={showPassword ? 'text' : 'password'}
							bind:value={formData.password}
							class="form-input w-full pr-10"
							placeholder="Mínimo 6 caracteres"
							required
							disabled={loading}
						/>
						<button
							type="button"
							on:click={() => (showPassword = !showPassword)}
							class="absolute right-3 top-1/2 -translate-y-1/2 text-slate-400 hover:text-slate-600 dark:hover:text-slate-300 transition-colors"
							tabindex="-1"
						>
							{#if showPassword}
								<EyeOff size={20} />
							{:else}
								<Eye size={20} />
							{/if}
						</button>
					</div>
				</div>

				<!-- Confirmar Contraseña -->
				<div>
					<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
						Confirmar Contraseña *
					</label>
					<div class="relative">
						<input
							type={showPasswordConfirmar ? 'text' : 'password'}
							bind:value={formData.password_confirmar}
							class="form-input w-full pr-10"
							placeholder="Repite la contraseña"
							required
							disabled={loading}
						/>
						<button
							type="button"
							on:click={() => (showPasswordConfirmar = !showPasswordConfirmar)}
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

				<!-- Nota de Seguridad -->
				<div class="p-3 rounded-lg bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800">
					<p class="text-xs text-yellow-800 dark:text-yellow-200">
						⚠️ El administrador recibirá un email con sus credenciales. Asegúrate de usar un email válido.
					</p>
				</div>

				<!-- Botones -->
				<div class="flex gap-3 justify-end pt-4 border-t border-border-light dark:border-border-dark">
					<button
						type="button"
						on:click={cerrarModal}
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
							Creando...
						{:else}
							Crear Administrador
						{/if}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}
