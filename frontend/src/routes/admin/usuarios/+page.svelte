<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { authUser, isAuthenticated } from '$lib/stores/auth';
	import {
		adminService,
		type UsuarioAdmin,
		type ActualizarUsuarioAdminRequest
	} from '$lib/services/admin';
	import { Users, Search, Filter, Edit2, CheckCircle, XCircle, AlertCircle, Mail, Calendar, X } from 'lucide-svelte';

	let usuarios: UsuarioAdmin[] = [];
	let usuariosFiltrados: UsuarioAdmin[] = [];
	let loading = true;
	let error = '';
	let success = '';

	// Filtros
	let filtroRol = '';
	let filtroEstado = '';
	let busqueda = '';

	// Modal de edición
	let showModal = false;
	let usuarioEditando: UsuarioAdmin | null = null;
	let formData = {
		nombre: '',
		apellido: '',
		rol: 'cliente' as 'cliente' | 'administrador' | 'super_admin',
		activo: true,
		email_verificado: false
	};

	onMount(async () => {
		if (!$isAuthenticated) {
			goto('/login?redirect=/admin/usuarios');
			return;
		}

		if ($authUser?.rol !== 'super_admin') {
			goto('/admin');
			return;
		}

		await cargarUsuarios();
	});

	async function cargarUsuarios() {
		loading = true;
		error = '';

		try {
			usuarios = await adminService.listarUsuarios();
		} catch (err: any) {
			console.error('Error al cargar usuarios:', err);
			error = err.message || 'Error al cargar usuarios';
		} finally {
			loading = false;
		}
	}

	// Reactive: filtrar usuarios automáticamente cuando cambien los filtros
	$: usuariosFiltrados = usuarios.filter((usuario) => {
		// Filtro por rol
		if (filtroRol && usuario.rol !== filtroRol) return false;

		// Filtro por estado
		if (filtroEstado === 'activo' && !usuario.activo) return false;
		if (filtroEstado === 'inactivo' && usuario.activo) return false;

		// Búsqueda por nombre, apellido o email
		if (busqueda) {
			const searchLower = busqueda.toLowerCase();
			const nombreCompleto = `${usuario.nombre} ${usuario.apellido}`.toLowerCase();
			const email = usuario.email.toLowerCase();
			if (!nombreCompleto.includes(searchLower) && !email.includes(searchLower)) {
				return false;
			}
		}

		return true;
	});

	function abrirModalEditar(usuario: UsuarioAdmin) {
		usuarioEditando = usuario;
		formData = {
			nombre: usuario.nombre,
			apellido: usuario.apellido,
			rol: usuario.rol,
			activo: usuario.activo,
			email_verificado: usuario.email_verificado
		};
		showModal = true;
		error = '';
		success = '';
	}

	function cerrarModal() {
		showModal = false;
		usuarioEditando = null;
	}

	async function handleSubmit() {
		if (!usuarioEditando) return;

		error = '';
		success = '';

		// Validaciones
		if (!formData.nombre.trim() || !formData.apellido.trim()) {
			error = 'El nombre y apellido son obligatorios';
			return;
		}

		loading = true;

		try {
			const request: ActualizarUsuarioAdminRequest = {
				nombre: formData.nombre.trim(),
				apellido: formData.apellido.trim(),
				rol: formData.rol,
				activo: formData.activo,
				email_verificado: formData.email_verificado
			};

			await adminService.actualizarUsuario(usuarioEditando.id_usuario, request);
			success = 'Usuario actualizado exitosamente';
			await cargarUsuarios();
			cerrarModal();

			// Limpiar mensaje después de 3 segundos
			setTimeout(() => {
				success = '';
			}, 3000);
		} catch (err: any) {
			error = err.message || 'Error al actualizar usuario';
		} finally {
			loading = false;
		}
	}

	function getRolBadgeClass(rol: string): string {
		const classes = {
			cliente: 'bg-blue-100 dark:bg-blue-900/40 text-blue-700 dark:text-blue-300',
			administrador: 'bg-purple-100 dark:bg-purple-900/40 text-purple-700 dark:text-purple-300',
			super_admin: 'bg-red-100 dark:bg-red-900/40 text-red-700 dark:text-red-300'
		};
		return classes[rol as keyof typeof classes] || classes.cliente;
	}

	function getRolLabel(rol: string): string {
		const labels = {
			cliente: 'Cliente',
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
			month: 'short',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function limpiarFiltros() {
		filtroRol = '';
		filtroEstado = '';
		busqueda = '';
	}
</script>

<svelte:head>
	<title>Gestión de Usuarios | KronosTech Admin</title>
</svelte:head>

<div class="space-y-6 p-4 md:p-6">
	<!-- Header -->
	<div>
		<h1 class="text-3xl font-bold tracking-tight text-text-light dark:text-text-dark">
			Gestión de Usuarios
		</h1>
		<p class="text-sm text-slate-600 dark:text-slate-400 mt-1">
			Administra todos los usuarios del sistema
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

	<!-- Filtros y Búsqueda -->
	<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 p-4">
		<div class="grid grid-cols-1 md:grid-cols-4 gap-4">
			<!-- Búsqueda -->
			<div class="md:col-span-2">
				<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
					<Search size={16} class="inline mr-2" />
					Buscar
				</label>
				<input
					type="text"
					bind:value={busqueda}
					class="form-input w-full"
					placeholder="Buscar por nombre o email..."
				/>
			</div>

			<!-- Filtro por Rol -->
			<div>
				<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
					<Filter size={16} class="inline mr-2" />
					Rol
				</label>
				<select bind:value={filtroRol} class="form-input w-full">
					<option value="">Todos</option>
					<option value="cliente">Cliente</option>
					<option value="administrador">Administrador</option>
					<option value="super_admin">Super Admin</option>
				</select>
			</div>

			<!-- Filtro por Estado -->
			<div>
				<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
					Estado
				</label>
				<select bind:value={filtroEstado} class="form-input w-full">
					<option value="">Todos</option>
					<option value="activo">Activos</option>
					<option value="inactivo">Inactivos</option>
				</select>
			</div>
		</div>

		{#if filtroRol || filtroEstado || busqueda}
			<div class="mt-4 flex items-center justify-between">
				<p class="text-sm text-slate-600 dark:text-slate-400">
					Mostrando {usuariosFiltrados.length} de {usuarios.length} usuarios
				</p>
				<button
					on:click={limpiarFiltros}
					class="text-sm text-primary hover:underline"
				>
					Limpiar filtros
				</button>
			</div>
		{/if}
	</div>

	<!-- Loading -->
	{#if loading && !showModal}
		<div class="flex items-center justify-center py-12">
			<div class="animate-spin rounded-full h-12 w-12 border-b-2 border-primary"></div>
		</div>
	{/if}

	<!-- Tabla de Usuarios -->
	{#if !loading || showModal}
		{#if usuariosFiltrados.length === 0}
			<div class="text-center py-12 rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50">
				<Users class="mx-auto text-slate-400 mb-4" size={48} />
				<h3 class="text-lg font-semibold text-text-light dark:text-text-dark mb-2">
					No se encontraron usuarios
				</h3>
				<p class="text-sm text-slate-600 dark:text-slate-400">
					{busqueda || filtroRol || filtroEstado
						? 'Intenta ajustar los filtros de búsqueda'
						: 'No hay usuarios registrados en el sistema'}
				</p>
			</div>
		{:else}
			<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 overflow-hidden">
				<div class="overflow-x-auto">
					<table class="w-full">
						<thead class="bg-slate-50 dark:bg-slate-800/50 border-b border-border-light dark:border-border-dark">
							<tr>
								<th class="px-6 py-3 text-left text-xs font-medium text-slate-600 dark:text-slate-400 uppercase tracking-wider">
									Usuario
								</th>
								<th class="px-6 py-3 text-left text-xs font-medium text-slate-600 dark:text-slate-400 uppercase tracking-wider">
									Email
								</th>
								<th class="px-6 py-3 text-left text-xs font-medium text-slate-600 dark:text-slate-400 uppercase tracking-wider">
									Rol
								</th>
								<th class="px-6 py-3 text-left text-xs font-medium text-slate-600 dark:text-slate-400 uppercase tracking-wider">
									Estado
								</th>
								<th class="px-6 py-3 text-left text-xs font-medium text-slate-600 dark:text-slate-400 uppercase tracking-wider">
									Registro
								</th>
								<th class="px-6 py-3 text-right text-xs font-medium text-slate-600 dark:text-slate-400 uppercase tracking-wider">
									Acciones
								</th>
							</tr>
						</thead>
						<tbody class="divide-y divide-border-light dark:divide-border-dark">
							{#each usuariosFiltrados as usuario (usuario.id_usuario)}
								<tr class="hover:bg-slate-50 dark:hover:bg-slate-800/30 transition-colors">
									<!-- Usuario -->
									<td class="px-6 py-4 whitespace-nowrap">
										<div class="flex items-center gap-3">
											<div class="w-10 h-10 rounded-full bg-primary/10 dark:bg-primary/20 flex items-center justify-center">
												<span class="text-primary font-semibold">
													{usuario.nombre.charAt(0)}{usuario.apellido.charAt(0)}
												</span>
											</div>
											<div>
												<p class="font-medium text-text-light dark:text-text-dark">
													{usuario.nombre} {usuario.apellido}
												</p>
												{#if usuario.telefono}
													<p class="text-xs text-slate-600 dark:text-slate-400">
														Tel: {usuario.telefono}
													</p>
												{/if}
											</div>
										</div>
									</td>

									<!-- Email -->
									<td class="px-6 py-4 whitespace-nowrap">
										<div class="flex items-center gap-2">
											<Mail size={14} class="text-slate-400" />
											<span class="text-sm text-text-light dark:text-text-dark">
												{usuario.email}
											</span>
											{#if usuario.email_verificado}
												<CheckCircle size={14} class="text-green-600 dark:text-green-400" title="Email verificado" />
											{/if}
										</div>
									</td>

									<!-- Rol -->
									<td class="px-6 py-4 whitespace-nowrap">
										<span class="px-2 py-1 rounded text-xs font-medium {getRolBadgeClass(usuario.rol)}">
											{getRolLabel(usuario.rol)}
										</span>
									</td>

									<!-- Estado -->
									<td class="px-6 py-4 whitespace-nowrap">
										{#if usuario.activo}
											<span class="inline-flex items-center gap-1 px-2 py-1 rounded text-xs font-medium bg-green-100 dark:bg-green-900/40 text-green-700 dark:text-green-300">
												<CheckCircle size={12} />
												Activo
											</span>
										{:else}
											<span class="inline-flex items-center gap-1 px-2 py-1 rounded text-xs font-medium bg-red-100 dark:bg-red-900/40 text-red-700 dark:text-red-300">
												<XCircle size={12} />
												Inactivo
											</span>
										{/if}
									</td>

									<!-- Registro -->
									<td class="px-6 py-4 whitespace-nowrap">
										<div class="flex items-center gap-2 text-sm text-slate-600 dark:text-slate-400">
											<Calendar size={14} />
											{formatDate(usuario.fecha_registro)}
										</div>
									</td>

									<!-- Acciones -->
									<td class="px-6 py-4 whitespace-nowrap text-right">
										<button
											on:click={() => abrirModalEditar(usuario)}
											class="inline-flex items-center gap-2 px-3 py-1.5 rounded-lg border border-border-light dark:border-border-dark hover:bg-slate-100 dark:hover:bg-slate-700 transition-colors text-sm"
										>
											<Edit2 size={14} />
											Editar
										</button>
									</td>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>
			</div>
		{/if}
	{/if}
</div>

<!-- Modal Editar Usuario -->
{#if showModal && usuarioEditando}
	<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
		<div class="bg-white dark:bg-slate-800 rounded-xl max-w-md w-full max-h-[90vh] overflow-y-auto">
			<!-- Header -->
			<div class="p-6 border-b border-border-light dark:border-border-dark flex items-center justify-between sticky top-0 bg-white dark:bg-slate-800 z-10">
				<h2 class="text-xl font-semibold text-text-light dark:text-text-dark">
					Editar Usuario
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
				<!-- Email (no editable) -->
				<div>
					<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
						Email
					</label>
					<input
						type="email"
						value={usuarioEditando.email}
						class="form-input w-full bg-slate-100 dark:bg-slate-800 cursor-not-allowed"
						disabled
					/>
					<p class="text-xs text-slate-600 dark:text-slate-400 mt-1">
						El email no puede ser modificado
					</p>
				</div>

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

				<!-- Rol -->
				<div>
					<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
						Rol *
					</label>
					<select bind:value={formData.rol} class="form-input w-full" required disabled={loading}>
						<option value="cliente">Cliente</option>
						<option value="administrador">Administrador</option>
						<option value="super_admin">Super Admin</option>
					</select>
					<p class="text-xs text-slate-600 dark:text-slate-400 mt-1">
						Cambiar el rol afecta los permisos del usuario
					</p>
				</div>

				<!-- Estado Activo -->
				<div class="flex items-center gap-2">
					<input
						type="checkbox"
						bind:checked={formData.activo}
						id="activo"
						class="w-4 h-4 text-primary border-gray-300 rounded focus:ring-primary"
					/>
					<label for="activo" class="text-sm text-text-light dark:text-text-dark">
						Usuario activo (puede iniciar sesión)
					</label>
				</div>

				<!-- Email Verificado -->
				<div class="flex items-center gap-2">
					<input
						type="checkbox"
						bind:checked={formData.email_verificado}
						id="email_verificado"
						class="w-4 h-4 text-primary border-gray-300 rounded focus:ring-primary"
					/>
					<label for="email_verificado" class="text-sm text-text-light dark:text-text-dark">
						Email verificado
					</label>
				</div>

				<!-- Info adicional -->
				<div class="p-3 rounded-lg bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800">
					<p class="text-xs text-blue-800 dark:text-blue-200">
						<strong>Fecha de registro:</strong> {formatDate(usuarioEditando.fecha_registro)}
					</p>
					{#if usuarioEditando.ultima_conexion}
						<p class="text-xs text-blue-800 dark:text-blue-200 mt-1">
							<strong>Última conexión:</strong> {formatDate(usuarioEditando.ultima_conexion)}
						</p>
					{/if}
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
							Guardando...
						{:else}
							Guardar Cambios
						{/if}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}
