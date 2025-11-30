<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { checkoutService } from '$lib/services/checkout';
	import type { Venta } from '$lib/services/checkout';

	let loading = true;
	let error = '';
	let pedidos: Venta[] = [];
	let filtroEstado = 'todos';

	onMount(async () => {
		await loadOrders();
	});

	async function loadOrders() {
		loading = true;
		error = '';

		try {
			pedidos = await checkoutService.getPedidos();
		} catch (err: any) {
			error = err.message ?? 'Error al cargar pedidos';
		} finally {
			loading = false;
		}
	}

	function formatDate(dateStr: string): string {
		const date = new Date(dateStr);
		return date.toLocaleDateString('es-PE', {
			day: '2-digit',
			month: 'short',
			year: 'numeric'
		});
	}

	function getEstadoBadgeClass(estado: string): string {
		switch (estado) {
			case 'pendiente':
				return 'bg-yellow-500/20 text-yellow-200 border-yellow-500/50';
			case 'confirmado':
				return 'bg-blue-500/20 text-blue-200 border-blue-500/50';
			case 'procesando':
				return 'bg-purple-500/20 text-purple-200 border-purple-500/50';
			case 'enviado':
				return 'bg-indigo-500/20 text-indigo-200 border-indigo-500/50';
			case 'entregado':
				return 'bg-green-500/20 text-green-200 border-green-500/50';
			case 'cancelado':
			case 'devuelto':
				return 'bg-red-500/20 text-red-200 border-red-500/50';
			default:
				return 'bg-slate-500/20 text-slate-200 border-slate-500/50';
		}
	}

	function getEstadoLabel(estado: string): string {
		const labels: Record<string, string> = {
			pendiente: 'Pendiente',
			confirmado: 'Confirmado',
			procesando: 'Procesando',
			enviado: 'Enviado',
			entregado: 'Entregado',
			cancelado: 'Cancelado',
			devuelto: 'Devuelto'
		};
		return labels[estado] || estado;
	}

	function viewOrder(idVenta: number) {
		goto(`/pedido/${idVenta}`);
	}

	function goToCatalog() {
		goto('/catalogo');
	}

	// Filtrar pedidos según el filtro seleccionado
	$: pedidosFiltrados =
		filtroEstado === 'todos'
			? pedidos
			: pedidos.filter((p) => p.estado === filtroEstado);
</script>

<svelte:head>
	<title>Mis Pedidos | KronosTech</title>
</svelte:head>

<div class="min-h-[calc(100vh-4rem)] bg-gradient-to-b from-slate-950 via-slate-900 to-slate-950 text-slate-100">
	<div class="max-w-6xl mx-auto px-4 lg:px-0 py-8 space-y-6">
		<!-- Header -->
		<div class="space-y-4">
			<div class="flex flex-col md:flex-row md:items-center md:justify-between gap-3">
				<div>
					<h1 class="text-3xl font-bold tracking-tight">Mis Pedidos</h1>
					<p class="text-sm text-slate-400 mt-1">
						Historial completo de tus compras
					</p>
				</div>

				<button
					type="button"
					class="inline-flex items-center justify-center px-4 py-2 text-sm font-medium rounded-2xl border border-white/10 bg-white/5 hover:bg-white/10 transition-colors"
					on:click={goToCatalog}
				>
					Ir al catálogo
				</button>
			</div>

			<!-- Filter Tabs -->
			<div class="flex gap-2 overflow-x-auto pb-2">
				<button
					class={`px-4 py-2 rounded-2xl text-sm font-medium whitespace-nowrap transition-all ${
						filtroEstado === 'todos'
							? 'bg-blue-500 text-white'
							: 'border border-white/10 bg-white/5 hover:bg-white/10'
					}`}
					on:click={() => (filtroEstado = 'todos')}
				>
					Todos
				</button>
				<button
					class={`px-4 py-2 rounded-2xl text-sm font-medium whitespace-nowrap transition-all ${
						filtroEstado === 'pendiente'
							? 'bg-blue-500 text-white'
							: 'border border-white/10 bg-white/5 hover:bg-white/10'
					}`}
					on:click={() => (filtroEstado = 'pendiente')}
				>
					Pendientes
				</button>
				<button
					class={`px-4 py-2 rounded-2xl text-sm font-medium whitespace-nowrap transition-all ${
						filtroEstado === 'procesando'
							? 'bg-blue-500 text-white'
							: 'border border-white/10 bg-white/5 hover:bg-white/10'
					}`}
					on:click={() => (filtroEstado = 'procesando')}
				>
					Procesando
				</button>
				<button
					class={`px-4 py-2 rounded-2xl text-sm font-medium whitespace-nowrap transition-all ${
						filtroEstado === 'enviado'
							? 'bg-blue-500 text-white'
							: 'border border-white/10 bg-white/5 hover:bg-white/10'
					}`}
					on:click={() => (filtroEstado = 'enviado')}
				>
					Enviados
				</button>
				<button
					class={`px-4 py-2 rounded-2xl text-sm font-medium whitespace-nowrap transition-all ${
						filtroEstado === 'entregado'
							? 'bg-blue-500 text-white'
							: 'border border-white/10 bg-white/5 hover:bg-white/10'
					}`}
					on:click={() => (filtroEstado = 'entregado')}
				>
					Entregados
				</button>
			</div>
		</div>

		{#if loading}
			<div class="rounded-3xl border border-white/10 bg-white/5 backdrop-blur-xl p-8 text-center">
				<p class="text-slate-400">Cargando pedidos...</p>
			</div>
		{:else if error}
			<div class="rounded-3xl border border-rose-500/40 bg-rose-500/10 backdrop-blur-xl p-8 text-center space-y-4">
				<p class="text-rose-100 text-lg">{error}</p>
				<button
					type="button"
					class="px-6 py-3 rounded-2xl text-sm font-semibold bg-blue-500 text-white hover:bg-blue-600 transition-colors"
					on:click={loadOrders}
				>
					Reintentar
				</button>
			</div>
		{:else if pedidosFiltrados.length === 0}
			<div class="rounded-3xl border border-white/10 bg-white/5 backdrop-blur-xl p-8 text-center space-y-4">
				<svg class="w-16 h-16 mx-auto text-slate-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z" />
				</svg>
				<p class="text-slate-300 text-lg">
					{filtroEstado === 'todos' ? 'No tienes pedidos aún' : `No tienes pedidos ${filtroEstado}`}
				</p>
				<button
					type="button"
					class="px-6 py-3 rounded-2xl text-sm font-semibold bg-blue-500 text-white hover:bg-blue-600 transition-colors shadow-lg shadow-blue-500/30"
					on:click={goToCatalog}
				>
					Explorar catálogo
				</button>
			</div>
		{:else}
			<div class="space-y-4">
				{#each pedidosFiltrados as pedido}
					<div class="rounded-3xl border border-white/10 bg-white/5 backdrop-blur-xl shadow-[0_18px_50px_rgba(0,0,0,0.45)] hover:border-white/20 transition-all">
						<div class="p-6">
							<!-- Order Header -->
							<div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 mb-4">
								<div>
									<div class="flex items-center gap-3 mb-2">
										<p class="text-lg font-semibold">{pedido.numero_pedido}</p>
										<span class={`text-xs px-2.5 py-1 rounded-full border font-medium ${getEstadoBadgeClass(pedido.estado)}`}>
											{getEstadoLabel(pedido.estado)}
										</span>
									</div>
									<p class="text-xs text-slate-400">
										Pedido el {formatDate(pedido.fecha_pedido)}
									</p>
								</div>

								<div class="flex items-center gap-3">
									<div class="text-right">
										<p class="text-xs text-slate-400 mb-1">Total</p>
										<p class="text-xl font-bold">S/. {pedido.total.toFixed(2)}</p>
									</div>
									<button
										type="button"
										class="px-4 py-2 rounded-2xl text-sm font-medium border border-white/10 bg-white/5 hover:bg-white/10 transition-colors"
										on:click={() => viewOrder(pedido.id_venta)}
									>
										Ver detalles →
									</button>
								</div>
							</div>

							<!-- Order Items Preview -->
							{#if pedido.items && pedido.items.length > 0}
								<div class="border-t border-white/10 pt-4">
									<p class="text-xs text-slate-400 mb-3">
										{pedido.items.length} artículo{pedido.items.length === 1 ? '' : 's'}
									</p>
									<div class="flex gap-2 overflow-x-auto pb-2">
										{#each pedido.items.slice(0, 5) as item}
											<div class="flex-shrink-0 w-16 h-16 rounded-lg overflow-hidden bg-slate-800 border border-white/10">
												{#if item.imagen}
													<img
														src={item.imagen}
														alt={item.nombre_producto}
														class="w-full h-full object-cover"
													/>
												{:else}
													<div class="w-full h-full flex items-center justify-center">
														<svg class="w-8 h-8 text-slate-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
															<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
														</svg>
													</div>
												{/if}
											</div>
										{/each}
										{#if pedido.items.length > 5}
											<div class="flex-shrink-0 w-16 h-16 rounded-lg bg-slate-800/50 border border-white/10 flex items-center justify-center">
												<p class="text-xs text-slate-400">+{pedido.items.length - 5}</p>
											</div>
										{/if}
									</div>
								</div>
							{/if}

							<!-- Additional Info -->
							<div class="border-t border-white/10 pt-4 mt-4 grid grid-cols-1 sm:grid-cols-3 gap-3">
								{#if pedido.fecha_entrega_estimada}
									<div>
										<p class="text-xs text-slate-400 mb-1">Entrega estimada</p>
										<p class="text-sm font-medium">{formatDate(pedido.fecha_entrega_estimada)}</p>
									</div>
								{/if}
								{#if pedido.numero_tracking}
									<div>
										<p class="text-xs text-slate-400 mb-1">Tracking</p>
										<p class="text-sm font-mono font-medium">{pedido.numero_tracking}</p>
									</div>
								{/if}
								{#if pedido.ciudad}
									<div>
										<p class="text-xs text-slate-400 mb-1">Envío a</p>
										<p class="text-sm font-medium truncate">
											{pedido.ciudad}{pedido.departamento ? `, ${pedido.departamento}` : ''}
										</p>
									</div>
								{/if}
							</div>
						</div>
					</div>
				{/each}
			</div>
		{/if}
	</div>
</div>
