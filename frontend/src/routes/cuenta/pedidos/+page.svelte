<script lang="ts">
	import { onMount } from 'svelte';
	import { checkoutService, type Pedido } from '$lib/services/checkout';
	import { Package, Calendar, CreditCard, Truck, Eye, ChevronRight, AlertCircle } from 'lucide-svelte';

	let pedidos: Pedido[] = [];
	let loading = true;
	let error = '';

	onMount(async () => {
		await cargarPedidos();
	});

	async function cargarPedidos() {
		loading = true;
		error = '';

		try {
			pedidos = await checkoutService.getPedidos();
		} catch (err: any) {
			console.error('Error al cargar pedidos:', err);
			error = err.message || 'Error al cargar pedidos';
		} finally {
			loading = false;
		}
	}

	function formatDate(dateString: string): string {
		const date = new Date(dateString);
		return date.toLocaleDateString('es-ES', {
			year: 'numeric',
			month: 'long',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function formatPrice(price: number): string {
		return new Intl.NumberFormat('es-PE', {
			style: 'currency',
			currency: 'PEN'
		}).format(price);
	}

	function getEstadoBadgeClass(estado: string): string {
		const classes = {
			pendiente: 'bg-yellow-100 dark:bg-yellow-900/40 text-yellow-700 dark:text-yellow-300',
			confirmado: 'bg-blue-100 dark:bg-blue-900/40 text-blue-700 dark:text-blue-300',
			preparando: 'bg-purple-100 dark:bg-purple-900/40 text-purple-700 dark:text-purple-300',
			enviado: 'bg-indigo-100 dark:bg-indigo-900/40 text-indigo-700 dark:text-indigo-300',
			entregado: 'bg-green-100 dark:bg-green-900/40 text-green-700 dark:text-green-300',
			cancelado: 'bg-red-100 dark:bg-red-900/40 text-red-700 dark:text-red-300'
		};
		return classes[estado as keyof typeof classes] || classes.pendiente;
	}

	function getEstadoLabel(estado: string): string {
		const labels = {
			pendiente: 'Pendiente',
			confirmado: 'Confirmado',
			preparando: 'Preparando',
			enviado: 'Enviado',
			entregado: 'Entregado',
			cancelado: 'Cancelado'
		};
		return labels[estado as keyof typeof labels] || estado;
	}

	function getEstadoPagoClass(estado: string): string {
		const classes = {
			pendiente: 'text-yellow-600 dark:text-yellow-400',
			procesando: 'text-blue-600 dark:text-blue-400',
			completado: 'text-green-600 dark:text-green-400',
			fallido: 'text-red-600 dark:text-red-400',
			reembolsado: 'text-purple-600 dark:text-purple-400'
		};
		return classes[estado as keyof typeof classes] || classes.pendiente;
	}

	function getEstadoPagoLabel(estado: string): string {
		const labels = {
			pendiente: 'Pago Pendiente',
			procesando: 'Procesando Pago',
			completado: 'Pago Completado',
			fallido: 'Pago Fallido',
			reembolsado: 'Reembolsado'
		};
		return labels[estado as keyof typeof labels] || estado;
	}
</script>

<svelte:head>
	<title>Mis Pedidos | KronosTech</title>
</svelte:head>

<div class="space-y-6">
	<!-- Header -->
	<div>
		<h1 class="text-3xl font-bold tracking-tight text-text-light dark:text-text-dark">
			Mis Pedidos
		</h1>
		<p class="text-sm text-slate-600 dark:text-slate-400 mt-1">
			Historial completo de tus pedidos y compras
		</p>
	</div>

	<!-- Loading -->
	{#if loading}
		<div class="flex items-center justify-center py-12">
			<div class="animate-spin rounded-full h-12 w-12 border-b-2 border-primary"></div>
		</div>
	{/if}

	<!-- Error -->
	{#if error}
		<div class="rounded-lg bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 p-4 flex items-start gap-3">
			<AlertCircle class="text-red-600 dark:text-red-400 flex-shrink-0 mt-0.5" size={20} />
			<p class="text-sm text-red-800 dark:text-red-200">{error}</p>
		</div>
	{/if}

	<!-- Lista de Pedidos -->
	{#if !loading && !error}
		{#if pedidos.length === 0}
			<div class="text-center py-12 rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50">
				<Package class="mx-auto text-slate-400 mb-4" size={48} />
				<h3 class="text-lg font-semibold text-text-light dark:text-text-dark mb-2">
					No tienes pedidos aún
				</h3>
				<p class="text-sm text-slate-600 dark:text-slate-400 mb-6">
					Explora nuestro catálogo y realiza tu primera compra
				</p>
				<a
					href="/catalogo"
					class="inline-flex items-center gap-2 px-6 py-3 rounded-lg bg-primary text-white hover:bg-primary/90 transition-colors"
				>
					Ver Catálogo
					<ChevronRight size={18} />
				</a>
			</div>
		{:else}
			<div class="space-y-4">
				{#each pedidos as pedido}
					<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 shadow-sm overflow-hidden hover:shadow-md transition-shadow">
						<!-- Header del Pedido -->
						<div class="p-4 border-b border-border-light dark:border-border-dark bg-slate-50 dark:bg-slate-800/30">
							<div class="flex flex-wrap items-center justify-between gap-4">
								<div class="flex items-center gap-4">
									<div class="w-10 h-10 rounded-lg bg-primary/10 dark:bg-primary/20 flex items-center justify-center">
										<Package class="text-primary" size={20} />
									</div>
									<div>
										<h3 class="font-semibold text-text-light dark:text-text-dark">
											Pedido {pedido.numero_pedido}
										</h3>
										<div class="flex items-center gap-2 mt-1">
											<Calendar size={14} class="text-slate-400" />
											<p class="text-xs text-slate-600 dark:text-slate-400">
												{formatDate(pedido.fecha_pedido)}
											</p>
										</div>
									</div>
								</div>

								<div class="flex flex-wrap items-center gap-3">
									<span class="px-3 py-1 rounded-full text-xs font-medium {getEstadoBadgeClass(pedido.estado_pedido)}">
										{getEstadoLabel(pedido.estado_pedido)}
									</span>
									<a
										href="/pedido/{pedido.id_venta}"
										class="flex items-center gap-2 px-4 py-2 rounded-lg border border-border-light dark:border-border-dark hover:bg-slate-100 dark:hover:bg-slate-700 transition-colors text-sm"
									>
										<Eye size={16} />
										Ver Detalles
									</a>
								</div>
							</div>
						</div>

						<!-- Contenido del Pedido -->
						<div class="p-4">
							<div class="grid grid-cols-1 md:grid-cols-3 gap-4">
								<!-- Estado de Pago -->
								<div class="flex items-start gap-3">
									<div class="w-8 h-8 rounded-lg bg-blue-500/10 dark:bg-blue-500/20 flex items-center justify-center flex-shrink-0">
										<CreditCard class="text-blue-600 dark:text-blue-400" size={16} />
									</div>
									<div class="flex-1">
										<p class="text-xs text-slate-600 dark:text-slate-400 uppercase tracking-wide mb-1">
											Estado de Pago
										</p>
										<p class="text-sm font-medium {getEstadoPagoClass(pedido.estado_pago)}">
											{getEstadoPagoLabel(pedido.estado_pago)}
										</p>
									</div>
								</div>

								<!-- Método de Envío -->
								<div class="flex items-start gap-3">
									<div class="w-8 h-8 rounded-lg bg-purple-500/10 dark:bg-purple-500/20 flex items-center justify-center flex-shrink-0">
										<Truck class="text-purple-600 dark:text-purple-400" size={16} />
									</div>
									<div class="flex-1">
										<p class="text-xs text-slate-600 dark:text-slate-400 uppercase tracking-wide mb-1">
											Método de Envío
										</p>
										<p class="text-sm font-medium text-text-light dark:text-text-dark">
											{pedido.metodo_envio || 'Estándar'}
										</p>
									</div>
								</div>

								<!-- Total -->
								<div class="flex items-start gap-3">
									<div class="w-8 h-8 rounded-lg bg-green-500/10 dark:bg-green-500/20 flex items-center justify-center flex-shrink-0">
										<CreditCard class="text-green-600 dark:text-green-400" size={16} />
									</div>
									<div class="flex-1">
										<p class="text-xs text-slate-600 dark:text-slate-400 uppercase tracking-wide mb-1">
											Total Pagado
										</p>
										<p class="text-lg font-bold text-primary">
											{formatPrice(pedido.total)}
										</p>
									</div>
								</div>
							</div>

							<!-- Dirección de Envío (si existe) -->
							{#if pedido.direccion_envio}
								<div class="mt-4 pt-4 border-t border-border-light dark:border-border-dark">
									<p class="text-xs text-slate-600 dark:text-slate-400 uppercase tracking-wide mb-2">
										Dirección de Envío
									</p>
									<p class="text-sm text-text-light dark:text-text-dark">
										{pedido.direccion_envio.direccion}, {pedido.direccion_envio.distrito}
										{#if pedido.direccion_envio.ciudad}, {pedido.direccion_envio.ciudad}{/if}
										{#if pedido.direccion_envio.provincia}, {pedido.direccion_envio.provincia}{/if}
									</p>
								</div>
							{/if}
						</div>
					</div>
				{/each}
			</div>
		{/if}
	{/if}
</div>
