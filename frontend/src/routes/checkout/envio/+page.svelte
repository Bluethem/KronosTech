<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { cart, cartItems, cartSubtotal } from '$lib/stores/cart';
	import { direccionSeleccionada } from '$lib/stores/direccion';
	import { checkoutService } from '$lib/services/checkout';
	import { cartService } from '$lib/services/cart';
	import { siteConfig } from '$lib/stores/config';
	import { Truck, Package, Store, ChevronLeft, Check, Loader2 } from 'lucide-svelte';

	type ShippingMethod = {
		id: string;
		nombre: string;
		descripcion: string;
		costo: number;
		dias: string;
		etiqueta?: string;
		icon: any;
	};

	// Los métodos de envío se generan dinámicamente desde la configuración
	$: shippingMethods = [
		{
			id: 'estandar',
			nombre: 'Envío Estándar',
			descripcion: `Entrega en ${$siteConfig.estimatedDeliveryDays}-${$siteConfig.estimatedDeliveryDays + 2} días hábiles.`,
			costo: $siteConfig.defaultShippingCost,
			dias: `${$siteConfig.estimatedDeliveryDays}-${$siteConfig.estimatedDeliveryDays + 2} días hábiles`,
			etiqueta: 'Recomendado',
			icon: Package
		},
		{
			id: 'express',
			nombre: 'Envío Express',
			descripcion: 'Entrega en 1-2 días hábiles.',
			costo: $siteConfig.expressShippingCost,
			dias: '1-2 días hábiles',
			icon: Truck
		},
		{
			id: 'tienda',
			nombre: 'Recojo en tienda',
			descripcion: 'Retira tu pedido en nuestras instalaciones.',
			costo: 0.0,
			dias: 'Disponible hoy',
			etiqueta: 'Gratis',
			icon: Store
		}
	] as ShippingMethod[];

	let selectedMethodId: string = 'estandar';
	let deliveryNotes = '';
	let globalError = '';
	let loading = true;
	let calculatedTotal: any = null;

	$: currentCart = $cart;
	$: selectedMethod = shippingMethods.find((m) => m.id === selectedMethodId) ?? shippingMethods[0];

	// Usar los totales calculados del backend si están disponibles
	$: subtotal = calculatedTotal?.subtotal ?? currentCart?.subtotal ?? $cartSubtotal ?? 0;
	$: descuento = calculatedTotal?.descuento_total ?? currentCart?.descuento_total ?? 0;
	$: costoEnvio = selectedMethod?.costo ?? 0;
	$: total = subtotal - descuento + costoEnvio;

	onMount(async () => {
		// Verificar que hay dirección seleccionada
		if (!$direccionSeleccionada) {
			goto('/checkout/direccion');
			return;
		}

		// Cargar carrito primero
		try {
			await cartService.getCart();
		} catch (error) {
			console.error('Error al cargar carrito:', error);
		}

		// Verificar que hay items en el carrito después de cargarlo
		if ($cartItems.length === 0) {
			goto('/carrito');
			return;
		}

		await calculateTotal();
	});

	async function calculateTotal() {
		loading = true;
		globalError = '';

		try {
			// Recuperar cupón aplicado (si existe)
			const codigoCupon = localStorage.getItem('applied_coupon') || undefined;

			// Calcular total con el backend (incluye cupón si existe)
			const result = await checkoutService.calcularTotal($direccionSeleccionada?.id_direccion, codigoCupon);
			calculatedTotal = result;
		} catch (err: any) {
			globalError = err.message ?? 'Error al calcular total';
		} finally {
			loading = false;
		}
	}

	function goBackToAddress() {
		goto('/checkout/direccion');
	}

	async function goToPayment() {
		globalError = '';

		if (!selectedMethod) {
			globalError = 'Selecciona un método de envío para continuar.';
			return;
		}

		// Guardar método de envío seleccionado en sessionStorage para usarlo en pago
		sessionStorage.setItem('metodo_envio', JSON.stringify(selectedMethod));
		sessionStorage.setItem('notas_entrega', deliveryNotes);

		goto('/checkout/pago');
	}
</script>

<svelte:head>
	<title>Método de envío | KronosTech</title>
</svelte:head>

<div class="min-h-[calc(100vh-4rem)] bg-surface-light dark:bg-surface-dark">
	<div class="max-w-6xl mx-auto px-4 lg:px-6 py-8 space-y-6">
		<!-- STEPPER -->
		<div class="space-y-4">
			<div class="flex items-center gap-3 text-xs text-slate-600 dark:text-slate-400">
				<div class="flex items-center gap-2">
					<div class="w-6 h-6 rounded-full bg-primary text-white flex items-center justify-center text-[10px] font-semibold">
						<Check size={12} />
					</div>
					<span class="font-medium text-primary">Carrito</span>
				</div>
				<div class="h-px flex-1 bg-primary/50"></div>
				<div class="flex items-center gap-2">
					<div class="w-6 h-6 rounded-full bg-primary text-white flex items-center justify-center text-[10px] font-semibold">
						<Check size={12} />
					</div>
					<span class="font-medium text-primary">Dirección</span>
				</div>
				<div class="h-px flex-1 bg-primary/50"></div>
				<div class="flex items-center gap-2">
					<div class="w-6 h-6 rounded-full bg-primary text-white flex items-center justify-center text-[10px] font-semibold">3</div>
					<span class="font-semibold text-text-light dark:text-text-dark">Envío</span>
				</div>
				<div class="h-px flex-1 bg-border-light dark:bg-border-dark"></div>
				<div class="flex items-center gap-2">
					<div class="w-6 h-6 rounded-full border border-border-light dark:border-border-dark text-slate-500 dark:text-slate-400 flex items-center justify-center text-[10px] font-semibold">4</div>
					<span>Pago</span>
				</div>
			</div>

			<div class="flex flex-col md:flex-row md:items-center md:justify-between gap-3">
				<div>
					<h1 class="text-2xl font-bold tracking-tight text-text-light dark:text-text-dark">Método de Envío</h1>
					<p class="text-sm text-slate-600 dark:text-slate-400 mt-1">
						Elige cómo quieres recibir tu pedido. El costo de envío se actualizará automáticamente.
					</p>
				</div>

				<button
					type="button"
					class="inline-flex items-center justify-center gap-2 px-4 py-2 text-sm font-medium rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800 hover:bg-slate-100 dark:hover:bg-slate-700 transition-colors text-text-light dark:text-text-dark"
					on:click={goBackToAddress}
				>
					<ChevronLeft size={16} />
					Volver a dirección
				</button>
			</div>
		</div>

		{#if globalError}
			<div class="rounded-xl border border-red-300 dark:border-red-800 bg-red-50 dark:bg-red-900/20 px-4 py-3 text-sm text-red-700 dark:text-red-400">
				{globalError}
			</div>
		{/if}

		{#if loading}
			<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800 p-8 text-center">
				<Loader2 size={32} class="animate-spin mx-auto text-primary mb-3" />
				<p class="text-slate-600 dark:text-slate-400">Calculando totales...</p>
			</div>
		{:else}
			<div class="grid grid-cols-1 lg:grid-cols-[minmax(0,2.1fr),minmax(0,1fr)] gap-6 items-start">
				<section class="space-y-4">
					<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800 p-5">
						<h2 class="text-sm font-semibold uppercase tracking-wide text-slate-600 dark:text-slate-400 mb-3 flex items-center gap-2">
							<Truck size={16} />
							Opciones de envío
						</h2>

						<div class="space-y-3">
							{#each shippingMethods as method (method.id)}
								<button
									type="button"
									class="w-full text-left rounded-xl border px-4 py-3 flex items-start gap-3 transition-all {selectedMethodId === method.id
										? 'border-primary bg-primary/5 dark:bg-primary/10 ring-1 ring-primary'
										: 'border-border-light dark:border-border-dark bg-slate-50 dark:bg-slate-900/50 hover:bg-slate-100 dark:hover:bg-slate-700'}"
									on:click={() => (selectedMethodId = method.id)}
								>
									<div class="mt-1">
										<div
											class="w-4 h-4 rounded-full border flex items-center justify-center {selectedMethodId === method.id
												? 'border-primary bg-primary'
												: 'border-slate-400 dark:border-slate-500 bg-white dark:bg-slate-800'}"
										>
											{#if selectedMethodId === method.id}
												<div class="w-1.5 h-1.5 rounded-full bg-white"></div>
											{/if}
										</div>
									</div>

									<div class="flex-1 min-w-0 space-y-1">
										<div class="flex items-center gap-2">
											<svelte:component this={method.icon} size={16} class="text-slate-500 dark:text-slate-400" />
											<p class="text-sm font-semibold truncate text-text-light dark:text-text-dark">
												{method.nombre}
											</p>
											{#if method.etiqueta}
												<span class="text-[10px] px-2 py-0.5 rounded-full bg-primary/10 text-primary border border-primary/20">
													{method.etiqueta}
												</span>
											{/if}
										</div>
										<p class="text-xs text-slate-600 dark:text-slate-400">
											{method.descripcion}
										</p>
										<p class="text-xs text-slate-500 dark:text-slate-500">
											Estimado: {method.dias}
										</p>
									</div>

									<div class="flex flex-col items-end gap-1 ml-3">
										<span class="text-sm font-semibold text-text-light dark:text-text-dark">
											{method.costo === 0 ? 'Gratis' : `S/. ${method.costo.toFixed(2)}`}
										</span>
									</div>
								</button>
							{/each}
						</div>
					</div>

					<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800 p-5 space-y-3">
						<div class="flex items-center justify-between">
							<h2 class="text-sm font-semibold uppercase tracking-wide text-slate-600 dark:text-slate-400">
								Observaciones para la entrega
							</h2>
							<span class="text-[11px] text-slate-500 dark:text-slate-500">Opcional</span>
						</div>
						<textarea
							rows="3"
							class="w-full rounded-lg border border-border-light dark:border-border-dark bg-slate-50 dark:bg-slate-900 px-3 py-2 text-sm text-text-light dark:text-text-dark focus:outline-none focus:ring-2 focus:ring-primary/50"
							bind:value={deliveryNotes}
							placeholder="Ej. Entregar en recepción, llamar antes de llegar, referencia adicional…"
						></textarea>
						<p class="text-[11px] text-slate-500 dark:text-slate-500">
							Esta información se enviará al repartidor junto con tu pedido.
						</p>
					</div>
				</section>

				<aside
					aria-label="Resumen del pedido"
					class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800 p-5 space-y-4 sticky top-24"
				>
					<h2 class="text-lg font-semibold text-text-light dark:text-text-dark">Resumen del pedido</h2>

					<div class="space-y-2 text-sm">
						<div class="flex justify-between">
							<span class="text-slate-600 dark:text-slate-400">
								Subtotal ({$cartItems.length} artículo{$cartItems.length === 1 ? '' : 's'})
							</span>
							<span class="font-medium text-text-light dark:text-text-dark">
								S/. {subtotal.toFixed(2)}
							</span>
						</div>

						{#if calculatedTotal?.cupon_aplicado}
							<div class="flex justify-between items-center text-emerald-600 dark:text-emerald-400">
								<span class="text-xs flex items-center gap-1">
									<svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21.44 11.05l-9.19 9.19a6 6 0 0 1-8.49-8.49l9.19-9.19a4 4 0 0 1 5.66 5.66l-9.2 9.19a2 2 0 0 1-2.83-2.83l8.49-8.48"></path></svg>
									Cupón: {calculatedTotal.cupon_aplicado}
								</span>
								<span class="font-medium">
									− S/. {calculatedTotal.descuento_cupon.toFixed(2)}
								</span>
							</div>
						{/if}

						<div class="flex justify-between">
							<span class="text-slate-600 dark:text-slate-400">Descuento total</span>
							<span class="font-medium text-text-light dark:text-text-dark">
								{#if descuento > 0}
									− S/. {descuento.toFixed(2)}
								{:else}
									S/. 0.00
								{/if}
							</span>
						</div>

						<div class="flex justify-between items-center">
							<span class="text-slate-600 dark:text-slate-400">Envío ({selectedMethod.nombre})</span>
							<span class="font-medium text-text-light dark:text-text-dark">
								{costoEnvio === 0 ? 'Gratis' : `S/. ${costoEnvio.toFixed(2)}`}
							</span>
						</div>

						<div class="border-t border-border-light dark:border-border-dark pt-3 mt-2 flex justify-between items-center">
							<span class="text-sm font-semibold text-text-light dark:text-text-dark">Total</span>
							<span class="text-xl font-bold text-text-light dark:text-text-dark">
								S/. {total.toFixed(2)}
							</span>
						</div>
					</div>

					<button
						type="button"
						class="w-full mt-2 px-4 py-3 rounded-xl text-sm font-semibold bg-primary text-white hover:bg-primary/90 transition-colors shadow-lg disabled:opacity-60 disabled:cursor-not-allowed"
						on:click={goToPayment}
						disabled={!$cartItems.length}
					>
						Continuar a pago
					</button>

					<p class="text-[11px] text-slate-500 dark:text-slate-400 mt-1">
						El método de envío seleccionado se usará para calcular la fecha estimada de entrega y el costo final del pedido.
					</p>
				</aside>
			</div>
		{/if}
	</div>
</div>
