<script lang="ts">
	import { onMount } from 'svelte';
	import {
		tarjetaService,
		type MetodoPagoCliente,
		type CrearMetodoPagoRequest,
		type ActualizarMetodoPagoRequest
	} from '$lib/services/tarjeta';
	import { CreditCard, Plus, Edit2, Trash2, CheckCircle, AlertCircle, Star, X } from 'lucide-svelte';

	let metodosPago: MetodoPagoCliente[] = [];
	let loading = true;
	let error = '';
	let success = '';

	// Modal state
	let showModal = false;
	let modalMode: 'crear' | 'editar' = 'crear';
	let metodoPagoEditando: MetodoPagoCliente | null = null;

	// Confirmación de eliminación
	let showDeleteConfirm = false;
	let metodoPagoEliminar: MetodoPagoCliente | null = null;

	// Formulario (id_metodo_pago 1 = Tarjeta de Crédito, asumiendo)
	let formData = {
		id_metodo_pago: 1,
		tipo: 'credito',
		token_pago: '',
		ultimos_4_digitos: '',
		marca: 'Visa',
		fecha_expiracion: '',
		nombre_titular: '',
		es_predeterminado: false
	};

	onMount(async () => {
		await cargarMetodosPago();
	});

	async function cargarMetodosPago() {
		loading = true;
		error = '';

		try {
			metodosPago = await tarjetaService.getMetodosPago();
		} catch (err: any) {
			console.error('Error al cargar métodos de pago:', err);
			error = err.message || 'Error al cargar métodos de pago';
		} finally {
			loading = false;
		}
	}

	function abrirModalCrear() {
		modalMode = 'crear';
		metodoPagoEditando = null;
		resetFormulario();
		showModal = true;
		error = '';
		success = '';
	}

	function abrirModalEditar(metodoPago: MetodoPagoCliente) {
		modalMode = 'editar';
		metodoPagoEditando = metodoPago;
		formData = {
			id_metodo_pago: metodoPago.id_metodo_pago,
			tipo: metodoPago.tipo,
			token_pago: metodoPago.token_pago || '',
			ultimos_4_digitos: metodoPago.ultimos_4_digitos || '',
			marca: metodoPago.marca || 'Visa',
			fecha_expiracion: metodoPago.fecha_expiracion || '',
			nombre_titular: metodoPago.nombre_titular || '',
			es_predeterminado: metodoPago.es_predeterminado || false
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
			id_metodo_pago: 1,
			tipo: 'credito',
			token_pago: '',
			ultimos_4_digitos: '',
			marca: 'Visa',
			fecha_expiracion: '',
			nombre_titular: '',
			es_predeterminado: false
		};
	}

	async function handleSubmit() {
		error = '';
		success = '';

		// Validaciones
		if (!formData.ultimos_4_digitos || formData.ultimos_4_digitos.length !== 4) {
			error = 'Los últimos 4 dígitos deben ser exactamente 4 números';
			return;
		}

		if (!formData.fecha_expiracion) {
			error = 'La fecha de expiración es obligatoria';
			return;
		}

		// Validar formato MM/YY
		const expRegex = /^(0[1-9]|1[0-2])\/\d{2}$/;
		if (!expRegex.test(formData.fecha_expiracion)) {
			error = 'La fecha de expiración debe tener formato MM/YY';
			return;
		}

		// Validar que la tarjeta no esté vencida
		const [mes, anio] = formData.fecha_expiracion.split('/');
		const fechaExp = new Date(2000 + parseInt(anio), parseInt(mes) - 1);
		const ahora = new Date();
		if (fechaExp < ahora) {
			error = 'La tarjeta está vencida';
			return;
		}

		if (!formData.nombre_titular.trim()) {
			error = 'El nombre del titular es obligatorio';
			return;
		}

		loading = true;

		try {
			if (modalMode === 'crear') {
				const request: CrearMetodoPagoRequest = {
					id_metodo_pago: formData.id_metodo_pago,
					tipo: formData.tipo,
					token_pago: formData.token_pago.trim() || undefined,
					ultimos_4_digitos: formData.ultimos_4_digitos.trim(),
					marca: formData.marca.trim(),
					fecha_expiracion: formData.fecha_expiracion.trim(),
					nombre_titular: formData.nombre_titular.trim(),
					es_predeterminado: formData.es_predeterminado
				};

				await tarjetaService.crearMetodoPago(request);
				success = 'Tarjeta agregada exitosamente';
			} else if (modalMode === 'editar' && metodoPagoEditando) {
				const request: ActualizarMetodoPagoRequest = {
					ultimos_4_digitos: formData.ultimos_4_digitos.trim(),
					marca: formData.marca.trim(),
					fecha_expiracion: formData.fecha_expiracion.trim(),
					nombre_titular: formData.nombre_titular.trim(),
					es_predeterminado: formData.es_predeterminado
				};

				await tarjetaService.actualizarMetodoPago(metodoPagoEditando.id_metodo_pago_cliente, request);
				success = 'Tarjeta actualizada exitosamente';
			}

			await cargarMetodosPago();
			cerrarModal();

			// Limpiar mensaje después de 3 segundos
			setTimeout(() => {
				success = '';
			}, 3000);
		} catch (err: any) {
			error = err.message || 'Error al guardar tarjeta';
		} finally {
			loading = false;
		}
	}

	function confirmarEliminar(metodoPago: MetodoPagoCliente) {
		metodoPagoEliminar = metodoPago;
		showDeleteConfirm = true;
	}

	async function eliminarMetodoPago() {
		if (!metodoPagoEliminar) return;

		loading = true;
		error = '';
		success = '';

		try {
			await tarjetaService.eliminarMetodoPago(metodoPagoEliminar.id_metodo_pago_cliente);
			success = 'Tarjeta eliminada exitosamente';
			await cargarMetodosPago();
			showDeleteConfirm = false;
			metodoPagoEliminar = null;

			// Limpiar mensaje después de 3 segundos
			setTimeout(() => {
				success = '';
			}, 3000);
		} catch (err: any) {
			error = err.message || 'Error al eliminar tarjeta';
			showDeleteConfirm = false;
		} finally {
			loading = false;
		}
	}

	function getMarcaIcon(marca?: string): string {
		if (!marca) return '💳';
		const marcaLower = marca.toLowerCase();
		if (marcaLower.includes('visa')) return '💳 Visa';
		if (marcaLower.includes('mastercard')) return '💳 Mastercard';
		if (marcaLower.includes('american')) return '💳 AmEx';
		return `💳 ${marca}`;
	}

	function formatFechaCreacion(fecha?: string): string {
		if (!fecha) return '';
		const date = new Date(fecha);
		return date.toLocaleDateString('es-ES', {
			year: 'numeric',
			month: 'long'
		});
	}
</script>

<svelte:head>
	<title>Mis Tarjetas | KronosTech</title>
</svelte:head>

<div class="space-y-6">
	<!-- Header -->
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-3xl font-bold tracking-tight text-text-light dark:text-text-dark">
				Mis Tarjetas
			</h1>
			<p class="text-sm text-slate-600 dark:text-slate-400 mt-1">
				Gestiona tus métodos de pago de forma segura
			</p>
		</div>
		<button
			on:click={abrirModalCrear}
			class="flex items-center gap-2 px-4 py-2 rounded-lg bg-primary text-white hover:bg-primary/90 transition-colors"
		>
			<Plus size={18} />
			Agregar Tarjeta
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

	<!-- Nota de Seguridad -->
	<div class="rounded-lg bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 p-4">
		<p class="text-sm text-blue-800 dark:text-blue-200">
			🔒 Tus datos de pago están encriptados y protegidos. Solo almacenamos los últimos 4 dígitos de tu tarjeta.
		</p>
	</div>

	<!-- Loading -->
	{#if loading && !showModal && !showDeleteConfirm}
		<div class="flex items-center justify-center py-12">
			<div class="animate-spin rounded-full h-12 w-12 border-b-2 border-primary"></div>
		</div>
	{/if}

	<!-- Lista de Tarjetas -->
	{#if !loading || showModal || showDeleteConfirm}
		{#if metodosPago.length === 0}
			<div class="text-center py-12 rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50">
				<CreditCard class="mx-auto text-slate-400 mb-4" size={48} />
				<h3 class="text-lg font-semibold text-text-light dark:text-text-dark mb-2">
					No tienes tarjetas guardadas
				</h3>
				<p class="text-sm text-slate-600 dark:text-slate-400 mb-6">
					Agrega una tarjeta para agilizar tus compras
				</p>
				<button
					on:click={abrirModalCrear}
					class="inline-flex items-center gap-2 px-6 py-3 rounded-lg bg-primary text-white hover:bg-primary/90 transition-colors"
				>
					<Plus size={18} />
					Agregar Tarjeta
				</button>
			</div>
		{:else}
			<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
				{#each metodosPago as metodoPago}
					<div class="rounded-xl border border-border-light dark:border-border-dark bg-gradient-to-br from-blue-600 to-purple-600 shadow-lg overflow-hidden hover:shadow-xl transition-shadow">
						<!-- Header -->
						<div class="p-4 text-white">
							<div class="flex items-start justify-between mb-4">
								<div class="flex items-center gap-2">
									{#if metodoPago.es_predeterminado}
										<span class="inline-flex items-center gap-1 px-2 py-0.5 rounded text-xs font-medium bg-yellow-400 text-yellow-900">
											<Star size={12} />
											Predeterminada
										</span>
									{/if}
								</div>
								<div class="flex items-center gap-2">
									<button
										on:click={() => abrirModalEditar(metodoPago)}
										class="p-2 hover:bg-white/20 rounded-lg transition-colors"
										title="Editar tarjeta"
									>
										<Edit2 size={16} />
									</button>
									<button
										on:click={() => confirmarEliminar(metodoPago)}
										class="p-2 hover:bg-white/20 rounded-lg transition-colors"
										title="Eliminar tarjeta"
									>
										<Trash2 size={16} />
									</button>
								</div>
							</div>

							<!-- Chip de Tarjeta (decorativo) -->
							<div class="mb-6">
								<div class="w-12 h-10 rounded bg-gradient-to-br from-yellow-300 to-yellow-500 opacity-80"></div>
							</div>

							<!-- Número de Tarjeta (parcial) -->
							<div class="mb-4 font-mono text-lg tracking-wider">
								•••• •••• •••• {metodoPago.ultimos_4_digitos || '****'}
							</div>

							<!-- Info -->
							<div class="flex items-end justify-between">
								<div>
									<p class="text-xs text-white/70 mb-1">Titular</p>
									<p class="font-semibold">{metodoPago.nombre_titular || 'Sin nombre'}</p>
								</div>
								<div class="text-right">
									<p class="text-xs text-white/70 mb-1">Vence</p>
									<p class="font-semibold">{metodoPago.fecha_expiracion || '--/--'}</p>
								</div>
							</div>
						</div>

						<!-- Footer -->
						<div class="px-4 py-3 bg-black/20 flex items-center justify-between">
							<span class="text-sm font-medium text-white">
								{getMarcaIcon(metodoPago.marca)}
							</span>
							<span class="text-xs text-white/70">
								Agregada {formatFechaCreacion(metodoPago.fecha_creacion)}
							</span>
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
		<div class="bg-white dark:bg-slate-800 rounded-xl max-w-md w-full max-h-[90vh] overflow-y-auto">
			<!-- Header -->
			<div class="p-6 border-b border-border-light dark:border-border-dark flex items-center justify-between sticky top-0 bg-white dark:bg-slate-800 z-10">
				<h2 class="text-xl font-semibold text-text-light dark:text-text-dark">
					{modalMode === 'crear' ? 'Agregar Tarjeta' : 'Editar Tarjeta'}
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
				<!-- Nombre del Titular -->
				<div>
					<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
						Nombre del Titular *
					</label>
					<input
						type="text"
						bind:value={formData.nombre_titular}
						class="form-input w-full"
						placeholder="NOMBRE APELLIDO"
						required
						disabled={loading}
					/>
				</div>

				<!-- Marca -->
				<div>
					<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
						Marca *
					</label>
					<select bind:value={formData.marca} class="form-input w-full" required disabled={loading}>
						<option value="Visa">Visa</option>
						<option value="Mastercard">Mastercard</option>
						<option value="American Express">American Express</option>
						<option value="Diners Club">Diners Club</option>
						<option value="Otra">Otra</option>
					</select>
				</div>

				<!-- Últimos 4 Dígitos -->
				<div>
					<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
						Últimos 4 Dígitos *
					</label>
					<input
						type="text"
						bind:value={formData.ultimos_4_digitos}
						class="form-input w-full"
						placeholder="1234"
						maxlength="4"
						pattern="[0-9]{4}"
						required
						disabled={loading}
					/>
					<p class="text-xs text-slate-600 dark:text-slate-400 mt-1">
						Solo los últimos 4 dígitos de tu tarjeta
					</p>
				</div>

				<!-- Fecha de Expiración -->
				<div>
					<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
						Fecha de Expiración *
					</label>
					<input
						type="text"
						bind:value={formData.fecha_expiracion}
						class="form-input w-full"
						placeholder="MM/YY"
						maxlength="5"
						pattern="(0[1-9]|1[0-2])\/\d{2}"
						required
						disabled={loading}
					/>
					<p class="text-xs text-slate-600 dark:text-slate-400 mt-1">
						Formato: MM/YY (ej: 12/25)
					</p>
				</div>

				<!-- Tipo -->
				<div>
					<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
						Tipo de Tarjeta
					</label>
					<select bind:value={formData.tipo} class="form-input w-full" disabled={loading}>
						<option value="credito">Crédito</option>
						<option value="debito">Débito</option>
					</select>
				</div>

				<!-- Predeterminada -->
				<div class="flex items-center gap-2">
					<input
						type="checkbox"
						bind:checked={formData.es_predeterminado}
						id="predeterminada"
						class="w-4 h-4 text-primary border-gray-300 rounded focus:ring-primary"
					/>
					<label for="predeterminada" class="text-sm text-text-light dark:text-text-dark">
						Establecer como tarjeta predeterminada
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
							Agregar Tarjeta
						{:else}
							Actualizar Tarjeta
						{/if}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}

<!-- Modal Confirmar Eliminación -->
{#if showDeleteConfirm && metodoPagoEliminar}
	<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
		<div class="bg-white dark:bg-slate-800 rounded-xl max-w-md w-full p-6">
			<div class="flex items-start gap-4 mb-6">
				<div class="w-12 h-12 rounded-full bg-red-100 dark:bg-red-900/40 flex items-center justify-center flex-shrink-0">
					<AlertCircle class="text-red-600 dark:text-red-400" size={24} />
				</div>
				<div>
					<h3 class="text-lg font-semibold text-text-light dark:text-text-dark mb-2">
						¿Eliminar tarjeta?
					</h3>
					<p class="text-sm text-slate-600 dark:text-slate-400">
						La tarjeta terminada en {metodoPagoEliminar.ultimos_4_digitos} será eliminada permanentemente.
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
					on:click={eliminarMetodoPago}
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
