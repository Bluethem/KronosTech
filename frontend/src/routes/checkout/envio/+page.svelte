<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { cart, cartItems, cartSubtotal } from '$lib/stores/cart';
	import { direccionSeleccionada } from '$lib/stores/direccion';
	import { checkoutService } from '$lib/services/checkout';
	import { cartService } from '$lib/services/cart';

	type ShippingMethod = {
		id: string;
		nombre: string;
		descripcion: string;
		costo: number;
		dias: string;
		etiqueta?: string;
	};

	const shippingMethods: ShippingMethod[] = [
		{
			id: 'estandar',
			nombre: 'Envío Estándar',
			descripcion: 'Entrega en 3-5 días hábiles.',
			costo: 15.0,
			dias: '3-5 días hábiles',
			etiqueta: 'Recomendado'
		},
		{
			id: 'express',
			nombre: 'Envío Express',
			descripcion: 'Entrega en 1-2 días hábiles.',
			costo: 35.0,
			dias: '1-2 días hábiles'
		},
		{
			id: 'tienda',
			nombre: 'Recojo en tienda',
			descripcion: 'Retira tu pedido en nuestras instalaciones.',
			costo: 0.0,
			dias: 'Disponible hoy',
			etiqueta: 'Gratis'
		}
	];

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

<div class="min-h-[calc(100vh-4rem)] bg-gradient-to-b from-slate-950 via-slate-900 to-slate-950 text-slate-100">
	<div class="max-w-6xl mx-auto px-4 lg:px-0 py-8 space-y-6">
		<!-- STEPPER -->
		<div class="space-y-4">
			<div class="flex items-center gap-3 text-xs text-slate-400">
				<div class="flex items-center gap-2">
					<div class="w-6 h-6 rounded-full bg-blue-500 text-white flex items-center justify-center text-[10px] font-semibold">1</div>
					<span class="font-medium">Carrito</span>
				</div>
				<div class="h-px flex-1 bg-slate-700/70"></div>
				<div class="flex items-center gap-2">
					<div class="w-6 h-6 rounded-full bg-blue-500 text-white flex items-center justify-center text-[10px] font-semibold">2</div>
					<span class="font-medium">Dirección</span>
				</div>
				<div class="h-px flex-1 bg-slate-700/70"></div>
				<div class="flex items-center gap-2">
					<div class="w-6 h-6 rounded-full bg-slate-100 text-slate-900 flex items-center justify-center text-[10px] font-semibold">3</div>
					<span class="font-semibold text-slate-100">Envío</span>
				</div>
				<div class="h-px flex-1 bg-slate-700/50"></div>
				<div class="flex items-center gap-2">
					<div class="w-6 h-6 rounded-full border border-slate-600 text-slate-400 flex items-center justify-center text-[10px] font-semibold">4</div>
					<span>Pago</span>
				</div>
			</div>

			<div class="flex flex-col md:flex-row md:items-center md:justify-between gap-3">
				<div>
					<h1 class="text-3xl font-bold tracking-tight">Método de Envío</h1>
					<p class="text-sm text-slate-400 mt-1">
						Elige cómo quieres recibir tu pedido. El costo de envío se actualizará automáticamente.
					</p>
				</div>

				<button
					type="button"
					class="inline-flex items-center justify-center px-4 py-2 text-sm font-medium rounded-2xl border border-white/10 bg-white/5 hover:bg-white/10 transition-colors"
					on:click={goBackToAddress}
				>
					← Volver a dirección
				</button>
			</div>
		</div>

		{#if globalError}
			<div class="rounded-2xl border border-rose-500/40 bg-rose-500/10 px-4 py-3 text-sm text-rose-100">
				{globalError}
			</div>
		{/if}

		{#if loading}
			<div class="rounded-3xl border border-white/10 bg-white/5 backdrop-blur-xl p-8 text-center">
				<p class="text-slate-400">Calculando totales...</p>
			</div>
		{:else}
			<div class="grid grid-cols-1 lg:grid-cols-[minmax(0,2.1fr),minmax(0,1fr)] gap-6 items-start">
				<section class="space-y-5">
					<div class="rounded-3xl border border-white/10 bg-white/5 backdrop-blur-xl shadow-[0_18px_50px_rgba(0,0,0,0.45)] p-5">
						<h2 class="text-sm font-semibold uppercase tracking-wide text-slate-300 mb-3">
							Opciones de envío
						</h2>

						<div class="space-y-3">
							{#each shippingMethods as method}
								<button
									type="button"
									class={`w-full text-left rounded-2xl border px-4 py-3 flex items-start gap-3 transition-all ${
										selectedMethodId === method.id
											? 'border-blue-500 bg-blue-500/10 shadow-[0_0_0_1px_rgba(59,130,246,0.4)]'
											: 'border-white/10 bg-black/40 hover:bg-white/5'
									}`}
									on:click={() => (selectedMethodId = method.id)}
								>
									<div class="mt-1">
										<div
											class={`w-4 h-4 rounded-full border flex items-center justify-center ${
												selectedMethodId === method.id
													? 'border-blue-400 bg-blue-500'
													: 'border-slate-500 bg-black'
											}`}
										>
											{#if selectedMethodId === method.id}
												<div class="w-1.5 h-1.5 rounded-full bg-white"></div>
											{/if}
										</div>
									</div>

									<div class="flex-1 min-w-0 space-y-1">
										<div class="flex items-center gap-2">
											<p class="text-sm font-semibold truncate">
												{method.nombre}
											</p>
											{#if method.etiqueta}
												<span class="text-[10px] px-2 py-0.5 rounded-full bg-blue-500/20 text-blue-200 border border-blue-500/50">
													{method.etiqueta}
												</span>
											{/if}
										</div>
										<p class="text-xs text-slate-300">
											{method.descripcion}
										</p>
										<p class="text-xs text-slate-400">
											Estimado: {method.dias}
										</p>
									</div>

									<div class="flex flex-col items-end gap-1 ml-3">
										<span class="text-sm font-semibold">
											{method.costo === 0 ? 'Gratis' : `S/. ${method.costo.toFixed(2)}`}
										</span>
									</div>
								</button>
							{/each}
						</div>
					</div>

					<div class="rounded-3xl border border-white/10 bg-white/5 backdrop-blur-xl shadow-[0_18px_50px_rgba(0,0,0,0.45)] p-5 space-y-3">
						<div class="flex items-center justify-between">
							<h2 class="text-sm font-semibold uppercase tracking-wide text-slate-300">
								Observaciones para la entrega
							</h2>
							<span class="text-[11px] text-slate-500">Opcional</span>
						</div>
						<textarea
							rows="3"
							class="w-full rounded-2xl border border-white/15 bg-black/40 px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500/70"
							bind:value={deliveryNotes}
							placeholder="Ej. Entregar en recepción, llamar antes de llegar, referencia adicional…"
						></textarea>
						<p class="text-[11px] text-slate-500">
							Esta información se enviará al repartidor junto con tu pedido.
						</p>
					</div>
				</section>

				<aside
					aria-label="Resumen del pedido"
					class="rounded-3xl border border-white/10 bg-white/5 backdrop-blur-xl p-5 space-y-4 shadow-[0_18px_50px_rgba(0,0,0,0.45)] sticky top-24"
				>
					<h2 class="text-lg font-semibold">Resumen del pedido</h2>

					<div class="space-y-2 text-sm">
						<div class="flex justify-between">
							<span class="text-slate-400">
								Subtotal ({$cartItems.length} artículo{$cartItems.length === 1 ? '' : 's'})
							</span>
							<span class="font-medium">
								S/. {subtotal.toFixed(2)}
							</span>
						</div>

						{#if calculatedTotal?.cupon_aplicado}
							<div class="flex justify-between items-center text-emerald-400">
								<span class="text-xs flex items-center gap-1">
									<svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21.44 11.05l-9.19 9.19a6 6 0 0 1-8.49-8.49l9.19-9.19a4 4 0 0 1 5.66 5.66l-9.2 9.19a2 2 0 0 1-2.83-2.83l8.49-8.48"></path></svg>
									Cupón: {calculatedTotal.cupon_aplicado}
								</span>
								<span class="font-medium text-emerald-300">
									− S/. {calculatedTotal.descuento_cupon.toFixed(2)}
								</span>
							</div>
						{/if}

						<div class="flex justify-between">
							<span class="text-slate-400">Descuento total</span>
							<span class="font-medium">
								{#if descuento > 0}
									− S/. {descuento.toFixed(2)}
								{:else}
									S/. 0.00
								{/if}
							</span>
						</div>

						<div class="flex justify-between items-center">
							<span class="text-slate-400">Envío ({selectedMethod.nombre})</span>
							<span class="font-medium">
								{costoEnvio === 0 ? 'Gratis' : `S/. ${costoEnvio.toFixed(2)}`}
							</span>
						</div>

						<div class="border-t border-white/10 pt-3 mt-2 flex justify-between items-center">
							<span class="text-sm font-semibold">Total</span>
							<span class="text-xl font-bold">
								S/. {total.toFixed(2)}
							</span>
						</div>
					</div>

					<button
						type="button"
						class="w-full mt-2 px-4 py-3 rounded-2xl text-sm font-semibold bg-blue-500 text-white hover:bg-blue-600 transition-colors shadow-lg shadow-blue-500/30 disabled:opacity-60 disabled:cursor-not-allowed"
						on:click={goToPayment}
						disabled={!$cartItems.length}
					>
						Continuar a pago
					</button>

					<p class="text-[11px] text-slate-500 mt-1">
						El método de envío seleccionado se usará para calcular la fecha estimada de entrega y el costo final del pedido.
					</p>
				</aside>
			</div>
		{/if}
	</div>
</div>
