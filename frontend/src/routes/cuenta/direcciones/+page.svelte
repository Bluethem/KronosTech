<script lang="ts">
	import { onMount } from 'svelte';
	import {
		direccionService,
		type Direccion,
		type CrearDireccionRequest,
		type ActualizarDireccionRequest
	} from '$lib/services/direccion';
	import { MapPin, Plus, Edit2, Trash2, CheckCircle, AlertCircle, Star, X } from 'lucide-svelte';

	let direcciones: Direccion[] = [];
	let loading = true;
	let error = '';
	let success = '';

	// Modal state
	let showModal = false;
	let modalMode: 'crear' | 'editar' = 'crear';
	let direccionEditando: Direccion | null = null;

	// Confirmación de eliminación
	let showDeleteConfirm = false;
	let direccionEliminar: Direccion | null = null;

	// Formulario
	let formData = {
		tipo: 'envio' as 'envio' | 'facturacion' | 'ambos',
		nombre_completo: '',
		direccion_linea1: '',
		direccion_linea2: '',
		ciudad: '',
		departamento: '',
		codigo_postal: '',
		pais: 'Perú',
		telefono_contacto: '',
		referencia: '',
		es_predeterminada: false
	};

	onMount(async () => {
		await cargarDirecciones();
	});

	async function cargarDirecciones() {
		loading = true;
		error = '';

		try {
			direcciones = await direccionService.getDirecciones();
		} catch (err: any) {
			console.error('Error al cargar direcciones:', err);
			error = err.message || 'Error al cargar direcciones';
		} finally {
			loading = false;
		}
	}

	function abrirModalCrear() {
		modalMode = 'crear';
		direccionEditando = null;
		resetFormulario();
		showModal = true;
		error = '';
		success = '';
	}

	function abrirModalEditar(direccion: Direccion) {
		modalMode = 'editar';
		direccionEditando = direccion;
		formData = {
			tipo: direccion.tipo,
			nombre_completo: direccion.nombre_completo || '',
			direccion_linea1: direccion.direccion_linea1,
			direccion_linea2: direccion.direccion_linea2 || '',
			ciudad: direccion.ciudad,
			departamento: direccion.departamento,
			codigo_postal: direccion.codigo_postal || '',
			pais: direccion.pais,
			telefono_contacto: direccion.telefono_contacto || '',
			referencia: direccion.referencia || '',
			es_predeterminada: direccion.es_predeterminada
		};
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
			tipo: 'envio',
			nombre_completo: '',
			direccion_linea1: '',
			direccion_linea2: '',
			ciudad: '',
			departamento: '',
			codigo_postal: '',
			pais: 'Perú',
			telefono_contacto: '',
			referencia: '',
			es_predeterminada: false
		};
	}

	async function handleSubmit() {
		error = '';
		success = '';

		// Validaciones
		if (!formData.direccion_linea1.trim()) {
			error = 'La dirección es obligatoria';
			return;
		}

		if (!formData.ciudad.trim()) {
			error = 'La ciudad es obligatoria';
			return;
		}

		if (!formData.departamento.trim()) {
			error = 'El departamento es obligatorio';
			return;
		}

		loading = true;

		try {
			if (modalMode === 'crear') {
				const request: CrearDireccionRequest = {
					tipo: formData.tipo,
					nombre_completo: formData.nombre_completo.trim() || undefined,
					direccion_linea1: formData.direccion_linea1.trim(),
					direccion_linea2: formData.direccion_linea2.trim() || undefined,
					ciudad: formData.ciudad.trim(),
					departamento: formData.departamento.trim(),
					codigo_postal: formData.codigo_postal.trim() || undefined,
					pais: formData.pais.trim(),
					telefono_contacto: formData.telefono_contacto.trim() || undefined,
					referencia: formData.referencia.trim() || undefined,
					es_predeterminada: formData.es_predeterminada
				};

				await direccionService.crearDireccion(request);
				success = 'Dirección creada exitosamente';
			} else if (modalMode === 'editar' && direccionEditando) {
				const request: ActualizarDireccionRequest = {
					tipo: formData.tipo,
					nombre_completo: formData.nombre_completo.trim() || undefined,
					direccion_linea1: formData.direccion_linea1.trim(),
					direccion_linea2: formData.direccion_linea2.trim() || undefined,
					ciudad: formData.ciudad.trim(),
					departamento: formData.departamento.trim(),
					codigo_postal: formData.codigo_postal.trim() || undefined,
					pais: formData.pais.trim(),
					telefono_contacto: formData.telefono_contacto.trim() || undefined,
					referencia: formData.referencia.trim() || undefined,
					es_predeterminada: formData.es_predeterminada
				};

				await direccionService.actualizarDireccion(direccionEditando.id_direccion, request);
				success = 'Dirección actualizada exitosamente';
			}

			await cargarDirecciones();
			cerrarModal();

			// Limpiar mensaje después de 3 segundos
			setTimeout(() => {
				success = '';
			}, 3000);
		} catch (err: any) {
			error = err.message || 'Error al guardar dirección';
		} finally {
			loading = false;
		}
	}

	function confirmarEliminar(direccion: Direccion) {
		direccionEliminar = direccion;
		showDeleteConfirm = true;
	}

	async function eliminarDireccion() {
		if (!direccionEliminar) return;

		loading = true;
		error = '';
		success = '';

		try {
			await direccionService.eliminarDireccion(direccionEliminar.id_direccion);
			success = 'Dirección eliminada exitosamente';
			await cargarDirecciones();
			showDeleteConfirm = false;
			direccionEliminar = null;

			// Limpiar mensaje después de 3 segundos
			setTimeout(() => {
				success = '';
			}, 3000);
		} catch (err: any) {
			error = err.message || 'Error al eliminar dirección';
			showDeleteConfirm = false;
		} finally {
			loading = false;
		}
	}

	function getTipoBadgeClass(tipo: string): string {
		const classes = {
			envio: 'bg-blue-100 dark:bg-blue-900/40 text-blue-700 dark:text-blue-300',
			facturacion: 'bg-purple-100 dark:bg-purple-900/40 text-purple-700 dark:text-purple-300',
			ambos: 'bg-green-100 dark:bg-green-900/40 text-green-700 dark:text-green-300'
		};
		return classes[tipo as keyof typeof classes] || classes.envio;
	}

	function getTipoLabel(tipo: string): string {
		const labels = {
			envio: 'Envío',
			facturacion: 'Facturación',
			ambos: 'Envío y Facturación'
		};
		return labels[tipo as keyof typeof labels] || tipo;
	}
</script>

<svelte:head>
	<title>Mis Direcciones | KronosTech</title>
</svelte:head>

<div class="space-y-6">
	<!-- Header -->
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-3xl font-bold tracking-tight text-text-light dark:text-text-dark">
				Mis Direcciones
			</h1>
			<p class="text-sm text-slate-600 dark:text-slate-400 mt-1">
				Gestiona tus direcciones de envío y facturación
			</p>
		</div>
		<button
			on:click={abrirModalCrear}
			class="flex items-center gap-2 px-4 py-2 rounded-lg bg-primary text-white hover:bg-primary/90 transition-colors"
		>
			<Plus size={18} />
			Nueva Dirección
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

	<!-- Loading -->
	{#if loading && !showModal && !showDeleteConfirm}
		<div class="flex items-center justify-center py-12">
			<div class="animate-spin rounded-full h-12 w-12 border-b-2 border-primary"></div>
		</div>
	{/if}

	<!-- Lista de Direcciones -->
	{#if !loading || showModal || showDeleteConfirm}
		{#if direcciones.length === 0}
			<div class="text-center py-12 rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50">
				<MapPin class="mx-auto text-slate-400 mb-4" size={48} />
				<h3 class="text-lg font-semibold text-text-light dark:text-text-dark mb-2">
					No tienes direcciones guardadas
				</h3>
				<p class="text-sm text-slate-600 dark:text-slate-400 mb-6">
					Agrega una dirección para facilitar tus compras
				</p>
				<button
					on:click={abrirModalCrear}
					class="inline-flex items-center gap-2 px-6 py-3 rounded-lg bg-primary text-white hover:bg-primary/90 transition-colors"
				>
					<Plus size={18} />
					Agregar Dirección
				</button>
			</div>
		{:else}
			<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
				{#each direcciones as direccion}
					<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 shadow-sm overflow-hidden hover:shadow-md transition-shadow">
						<!-- Header -->
						<div class="p-4 border-b border-border-light dark:border-border-dark bg-slate-50 dark:bg-slate-800/30">
							<div class="flex items-start justify-between gap-3">
								<div class="flex items-start gap-3 flex-1">
									<div class="w-10 h-10 rounded-lg bg-primary/10 dark:bg-primary/20 flex items-center justify-center flex-shrink-0">
										<MapPin class="text-primary" size={20} />
									</div>
									<div class="flex-1">
										<div class="flex items-center gap-2 mb-1">
											{#if direccion.es_predeterminada}
												<span class="inline-flex items-center gap-1 px-2 py-0.5 rounded text-xs font-medium bg-yellow-100 dark:bg-yellow-900/40 text-yellow-700 dark:text-yellow-300">
													<Star size={12} />
													Predeterminada
												</span>
											{/if}
											<span class="px-2 py-0.5 rounded text-xs font-medium {getTipoBadgeClass(direccion.tipo)}">
												{getTipoLabel(direccion.tipo)}
											</span>
										</div>
										{#if direccion.nombre_completo}
											<p class="font-semibold text-text-light dark:text-text-dark">
												{direccion.nombre_completo}
											</p>
										{/if}
									</div>
								</div>

								<div class="flex items-center gap-2">
									<button
										on:click={() => abrirModalEditar(direccion)}
										class="p-2 hover:bg-slate-100 dark:hover:bg-slate-700 rounded-lg transition-colors"
										title="Editar dirección"
									>
										<Edit2 size={16} class="text-slate-600 dark:text-slate-400" />
									</button>
									<button
										on:click={() => confirmarEliminar(direccion)}
										class="p-2 hover:bg-red-50 dark:hover:bg-red-900/20 rounded-lg transition-colors"
										title="Eliminar dirección"
									>
										<Trash2 size={16} class="text-red-600 dark:text-red-400" />
									</button>
								</div>
							</div>
						</div>

						<!-- Contenido -->
						<div class="p-4 space-y-2 text-sm">
							<p class="text-text-light dark:text-text-dark">
								{direccion.direccion_linea1}
							</p>
							{#if direccion.direccion_linea2}
								<p class="text-text-light dark:text-text-dark">
									{direccion.direccion_linea2}
								</p>
							{/if}
							<p class="text-slate-600 dark:text-slate-400">
								{direccion.ciudad}, {direccion.departamento}
							</p>
							{#if direccion.codigo_postal}
								<p class="text-slate-600 dark:text-slate-400">
									CP: {direccion.codigo_postal}
								</p>
							{/if}
							<p class="text-slate-600 dark:text-slate-400">
								{direccion.pais}
							</p>
							{#if direccion.telefono_contacto}
								<p class="text-slate-600 dark:text-slate-400">
									Tel: {direccion.telefono_contacto}
								</p>
							{/if}
							{#if direccion.referencia}
								<p class="text-xs text-slate-500 dark:text-slate-500 italic mt-2">
									Ref: {direccion.referencia}
								</p>
							{/if}
						</div>
					</div>
				{/each}
			</div>
		{/if}
	{/if}
</div>

<!-- Modal Crear/Editar -->
{#if showModal}
	<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
		<div class="bg-white dark:bg-slate-800 rounded-xl max-w-2xl w-full max-h-[90vh] overflow-y-auto">
			<!-- Header -->
			<div class="p-6 border-b border-border-light dark:border-border-dark flex items-center justify-between sticky top-0 bg-white dark:bg-slate-800 z-10">
				<h2 class="text-xl font-semibold text-text-light dark:text-text-dark">
					{modalMode === 'crear' ? 'Nueva Dirección' : 'Editar Dirección'}
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
				<!-- Tipo de Dirección -->
				<div>
					<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
						Tipo de Dirección *
					</label>
					<select bind:value={formData.tipo} class="form-input w-full" required>
						<option value="envio">Envío</option>
						<option value="facturacion">Facturación</option>
						<option value="ambos">Envío y Facturación</option>
					</select>
				</div>

				<!-- Nombre Completo -->
				<div>
					<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
						Nombre Completo
					</label>
					<input
						type="text"
						bind:value={formData.nombre_completo}
						class="form-input w-full"
						placeholder="Ej: Juan Pérez"
					/>
				</div>

				<!-- Dirección Línea 1 -->
				<div>
					<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
						Dirección *
					</label>
					<input
						type="text"
						bind:value={formData.direccion_linea1}
						class="form-input w-full"
						placeholder="Ej: Av. Principal 123"
						required
					/>
				</div>

				<!-- Dirección Línea 2 -->
				<div>
					<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
						Dirección Línea 2 (Opcional)
					</label>
					<input
						type="text"
						bind:value={formData.direccion_linea2}
						class="form-input w-full"
						placeholder="Ej: Depto 101, Piso 2"
					/>
				</div>

				<!-- Ciudad y Departamento -->
				<div class="grid grid-cols-2 gap-4">
					<div>
						<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
							Ciudad *
						</label>
						<input
							type="text"
							bind:value={formData.ciudad}
							class="form-input w-full"
							placeholder="Ej: Lima"
							required
						/>
					</div>
					<div>
						<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
							Departamento *
						</label>
						<input
							type="text"
							bind:value={formData.departamento}
							class="form-input w-full"
							placeholder="Ej: Lima"
							required
						/>
					</div>
				</div>

				<!-- Código Postal y País -->
				<div class="grid grid-cols-2 gap-4">
					<div>
						<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
							Código Postal
						</label>
						<input
							type="text"
							bind:value={formData.codigo_postal}
							class="form-input w-full"
							placeholder="Ej: 15001"
						/>
					</div>
					<div>
						<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
							País
						</label>
						<input
							type="text"
							bind:value={formData.pais}
							class="form-input w-full"
							placeholder="Perú"
						/>
					</div>
				</div>

				<!-- Teléfono -->
				<div>
					<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
						Teléfono de Contacto
					</label>
					<input
						type="tel"
						bind:value={formData.telefono_contacto}
						class="form-input w-full"
						placeholder="999999999"
					/>
				</div>

				<!-- Referencia -->
				<div>
					<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
						Referencia
					</label>
					<textarea
						bind:value={formData.referencia}
						class="form-input w-full"
						rows="2"
						placeholder="Ej: Casa blanca con portón negro, frente al parque"
					></textarea>
				</div>

				<!-- Predeterminada -->
				<div class="flex items-center gap-2">
					<input
						type="checkbox"
						bind:checked={formData.es_predeterminada}
						id="predeterminada"
						class="w-4 h-4 text-primary border-gray-300 rounded focus:ring-primary"
					/>
					<label for="predeterminada" class="text-sm text-text-light dark:text-text-dark">
						Establecer como dirección predeterminada
					</label>
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
						{:else if modalMode === 'crear'}
							Crear Dirección
						{:else}
							Actualizar Dirección
						{/if}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}

<!-- Modal Confirmar Eliminación -->
{#if showDeleteConfirm && direccionEliminar}
	<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
		<div class="bg-white dark:bg-slate-800 rounded-xl max-w-md w-full p-6">
			<div class="flex items-start gap-4 mb-6">
				<div class="w-12 h-12 rounded-full bg-red-100 dark:bg-red-900/40 flex items-center justify-center flex-shrink-0">
					<AlertCircle class="text-red-600 dark:text-red-400" size={24} />
				</div>
				<div>
					<h3 class="text-lg font-semibold text-text-light dark:text-text-dark mb-2">
						¿Eliminar dirección?
					</h3>
					<p class="text-sm text-slate-600 dark:text-slate-400">
						Esta acción no se puede deshacer. La dirección será eliminada permanentemente.
					</p>
				</div>
			</div>

			<div class="flex gap-3 justify-end">
				<button
					on:click={() => (showDeleteConfirm = false)}
					class="px-4 py-2 rounded-lg border border-border-light dark:border-border-dark hover:bg-slate-100 dark:hover:bg-slate-700 transition-colors"
					disabled={loading}
				>
					Cancelar
				</button>
				<button
					on:click={eliminarDireccion}
					class="px-4 py-2 rounded-lg bg-red-600 text-white hover:bg-red-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
					disabled={loading}
				>
					{#if loading}
						Eliminando...
					{:else}
						Eliminar
					{/if}
				</button>
			</div>
		</div>
	</div>
{/if}
