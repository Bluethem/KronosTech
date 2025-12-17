<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { checkoutService } from '$lib/services/checkout';
	import type { Venta } from '$lib/services/checkout';
	import { CheckCircle, Package, Truck, MapPin, FileText, ShoppingBag, Loader2 } from 'lucide-svelte';

	let loading = true;
	let error = '';
	let venta: Venta | null = null;

	$: idVenta = Number($page.params.id);

	onMount(async () => {
		await loadOrder();
	});

	async function loadOrder() {
		loading = true;
		error = '';

		try {
			venta = await checkoutService.getPedido(idVenta);
		} catch (err: any) {
			error = err.message ?? 'Error al cargar el pedido';
		} finally {
			loading = false;
		}
	}

	function formatDate(dateStr: string): string {
		const date = new Date(dateStr);
		return date.toLocaleDateString('es-PE', {
			timeZone: 'America/Lima',
			day: '2-digit',
			month: 'long',
			year: 'numeric'
		});
	}

	function getEstadoBadgeClass(estado: string): string {
		switch (estado) {
			case 'pendiente':
				return 'bg-amber-100 dark:bg-amber-900/30 text-amber-700 dark:text-amber-400 border-amber-200 dark:border-amber-800';
			case 'confirmado':
				return 'bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-400 border-blue-200 dark:border-blue-800';
			case 'procesando':
				return 'bg-purple-100 dark:bg-purple-900/30 text-purple-700 dark:text-purple-400 border-purple-200 dark:border-purple-800';
			case 'enviado':
				return 'bg-indigo-100 dark:bg-indigo-900/30 text-indigo-700 dark:text-indigo-400 border-indigo-200 dark:border-indigo-800';
			case 'entregado':
				return 'bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-400 border-green-200 dark:border-green-800';
			case 'cancelado':
			case 'devuelto':
				return 'bg-red-100 dark:bg-red-900/30 text-red-700 dark:text-red-400 border-red-200 dark:border-red-800';
			default:
				return 'bg-slate-100 dark:bg-slate-800 text-slate-700 dark:text-slate-400 border-slate-200 dark:border-slate-700';
		}
	}

	function getEstadoPagoClass(estadoPago: string): string {
		switch (estadoPago) {
			case 'completado':
				return 'bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-400 border-green-200 dark:border-green-800';
			case 'procesando':
			case 'pendiente':
				return 'bg-amber-100 dark:bg-amber-900/30 text-amber-700 dark:text-amber-400 border-amber-200 dark:border-amber-800';
			case 'fallido':
			case 'rechazado':
			case 'cancelado':
				return 'bg-red-100 dark:bg-red-900/30 text-red-700 dark:text-red-400 border-red-200 dark:border-red-800';
			case 'reembolsado':
			case 'parcialmente_reembolsado':
				return 'bg-orange-100 dark:bg-orange-900/30 text-orange-700 dark:text-orange-400 border-orange-200 dark:border-orange-800';
			default:
				return 'bg-slate-100 dark:bg-slate-800 text-slate-700 dark:text-slate-400 border-slate-200 dark:border-slate-700';
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

	function getEstadoPagoLabel(estadoPago: string): string {
		const labels: Record<string, string> = {
			pendiente: 'Pago Pendiente',
			procesando: 'Procesando Pago',
			completado: 'Pago Completado',
			fallido: 'Pago Fallido',
			rechazado: 'Pago Rechazado',
			cancelado: 'Pago Cancelado',
			reembolsado: 'Reembolsado',
			parcialmente_reembolsado: 'Parcialmente Reembolsado'
		};
		return labels[estadoPago] || estadoPago;
	}

	function continueShopping() {
		goto('/catalogo');
	}

	function viewOrderDetails() {
		goto(`/pedido/${idVenta}`);
	}
</script>

<svelte:head>
	<title>Pedido Confirmado | KronosTech</title>
</svelte:head>

<div class="min-h-[calc(100vh-4rem)] bg-surface-light dark:bg-surface-dark">
	<div class="max-w-4xl mx-auto px-4 lg:px-6 py-8 space-y-6">
		{#if loading}
			<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800 p-8 text-center">
				<Loader2 size={32} class="animate-spin mx-auto text-primary mb-3" />
				<p class="text-slate-600 dark:text-slate-400">Cargando pedido...</p>
			</div>
		{:else if error}
			<div class="rounded-xl border border-red-300 dark:border-red-800 bg-red-50 dark:bg-red-900/20 p-8 text-center space-y-4">
				<p class="text-red-700 dark:text-red-400 text-lg">{error}</p>
				<button
					type="button"
					class="px-6 py-3 rounded-xl text-sm font-semibold bg-primary text-white hover:bg-primary/90 transition-colors"
					on:click={continueShopping}
				>
					Volver al catálogo
				</button>
			</div>
		{:else if venta}
			<!-- Success Header -->
			<div class="rounded-xl border border-green-300 dark:border-green-800 bg-green-50 dark:bg-green-900/20 p-8 text-center space-y-3">
				<div class="flex justify-center mb-4">
					<div class="w-16 h-16 rounded-full bg-green-100 dark:bg-green-900/30 flex items-center justify-center">
						<CheckCircle size={40} class="text-green-600 dark:text-green-400" />
					</div>
				</div>
				<h1 class="text-2xl font-bold tracking-tight text-green-800 dark:text-green-300">¡Pedido Confirmado!</h1>
				<p class="text-slate-700 dark:text-slate-300">
					Tu pedido ha sido registrado exitosamente. Recibirás un correo con los detalles de tu compra.
				</p>
			</div>

			<!-- Order Number and Status -->
			<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800 p-6 space-y-4">
				<div class="flex flex-col md:flex-row md:items-center md:justify-between gap-4">
					<div>
						<p class="text-xs text-slate-500 dark:text-slate-400 uppercase tracking-wide mb-1">Número de pedido</p>
						<p class="text-2xl font-bold text-text-light dark:text-text-dark">{venta.numero_pedido}</p>
					</div>
					<div class="flex flex-wrap gap-2">
						<span class="text-xs px-3 py-1.5 rounded-full border font-medium {getEstadoBadgeClass(venta.estado)}">
							{getEstadoLabel(venta.estado)}
						</span>
						<span class="text-xs px-3 py-1.5 rounded-full border font-medium {getEstadoPagoClass(venta.estado_pago)}">
							{getEstadoPagoLabel(venta.estado_pago)}
						</span>
					</div>
				</div>

				<div class="grid grid-cols-1 md:grid-cols-2 gap-4 pt-4 border-t border-border-light dark:border-border-dark">
					<div>
						<p class="text-xs text-slate-500 dark:text-slate-400 mb-1">Fecha del pedido</p>
						<p class="text-sm font-medium text-text-light dark:text-text-dark">{formatDate(venta.fecha_pedido)}</p>
					</div>
					{#if venta.fecha_entrega_estimada}
						<div>
							<p class="text-xs text-slate-500 dark:text-slate-400 mb-1">Entrega estimada</p>
							<p class="text-sm font-medium text-text-light dark:text-text-dark">{formatDate(venta.fecha_entrega_estimada)}</p>
						</div>
					{/if}
					{#if venta.numero_tracking}
						<div>
							<p class="text-xs text-slate-500 dark:text-slate-400 mb-1">Número de seguimiento</p>
							<p class="text-sm font-mono font-medium text-text-light dark:text-text-dark">{venta.numero_tracking}</p>
						</div>
					{/if}
				</div>
			</div>

			<!-- Items Purchased -->
			<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800 p-6 space-y-4">
				<h2 class="text-lg font-semibold text-text-light dark:text-text-dark flex items-center gap-2">
					<Package size={20} />
					Artículos del pedido
				</h2>

				<div class="space-y-3">
					{#each venta.items as item (item.id_venta_detalle)}
						<div class="rounded-lg border border-border-light dark:border-border-dark bg-slate-50 dark:bg-slate-900/50 p-4 flex gap-4">
							{#if item.imagen}
								<img
									src={item.imagen}
									alt={item.nombre_producto}
									class="w-16 h-16 object-contain rounded-lg bg-white dark:bg-slate-800 border border-border-light dark:border-border-dark p-1"
								/>
							{:else}
								<div class="w-16 h-16 bg-slate-100 dark:bg-slate-800 rounded-lg flex items-center justify-center border border-border-light dark:border-border-dark">
									<ShoppingBag size={24} class="text-slate-400" />
								</div>
							{/if}

							<div class="flex-1 min-w-0">
								<p class="text-sm font-semibold truncate text-text-light dark:text-text-dark">{item.nombre_producto}</p>
								<p class="text-xs text-slate-500 dark:text-slate-500">SKU: {item.sku}</p>
								<div class="flex items-center gap-3 mt-2">
									<p class="text-xs text-slate-600 dark:text-slate-400">Cantidad: {item.cantidad}</p>
									{#if item.descuento_unitario > 0}
										<p class="text-xs text-green-600 dark:text-green-400">Descuento: S/. {item.descuento_unitario.toFixed(2)}</p>
									{/if}
								</div>
							</div>

							<div class="text-right">
								<p class="text-sm font-medium text-text-light dark:text-text-dark">S/. {item.precio_final.toFixed(2)} c/u</p>
								<p class="text-xs text-slate-500 dark:text-slate-500 mt-1">Subtotal:</p>
								<p class="text-sm font-semibold text-text-light dark:text-text-dark">S/. {item.subtotal.toFixed(2)}</p>
							</div>
						</div>
					{/each}
				</div>
			</div>

			<!-- Shipping Address -->
			{#if venta.direccion_envio}
				<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800 p-6 space-y-3">
					<h2 class="text-lg font-semibold text-text-light dark:text-text-dark flex items-center gap-2">
						<MapPin size={20} />
						Dirección de envío
					</h2>
					<div class="rounded-lg border border-border-light dark:border-border-dark bg-slate-50 dark:bg-slate-900/50 px-4 py-3 space-y-1">
						<p class="text-sm text-text-light dark:text-text-dark">{venta.direccion_envio}</p>
						<p class="text-sm text-slate-600 dark:text-slate-400">
							{venta.ciudad}{venta.departamento ? `, ${venta.departamento}` : ''}
							{venta.codigo_postal ? ` - ${venta.codigo_postal}` : ''}
						</p>
						{#if venta.telefono_contacto}
							<p class="text-xs text-slate-500 dark:text-slate-500 pt-2 border-t border-border-light dark:border-border-dark">
								Tel: {venta.telefono_contacto}
							</p>
						{/if}
						{#if venta.metodo_envio}
							<p class="text-xs text-primary pt-2 border-t border-border-light dark:border-border-dark flex items-center gap-1">
								<Truck size={14} />
								Método: {venta.metodo_envio}
							</p>
						{/if}
					</div>
				</div>
			{/if}

			<!-- Order Summary -->
			<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800 p-6 space-y-4">
				<h2 class="text-lg font-semibold text-text-light dark:text-text-dark">Resumen del pedido</h2>

				<div class="space-y-2 text-sm">
					<div class="flex justify-between">
						<span class="text-slate-600 dark:text-slate-400">Subtotal</span>
						<span class="font-medium text-text-light dark:text-text-dark">S/. {venta.subtotal.toFixed(2)}</span>
					</div>

					{#if venta.descuento_total > 0}
						<div class="flex justify-between">
							<span class="text-slate-600 dark:text-slate-400">Descuento</span>
							<span class="font-medium text-green-600 dark:text-green-400">− S/. {venta.descuento_total.toFixed(2)}</span>
						</div>
					{/if}

					<div class="flex justify-between">
						<span class="text-slate-600 dark:text-slate-400">Envío</span>
						<span class="font-medium text-text-light dark:text-text-dark">
							{venta.costo_envio === 0 ? 'Gratis' : `S/. ${venta.costo_envio.toFixed(2)}`}
						</span>
					</div>

					<div class="border-t border-border-light dark:border-border-dark pt-3 mt-2 flex justify-between items-center">
						<span class="text-base font-semibold text-text-light dark:text-text-dark">Total pagado</span>
						<span class="text-2xl font-bold text-primary">
							S/. {venta.total.toFixed(2)}
						</span>
					</div>
				</div>
			</div>

			<!-- Notes -->
			{#if venta.notas_cliente}
				<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800 p-6 space-y-3">
					<h2 class="text-sm font-semibold uppercase tracking-wide text-slate-600 dark:text-slate-400 flex items-center gap-2">
						<FileText size={16} />
						Observaciones
					</h2>
					<div class="rounded-lg border border-border-light dark:border-border-dark bg-slate-50 dark:bg-slate-900/50 px-4 py-3">
						<p class="text-sm text-slate-700 dark:text-slate-300">{venta.notas_cliente}</p>
					</div>
				</div>
			{/if}

			<!-- Action Buttons -->
			<div class="flex flex-col sm:flex-row gap-3">
				<button
					type="button"
					class="flex-1 px-6 py-3 rounded-xl text-sm font-semibold bg-primary text-white hover:bg-primary/90 transition-colors shadow-lg"
					on:click={continueShopping}
				>
					Continuar comprando
				</button>
				<button
					type="button"
					class="flex-1 px-6 py-3 rounded-xl text-sm font-semibold border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800 hover:bg-slate-100 dark:hover:bg-slate-700 transition-colors text-text-light dark:text-text-dark"
					on:click={viewOrderDetails}
				>
					Ver detalles del pedido
				</button>
			</div>

			<!-- Info Message -->
			<div class="rounded-xl border border-blue-200 dark:border-blue-800 bg-blue-50 dark:bg-blue-900/20 px-4 py-3 text-center">
				<p class="text-sm text-blue-700 dark:text-blue-300">
					Hemos enviado un correo de confirmación a tu dirección de email con todos los detalles de tu pedido.
				</p>
			</div>
		{/if}
	</div>
</div>
