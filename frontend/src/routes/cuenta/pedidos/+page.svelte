<script lang="ts">
	import { onMount } from 'svelte';
	import { checkoutService, type Venta } from '$lib/services/checkout';
	import { Package, Calendar, CreditCard, Truck, Eye, ChevronRight, AlertCircle, XCircle, MapPin } from 'lucide-svelte';

	let pedidos: Venta[] = [];
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
		return date.toLocaleDateString('es-PE', {
			timeZone: 'America/Lima',
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
		const normalized = estado?.toLowerCase() || 'pendiente';
		const classes: Record<string, string> = {
			pendiente: 'bg-yellow-100 dark:bg-yellow-900/40 text-yellow-700 dark:text-yellow-300 border border-yellow-200 dark:border-yellow-800',
			confirmado: 'bg-blue-100 dark:bg-blue-900/40 text-blue-700 dark:text-blue-300 border border-blue-200 dark:border-blue-800',
			preparando: 'bg-purple-100 dark:bg-purple-900/40 text-purple-700 dark:text-purple-300 border border-purple-200 dark:border-purple-800',
			procesando: 'bg-purple-100 dark:bg-purple-900/40 text-purple-700 dark:text-purple-300 border border-purple-200 dark:border-purple-800',
			enviado: 'bg-indigo-100 dark:bg-indigo-900/40 text-indigo-700 dark:text-indigo-300 border border-indigo-200 dark:border-indigo-800',
			entregado: 'bg-green-100 dark:bg-green-900/40 text-green-700 dark:text-green-300 border border-green-200 dark:border-green-800',
			cancelado: 'bg-red-100 dark:bg-red-900/40 text-red-700 dark:text-red-300 border border-red-200 dark:border-red-800',
			devuelto: 'bg-orange-100 dark:bg-orange-900/40 text-orange-700 dark:text-orange-300 border border-orange-200 dark:border-orange-800'
		};
		return classes[normalized] || classes.pendiente;
	}

	function getEstadoLabel(estado: string): string {
		const normalized = estado?.toLowerCase() || 'pendiente';
		const labels: Record<string, string> = {
			pendiente: 'Pendiente',
			confirmado: 'Confirmado',
			preparando: 'Preparando',
			procesando: 'Procesando',
			enviado: 'Enviado',
			entregado: 'Entregado',
			cancelado: 'Cancelado',
			devuelto: 'Devuelto'
		};
		return labels[normalized] || estado;
	}

	function getEstadoPagoClass(estado: string): string {
		const normalized = estado?.toLowerCase() || 'pendiente';
		const classes: Record<string, string> = {
			pendiente: 'text-yellow-600 dark:text-yellow-400',
			procesando: 'text-blue-600 dark:text-blue-400',
			completado: 'text-green-600 dark:text-green-400',
			fallido: 'text-red-600 dark:text-red-400',
			rechazado: 'text-red-600 dark:text-red-400',
			cancelado: 'text-slate-600 dark:text-slate-400',
			reembolsado: 'text-purple-600 dark:text-purple-400'
		};
		return classes[normalized] || classes.pendiente;
	}

	function getEstadoPagoLabel(estado: string): string {
		const normalized = estado?.toLowerCase() || 'pendiente';
		const labels: Record<string, string> = {
			pendiente: 'Pago Pendiente',
			procesando: 'Procesando Pago',
			completado: 'Pago Completado',
			fallido: 'Pago Fallido',
			rechazado: 'Pago Rechazado',
			cancelado: 'Pago Cancelado',
			reembolsado: 'Reembolsado'
		};
		return labels[normalized] || estado;
	}

	function formatDireccion(pedido: Venta): string {
		const partes = [];
		if (pedido.direccion_envio) partes.push(pedido.direccion_envio);
		if (pedido.ciudad) partes.push(pedido.ciudad);
		if (pedido.departamento) partes.push(pedido.departamento);
		if (pedido.codigo_postal) partes.push(`CP: ${pedido.codigo_postal}`);
		return partes.length > 0 ? partes.join(', ') : 'Dirección no disponible';
	}

	function isPedidoCancelado(estado: string): boolean {
		return estado?.toLowerCase() === 'cancelado';
	}
</script>

<svelte:head>
	<title>Mis Pedidos | KronosTech</title>
</svelte:head>

<div class="space-y-8">
	<!-- Header -->
	<div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
		<div>
			<h1 class="text-2xl sm:text-3xl font-bold tracking-tight text-text-light dark:text-text-dark">
				Mis Pedidos
			</h1>
			<p class="text-slate-600 dark:text-slate-400 mt-2">
				Historial completo de tus pedidos y compras
			</p>
		</div>
		<a href="/catalogo" class="inline-flex items-center justify-center gap-2 px-5 py-2.5 rounded-lg bg-primary text-white hover:bg-primary/90 transition-colors font-medium shadow-sm">
			<Package size={18} />
			Nueva Compra
		</a>
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
			<div class="text-center py-16 rounded-2xl border-2 border-dashed border-border-light dark:border-border-dark bg-gradient-to-b from-surface-light to-slate-50 dark:from-slate-800/50 dark:to-slate-900/50">
				<div class="w-20 h-20 mx-auto mb-6 rounded-2xl bg-slate-100 dark:bg-slate-700 flex items-center justify-center">
					<Package class="text-slate-400" size={40} />
				</div>
				<h3 class="text-xl font-bold text-text-light dark:text-text-dark mb-3">
					No tienes pedidos aún
				</h3>
				<p class="text-slate-600 dark:text-slate-400 mb-8 max-w-md mx-auto">
					Explora nuestro catálogo con los mejores componentes y realiza tu primera compra
				</p>
				<a
					href="/catalogo"
					class="inline-flex items-center gap-2 px-8 py-3.5 rounded-xl bg-primary text-white hover:bg-primary/90 transition-all shadow-lg shadow-primary/25 font-semibold"
				>
					Explorar Catálogo
					<ChevronRight size={20} />
				</a>
			</div>
		{:else}
			<div class="space-y-6">
				{#each pedidos as pedido (pedido.id_venta)}
					<div class="rounded-2xl border overflow-hidden hover:shadow-lg transition-all duration-300
						{isPedidoCancelado(pedido.estado) 
							? 'border-red-200 dark:border-red-900/50 bg-red-50/30 dark:bg-red-900/10' 
							: 'border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50'}">
						
						<!-- Banner de Cancelado (si aplica) -->
						{#if isPedidoCancelado(pedido.estado)}
							<div class="bg-red-100 dark:bg-red-900/30 px-5 py-3 border-b border-red-200 dark:border-red-800 flex items-center gap-3">
								<XCircle class="text-red-600 dark:text-red-400" size={20} />
								<p class="text-sm font-medium text-red-700 dark:text-red-300">
									Este pedido ha sido cancelado
								</p>
							</div>
						{/if}

						<!-- Header del Pedido -->
						<div class="p-5 sm:p-6 border-b border-border-light dark:border-border-dark 
							{isPedidoCancelado(pedido.estado) 
								? 'bg-gradient-to-r from-red-50/50 to-slate-50 dark:from-red-900/10 dark:to-slate-800/30' 
								: 'bg-gradient-to-r from-slate-50 to-slate-100 dark:from-slate-800/50 dark:to-slate-800/30'}">
							<div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
								<div class="flex items-center gap-4">
									<div class="w-14 h-14 rounded-xl flex items-center justify-center shadow-sm
										{isPedidoCancelado(pedido.estado) 
											? 'bg-red-100 dark:bg-red-900/30' 
											: 'bg-primary/10 dark:bg-primary/20'}">
										{#if isPedidoCancelado(pedido.estado)}
											<XCircle class="text-red-600 dark:text-red-400" size={28} />
										{:else}
											<Package class="text-primary" size={28} />
										{/if}
									</div>
									<div>
										<h3 class="text-lg font-bold text-text-light dark:text-text-dark">
											Pedido {pedido.numero_pedido}
										</h3>
										<div class="flex items-center gap-2 mt-1.5">
											<Calendar size={16} class="text-slate-400" />
											<p class="text-sm text-slate-600 dark:text-slate-400">
												{formatDate(pedido.fecha_pedido)}
											</p>
										</div>
									</div>
								</div>

								<div class="flex flex-wrap items-center gap-3">
									<span class="px-4 py-1.5 rounded-full text-sm font-semibold {getEstadoBadgeClass(pedido.estado)}">
										{getEstadoLabel(pedido.estado)}
									</span>
									<a
										href="/pedido/{pedido.id_venta}"
										class="flex items-center gap-2 px-5 py-2.5 rounded-xl bg-white dark:bg-slate-700 border border-border-light dark:border-border-dark hover:bg-slate-50 dark:hover:bg-slate-600 transition-colors text-sm font-medium shadow-sm"
									>
										<Eye size={18} />
										Ver Detalles
									</a>
								</div>
							</div>
						</div>

						<!-- Contenido del Pedido -->
						<div class="p-5 sm:p-6 {isPedidoCancelado(pedido.estado) ? 'opacity-75' : ''}">
							<div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
								<!-- Estado de Pago -->
								<div class="flex items-center gap-4 p-4 rounded-xl 
									{isPedidoCancelado(pedido.estado)
										? 'bg-slate-50 dark:bg-slate-800/50 border border-slate-200 dark:border-slate-700'
										: 'bg-blue-50/50 dark:bg-blue-900/10 border border-blue-100 dark:border-blue-900/30'}">
									<div class="w-12 h-12 rounded-xl flex items-center justify-center flex-shrink-0
										{isPedidoCancelado(pedido.estado) ? 'bg-slate-200 dark:bg-slate-700' : 'bg-blue-500/20'}">
										<CreditCard class="{isPedidoCancelado(pedido.estado) ? 'text-slate-500' : 'text-blue-600 dark:text-blue-400'}" size={24} />
									</div>
									<div class="flex-1">
										<p class="text-xs text-slate-500 dark:text-slate-400 uppercase tracking-wider font-medium mb-1">
											Estado de Pago
										</p>
										<p class="text-base font-bold {getEstadoPagoClass(pedido.estado_pago)}">
											{getEstadoPagoLabel(pedido.estado_pago)}
										</p>
									</div>
								</div>

								<!-- Método de Envío -->
								<div class="flex items-center gap-4 p-4 rounded-xl 
									{isPedidoCancelado(pedido.estado)
										? 'bg-slate-50 dark:bg-slate-800/50 border border-slate-200 dark:border-slate-700'
										: 'bg-purple-50/50 dark:bg-purple-900/10 border border-purple-100 dark:border-purple-900/30'}">
									<div class="w-12 h-12 rounded-xl flex items-center justify-center flex-shrink-0
										{isPedidoCancelado(pedido.estado) ? 'bg-slate-200 dark:bg-slate-700' : 'bg-purple-500/20'}">
										<Truck class="{isPedidoCancelado(pedido.estado) ? 'text-slate-500' : 'text-purple-600 dark:text-purple-400'}" size={24} />
									</div>
									<div class="flex-1">
										<p class="text-xs text-slate-500 dark:text-slate-400 uppercase tracking-wider font-medium mb-1">
											Método de Envío
										</p>
										<p class="text-base font-bold text-text-light dark:text-text-dark">
											{pedido.metodo_envio || 'Estándar'}
										</p>
									</div>
								</div>

								<!-- Total -->
								<div class="flex items-center gap-4 p-4 rounded-xl 
									{isPedidoCancelado(pedido.estado)
										? 'bg-slate-50 dark:bg-slate-800/50 border border-slate-200 dark:border-slate-700'
										: 'bg-green-50/50 dark:bg-green-900/10 border border-green-100 dark:border-green-900/30'}">
									<div class="w-12 h-12 rounded-xl flex items-center justify-center flex-shrink-0
										{isPedidoCancelado(pedido.estado) ? 'bg-slate-200 dark:bg-slate-700' : 'bg-green-500/20'}">
										<CreditCard class="{isPedidoCancelado(pedido.estado) ? 'text-slate-500' : 'text-green-600 dark:text-green-400'}" size={24} />
									</div>
									<div class="flex-1">
										<p class="text-xs text-slate-500 dark:text-slate-400 uppercase tracking-wider font-medium mb-1">
											{isPedidoCancelado(pedido.estado) ? 'Total del Pedido' : 'Total Pagado'}
										</p>
										<p class="text-xl font-bold {isPedidoCancelado(pedido.estado) ? 'text-slate-500 line-through' : 'text-green-600 dark:text-green-400'}">
											{formatPrice(pedido.total)}
										</p>
									</div>
								</div>
							</div>

							<!-- Dirección de Envío -->
							<div class="mt-5 pt-5 border-t border-border-light dark:border-border-dark">
								<div class="flex items-start gap-3">
									<div class="w-10 h-10 rounded-lg bg-slate-100 dark:bg-slate-700 flex items-center justify-center flex-shrink-0">
										<MapPin class="text-slate-500" size={20} />
									</div>
									<div>
										<p class="text-xs text-slate-500 dark:text-slate-400 uppercase tracking-wider font-medium mb-1">
											Dirección de Envío
										</p>
										<p class="text-sm text-text-light dark:text-text-dark">
											{formatDireccion(pedido)}
										</p>
									</div>
								</div>
							</div>

							<!-- Mensaje de cancelación -->
							{#if isPedidoCancelado(pedido.estado)}
								<div class="mt-4 p-4 rounded-xl bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800">
									<p class="text-sm text-red-700 dark:text-red-300">
										<strong>Pedido cancelado.</strong> Si el pago fue procesado, el reembolso se realizará en un plazo de 5-10 días hábiles.
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
