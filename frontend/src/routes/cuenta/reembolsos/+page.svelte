<script lang="ts">
	import { onMount } from 'svelte';
	import { checkoutService, type Venta } from '$lib/services/checkout';
	import { reembolsoService, type Reembolso, type SolicitarReembolsoRequest } from '$lib/services/reembolso';
	import { RotateCcw, AlertCircle, CheckCircle, Clock, XCircle, ChevronRight } from 'lucide-svelte';

	let pedidos: Venta[] = [];
	let reembolsos: Reembolso[] = [];
	let loading = true;
	let showForm = false;
	let selectedPedido: Venta | null = null;

	// Formulario
	let tipoReembolso: 'total' | 'parcial' = 'total';
	let montoReembolso = 0;
	let motivo = '';
	let submitting = false;
	let error = '';
	let success = '';

	onMount(async () => {
		await loadData();
	});

	async function loadData() {
		loading = true;
		try {
			// Cargar pedidos y reembolsos en paralelo
			const [pedidosData, reembolsosData] = await Promise.all([
				checkoutService.getPedidos(),
				reembolsoService.getMisReembolsos()
			]);
			pedidos = pedidosData;
			reembolsos = reembolsosData;
		} catch (err: any) {
			console.error('Error al cargar datos:', err);
			error = err.message || 'Error al cargar datos';
		} finally {
			loading = false;
		}
	}

	function iniciarSolicitud(pedido: Venta) {
		// Verificar si ya tiene reembolso activo
		const tieneReembolso = reembolsos.some(
			r => r.id_venta === pedido.id_venta && ['solicitado', 'procesando', 'completado'].includes(r.estado || '')
		);

		if (tieneReembolso) {
			error = 'Este pedido ya tiene una solicitud de reembolso activa';
			return;
		}

		selectedPedido = pedido;
		showForm = true;
		tipoReembolso = 'total';
		montoReembolso = pedido.total;
		motivo = '';
		error = '';
		success = '';
	}

	function cancelarSolicitud() {
		showForm = false;
		selectedPedido = null;
		tipoReembolso = 'total';
		montoReembolso = 0;
		motivo = '';
		error = '';
		success = '';
	}

	async function submitReembolso() {
		if (!selectedPedido) return;

		error = '';
		success = '';

		// Validaciones
		if (!motivo.trim()) {
			error = 'Debe especificar el motivo del reembolso';
			return;
		}

		if (montoReembolso <= 0) {
			error = 'El monto debe ser mayor a 0';
			return;
		}

		if (montoReembolso > selectedPedido.total) {
			error = `El monto no puede exceder el total del pedido (S/. ${selectedPedido.total.toFixed(2)})`;
			return;
		}

		submitting = true;

		try {
			const request: SolicitarReembolsoRequest = {
				id_venta: selectedPedido.id_venta,
				tipo_reembolso: tipoReembolso,
				monto_reembolsado: montoReembolso,
				motivo: motivo.trim()
			};

			const response = await reembolsoService.solicitarReembolso(request);

			if (response.success) {
				success = response.message;
				// Recargar datos
				await loadData();
				// Cerrar formulario después de 2 segundos
				setTimeout(() => {
					cancelarSolicitud();
				}, 2000);
			} else {
				error = response.message || 'Error al solicitar reembolso';
			}
		} catch (err: any) {
			error = err.message || 'Error al solicitar reembolso';
		} finally {
			submitting = false;
		}
	}

	function getEstadoBadgeClass(estado?: string): string {
		const estadoLower = estado?.toLowerCase() || '';
		const map: Record<string, string> = {
			solicitado: 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900/40 dark:text-yellow-300 border border-yellow-200',
			procesando: 'bg-blue-100 text-blue-800 dark:bg-blue-900/40 dark:text-blue-300 border border-blue-200',
			completado: 'bg-green-100 text-green-800 dark:bg-green-900/40 dark:text-green-300 border border-green-200',
			rechazado: 'bg-red-100 text-red-800 dark:bg-red-900/40 dark:text-red-300 border border-red-200',
			cancelado: 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300 border border-gray-200'
		};
		return map[estadoLower] || 'bg-gray-100 text-gray-800';
	}

	function getEstadoIcon(estado?: string) {
		const estadoLower = estado?.toLowerCase() || '';
		switch (estadoLower) {
			case 'solicitado':
				return Clock;
			case 'completado':
				return CheckCircle;
			case 'rechazado':
			case 'cancelado':
				return XCircle;
			default:
				return AlertCircle;
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

	$: if (tipoReembolso === 'total' && selectedPedido) {
		montoReembolso = selectedPedido.total;
	}
</script>

<svelte:head>
	<title>Mis Reembolsos | KronosTech</title>
</svelte:head>

<div class="space-y-6">
	<!-- Header -->
	<div>
		<h1 class="text-3xl font-bold tracking-tight text-text-light dark:text-text-dark">
			Mis Reembolsos
		</h1>
		<p class="text-sm text-slate-600 dark:text-slate-400 mt-1">
			Solicita reembolsos de tus pedidos y consulta su estado
		</p>
	</div>

	{#if loading}
		<!-- Loading -->
		<div class="flex items-center justify-center py-20">
			<div class="text-center">
				<div class="animate-spin rounded-full h-12 w-12 border-b-2 border-primary mx-auto mb-4"></div>
				<p class="text-slate-600 dark:text-slate-400">Cargando información...</p>
			</div>
		</div>
	{:else}
		<!-- Formulario de Solicitud (Modal) -->
		{#if showForm && selectedPedido}
			<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm p-4">
				<div class="bg-surface-light dark:bg-slate-800 rounded-xl shadow-2xl max-w-2xl w-full max-h-[90vh] overflow-y-auto">
					<div class="p-6 border-b border-border-light dark:border-border-dark">
						<h2 class="text-2xl font-bold text-text-light dark:text-text-dark">
							Solicitar Reembolso
						</h2>
						<p class="text-sm text-slate-600 dark:text-slate-400 mt-1">
							Pedido #{selectedPedido.numero_pedido} - S/. {selectedPedido.total.toFixed(2)}
						</p>
					</div>

					<div class="p-6 space-y-5">
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

						<!-- Tipo de Reembolso -->
						<div>
							<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
								Tipo de Reembolso *
							</label>
							<div class="grid grid-cols-2 gap-3">
								<button
									type="button"
									class="p-4 rounded-lg border-2 transition-all {tipoReembolso === 'total'
										? 'border-primary bg-primary/10 dark:bg-primary/20'
										: 'border-border-light dark:border-border-dark hover:border-primary/50'}"
									on:click={() => tipoReembolso = 'total'}
								>
									<p class="font-semibold text-text-light dark:text-text-dark">Reembolso Total</p>
									<p class="text-xs text-slate-600 dark:text-slate-400 mt-1">
										S/. {selectedPedido.total.toFixed(2)}
									</p>
								</button>
								<button
									type="button"
									class="p-4 rounded-lg border-2 transition-all {tipoReembolso === 'parcial'
										? 'border-primary bg-primary/10 dark:bg-primary/20'
										: 'border-border-light dark:border-border-dark hover:border-primary/50'}"
									on:click={() => tipoReembolso = 'parcial'}
								>
									<p class="font-semibold text-text-light dark:text-text-dark">Reembolso Parcial</p>
									<p class="text-xs text-slate-600 dark:text-slate-400 mt-1">
										Especifica el monto
									</p>
								</button>
							</div>
						</div>

						<!-- Monto (solo si es parcial) -->
						{#if tipoReembolso === 'parcial'}
							<div>
								<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
									Monto a Reembolsar *
								</label>
								<div class="relative">
									<span class="absolute left-3 top-1/2 -translate-y-1/2 text-slate-500">S/.</span>
									<input
										type="number"
										step="0.01"
										min="0.01"
										max={selectedPedido.total}
										bind:value={montoReembolso}
										class="form-input w-full pl-12"
										placeholder="0.00"
									/>
								</div>
								<p class="text-xs text-slate-600 dark:text-slate-400 mt-1">
									Máximo: S/. {selectedPedido.total.toFixed(2)}
								</p>
							</div>
						{/if}

						<!-- Motivo -->
						<div>
							<label class="block text-sm font-medium text-text-light dark:text-text-dark mb-2">
								Motivo del Reembolso *
							</label>
							<textarea
								bind:value={motivo}
								rows="4"
								class="form-input w-full resize-none"
								placeholder="Explica el motivo de tu solicitud..."
							></textarea>
							<p class="text-xs text-slate-600 dark:text-slate-400 mt-1">
								Describe claramente el motivo de tu solicitud
							</p>
						</div>
					</div>

					<!-- Footer -->
					<div class="p-6 border-t border-border-light dark:border-border-dark flex gap-3 justify-end">
						<button
							type="button"
							class="px-4 py-2 rounded-lg border border-border-light dark:border-border-dark hover:bg-slate-100 dark:hover:bg-slate-700 transition-colors"
							on:click={cancelarSolicitud}
							disabled={submitting}
						>
							Cancelar
						</button>
						<button
							type="button"
							class="px-4 py-2 rounded-lg bg-primary text-white hover:bg-primary/90 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
							on:click={submitReembolso}
							disabled={submitting}
						>
							{#if submitting}
								Procesando...
							{:else}
								Solicitar Reembolso
							{/if}
						</button>
					</div>
				</div>
			</div>
		{/if}

		<!-- Reembolsos Existentes -->
		{#if reembolsos.length > 0}
			<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 shadow-sm overflow-hidden">
				<div class="px-6 py-4 border-b border-border-light dark:border-border-dark">
					<h2 class="text-lg font-semibold text-text-light dark:text-text-dark">
						Solicitudes de Reembolso
					</h2>
					<p class="text-xs text-slate-600 dark:text-slate-400 mt-0.5">
						{reembolsos.length} solicitud{reembolsos.length !== 1 ? 'es' : ''}
					</p>
				</div>

				<div class="divide-y divide-border-light dark:divide-border-dark">
					{#each reembolsos as reembolso}
						<div class="p-6 hover:bg-slate-50 dark:hover:bg-slate-700/30 transition-colors">
							<div class="flex items-start justify-between gap-4">
								<div class="flex-1">
									<div class="flex items-center gap-3 mb-2">
										<p class="font-semibold text-text-light dark:text-text-dark">
											Pedido #{reembolso.numero_pedido}
										</p>
										<span class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-xs font-medium {getEstadoBadgeClass(reembolso.estado)}">
											<svelte:component this={getEstadoIcon(reembolso.estado)} size={14} />
											{reembolso.estado}
										</span>
									</div>
									<div class="grid grid-cols-2 gap-4 text-sm">
										<div>
											<p class="text-slate-600 dark:text-slate-400">Tipo</p>
											<p class="font-medium text-text-light dark:text-text-dark capitalize">
												{reembolso.tipo_reembolso}
											</p>
										</div>
										<div>
											<p class="text-slate-600 dark:text-slate-400">Monto</p>
											<p class="font-medium text-text-light dark:text-text-dark">
												S/. {reembolso.monto_reembolsado.toFixed(2)}
											</p>
										</div>
										<div>
											<p class="text-slate-600 dark:text-slate-400">Fecha solicitud</p>
											<p class="font-medium text-text-light dark:text-text-dark">
												{formatDate(reembolso.fecha_solicitado)}
											</p>
										</div>
									</div>
								</div>
							</div>
						</div>
					{/each}
				</div>
			</div>
		{/if}

		<!-- Pedidos Disponibles -->
		<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 shadow-sm overflow-hidden">
			<div class="px-6 py-4 border-b border-border-light dark:border-border-dark">
				<h2 class="text-lg font-semibold text-text-light dark:text-text-dark">
					Solicitar Nuevo Reembolso
				</h2>
				<p class="text-xs text-slate-600 dark:text-slate-400 mt-0.5">
					Selecciona un pedido para solicitar reembolso
				</p>
			</div>

			<div class="p-6">
				{#if pedidos.length === 0}
					<div class="text-center py-12">
						<div class="w-16 h-16 rounded-xl border border-border-light dark:border-border-dark bg-slate-100 dark:bg-slate-800 flex items-center justify-center mx-auto mb-4">
							<RotateCcw size={32} class="text-slate-400 dark:text-slate-500" />
						</div>
						<p class="text-lg font-semibold text-text-light dark:text-text-dark mb-2">
							No tienes pedidos
						</p>
						<p class="text-sm text-slate-600 dark:text-slate-400">
							Realiza una compra para poder solicitar reembolsos
						</p>
					</div>
				{:else}
					<div class="space-y-3">
						{#each pedidos as pedido}
							{@const tieneReembolso = reembolsos.some(
								r => r.id_venta === pedido.id_venta && ['solicitado', 'procesando', 'completado'].includes(r.estado || '')
							)}
							<button
								type="button"
								class="w-full p-4 rounded-lg border border-border-light dark:border-border-dark hover:bg-slate-50 dark:hover:bg-slate-700/30 transition-colors text-left disabled:opacity-50 disabled:cursor-not-allowed"
								on:click={() => iniciarSolicitud(pedido)}
								disabled={tieneReembolso}
							>
								<div class="flex items-center justify-between">
									<div class="flex-1">
										<div class="flex items-center gap-3 mb-2">
											<p class="font-semibold text-text-light dark:text-text-dark">
												#{pedido.numero_pedido}
											</p>
											{#if tieneReembolso}
												<span class="px-2 py-0.5 rounded text-xs font-medium bg-blue-100 dark:bg-blue-900/40 text-blue-700 dark:text-blue-300">
													Reembolso solicitado
												</span>
											{/if}
										</div>
										<div class="flex items-center gap-4 text-sm">
											<div>
												<span class="text-slate-600 dark:text-slate-400">Fecha:</span>
												<span class="font-medium text-text-light dark:text-text-dark ml-1">
													{formatDate(pedido.fecha_pedido)}
												</span>
											</div>
											<div>
												<span class="text-slate-600 dark:text-slate-400">Total:</span>
												<span class="font-medium text-text-light dark:text-text-dark ml-1">
													S/. {pedido.total.toFixed(2)}
												</span>
											</div>
										</div>
									</div>
									{#if !tieneReembolso}
										<ChevronRight class="text-slate-400" size={20} />
									{/if}
								</div>
							</button>
						{/each}
					</div>
				{/if}
			</div>
		</div>
	{/if}
</div>
