<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { cartItems } from '$lib/stores/cart';
	import { direcciones, direccionSeleccionada, setDirecciones, seleccionarDireccion } from '$lib/stores/direccion';
	import { direccionService } from '$lib/services/direccion';
	import { cartService } from '$lib/services/cart';
	import type { Direccion, CrearDireccionRequest } from '$lib/services/direccion';
	import { MapPin, Plus, X, Check, ChevronLeft, Loader2 } from 'lucide-svelte';

	let loading = true;
	let globalError = '';
	let showNewAddressForm = false;
	let savingAddress = false;

	// Formulario de nueva dirección
	let newAddress: CrearDireccionRequest = {
		tipo: 'envio',
		direccion_linea1: '',
		ciudad: '',
		departamento: '',
		pais: 'Perú',
		nombre_completo: '',
		telefono_contacto: '',
		direccion_linea2: '',
		codigo_postal: '',
		referencia: '',
		es_predeterminada: false
	};

	onMount(async () => {
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

		await loadDirecciones();
	});

	async function loadDirecciones() {
		loading = true;
		globalError = '';

		try {
			const lista = await direccionService.getDirecciones();
			setDirecciones(lista);
		} catch (err: any) {
			globalError = err.message ?? 'Error al cargar direcciones';
		} finally {
			loading = false;
		}
	}

	async function handleCreateAddress() {
		if (!newAddress.direccion_linea1.trim() || !newAddress.ciudad.trim() || !newAddress.departamento.trim()) {
			globalError = 'Completa los campos obligatorios (dirección, ciudad y departamento).';
			return;
		}

		savingAddress = true;
		globalError = '';

		try {
			const created = await direccionService.crearDireccion(newAddress);

			// Actualizar la lista
			direcciones.update(list => [...list, created]);

			// Seleccionar la nueva dirección
			seleccionarDireccion(created);

			// Cerrar el formulario
			showNewAddressForm = false;

			// Reset form
			newAddress = {
				tipo: 'envio',
				direccion_linea1: '',
				ciudad: '',
				departamento: '',
				pais: 'Perú',
				nombre_completo: '',
				telefono_contacto: '',
				direccion_linea2: '',
				codigo_postal: '',
				referencia: '',
				es_predeterminada: false
			};
		} catch (err: any) {
			globalError = err.message ?? 'Error al crear dirección';
		} finally {
			savingAddress = false;
		}
	}

	function handleSelectAddress(dir: Direccion) {
		seleccionarDireccion(dir);
	}

	function continueToShipping() {
		if (!$direccionSeleccionada) {
			globalError = 'Selecciona una dirección de envío para continuar.';
			return;
		}

		goto('/checkout/envio');
	}

	function goBackToCart() {
		goto('/carrito');
	}
</script>

<svelte:head>
	<title>Dirección de envío | KronosTech</title>
</svelte:head>

<div class="min-h-[calc(100vh-4rem)] bg-surface-light dark:bg-surface-dark">
	<div class="max-w-6xl mx-auto px-4 lg:px-6 py-8 space-y-6">
		<!-- STEPPER -->
		<div class="space-y-4">
			<div class="flex items-center gap-3 text-xs text-slate-600 dark:text-slate-400">
				<!-- Carrito -->
				<div class="flex items-center gap-2">
					<div class="w-6 h-6 rounded-full bg-primary text-white flex items-center justify-center text-[10px] font-semibold">
						<Check size={12} />
					</div>
					<span class="font-medium text-primary">Carrito</span>
				</div>
				<div class="h-px flex-1 bg-primary/50"></div>
				<!-- Dirección (actual) -->
				<div class="flex items-center gap-2">
					<div class="w-6 h-6 rounded-full bg-primary text-white flex items-center justify-center text-[10px] font-semibold">2</div>
					<span class="font-semibold text-text-light dark:text-text-dark">Dirección</span>
				</div>
				<div class="h-px flex-1 bg-border-light dark:bg-border-dark"></div>
				<!-- Envío -->
				<div class="flex items-center gap-2">
					<div class="w-6 h-6 rounded-full border border-border-light dark:border-border-dark text-slate-500 dark:text-slate-400 flex items-center justify-center text-[10px] font-semibold">3</div>
					<span>Envío</span>
				</div>
				<div class="h-px flex-1 bg-border-light dark:bg-border-dark"></div>
				<!-- Pago -->
				<div class="flex items-center gap-2">
					<div class="w-6 h-6 rounded-full border border-border-light dark:border-border-dark text-slate-500 dark:text-slate-400 flex items-center justify-center text-[10px] font-semibold">4</div>
					<span>Pago</span>
				</div>
			</div>

			<!-- Título + volver -->
			<div class="flex flex-col md:flex-row md:items-center md:justify-between gap-3">
				<div>
					<h1 class="text-2xl font-bold tracking-tight text-text-light dark:text-text-dark">Dirección de Envío</h1>
					<p class="text-sm text-slate-600 dark:text-slate-400 mt-1">
						Selecciona dónde quieres recibir tu pedido o agrega una nueva dirección.
					</p>
				</div>

				<button
					type="button"
					class="inline-flex items-center justify-center gap-2 px-4 py-2 text-sm font-medium rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800 hover:bg-slate-100 dark:hover:bg-slate-700 transition-colors text-text-light dark:text-text-dark"
					on:click={goBackToCart}
				>
					<ChevronLeft size={16} />
					Volver al carrito
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
				<p class="text-slate-600 dark:text-slate-400">Cargando direcciones...</p>
			</div>
		{:else}
			<div class="grid grid-cols-1 lg:grid-cols-[minmax(0,2.1fr),minmax(0,1fr)] gap-6 items-start">
				<!-- Columna izquierda: direcciones -->
				<section class="space-y-4">
					<!-- Direcciones existentes -->
					{#if $direcciones.length > 0}
						<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800 p-5 space-y-3">
							<h2 class="text-sm font-semibold uppercase tracking-wide text-slate-600 dark:text-slate-400 flex items-center gap-2">
								<MapPin size={16} />
								Mis direcciones
							</h2>

							<div class="space-y-3">
								{#each $direcciones as dir (dir.id_direccion)}
									<button
										type="button"
										class="w-full text-left rounded-xl border px-4 py-3 transition-all {$direccionSeleccionada?.id_direccion === dir.id_direccion
											? 'border-primary bg-primary/5 dark:bg-primary/10 ring-1 ring-primary'
											: 'border-border-light dark:border-border-dark bg-slate-50 dark:bg-slate-900/50 hover:bg-slate-100 dark:hover:bg-slate-700'}"
										on:click={() => handleSelectAddress(dir)}
									>
										<div class="flex items-start gap-3">
											<!-- Radio -->
											<div class="mt-1">
												<div
													class="w-4 h-4 rounded-full border flex items-center justify-center {$direccionSeleccionada?.id_direccion === dir.id_direccion
														? 'border-primary bg-primary'
														: 'border-slate-400 dark:border-slate-500 bg-white dark:bg-slate-800'}"
												>
													{#if $direccionSeleccionada?.id_direccion === dir.id_direccion}
														<div class="w-1.5 h-1.5 rounded-full bg-white"></div>
													{/if}
												</div>
											</div>

											<!-- Info -->
											<div class="flex-1 min-w-0 space-y-1">
												<div class="flex items-center gap-2">
													{#if dir.nombre_completo}
														<p class="text-sm font-semibold truncate text-text-light dark:text-text-dark">
															{dir.nombre_completo}
														</p>
													{/if}
													{#if dir.es_predeterminada}
														<span class="text-[10px] px-2 py-0.5 rounded-full bg-primary/10 text-primary border border-primary/20">
															Predeterminada
														</span>
													{/if}
												</div>
												<p class="text-xs text-slate-700 dark:text-slate-300">
													{dir.direccion_linea1}
													{#if dir.direccion_linea2}, {dir.direccion_linea2}{/if}
												</p>
												<p class="text-xs text-slate-600 dark:text-slate-400">
													{dir.ciudad}, {dir.departamento}
													{#if dir.codigo_postal} - {dir.codigo_postal}{/if}
												</p>
												{#if dir.telefono_contacto}
													<p class="text-xs text-slate-500 dark:text-slate-500">
														Tel: {dir.telefono_contacto}
													</p>
												{/if}
											</div>
										</div>
									</button>
								{/each}
							</div>
						</div>
					{/if}

					<!-- Botón para nueva dirección -->
					{#if !showNewAddressForm}
						<button
							type="button"
							class="w-full rounded-xl border-2 border-dashed border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 hover:bg-slate-100 dark:hover:bg-slate-700 px-4 py-6 text-sm font-medium transition-colors text-slate-600 dark:text-slate-400 flex items-center justify-center gap-2"
							on:click={() => (showNewAddressForm = true)}
						>
							<Plus size={18} />
							Agregar nueva dirección
						</button>
					{:else}
						<!-- Formulario nueva dirección -->
						<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800 p-5 space-y-4">
							<div class="flex items-center justify-between">
								<h2 class="text-sm font-semibold uppercase tracking-wide text-slate-600 dark:text-slate-400">
									Nueva dirección
								</h2>
								<button
									type="button"
									class="p-1 rounded-lg hover:bg-slate-100 dark:hover:bg-slate-700 text-slate-500 transition-colors"
									on:click={() => (showNewAddressForm = false)}
								>
									<X size={18} />
								</button>
							</div>

							<form class="space-y-3" on:submit|preventDefault={handleCreateAddress}>
								<div class="grid grid-cols-1 md:grid-cols-2 gap-3">
									<div class="space-y-1">
										<label class="text-xs font-medium text-slate-600 dark:text-slate-400">Nombre completo</label>
										<input
											class="w-full rounded-lg border border-border-light dark:border-border-dark bg-slate-50 dark:bg-slate-900 px-3 py-2 text-sm text-text-light dark:text-text-dark focus:outline-none focus:ring-2 focus:ring-primary/50"
											bind:value={newAddress.nombre_completo}
											placeholder="Juan Pérez"
										/>
									</div>

									<div class="space-y-1">
										<label class="text-xs font-medium text-slate-600 dark:text-slate-400">Teléfono</label>
										<input
											class="w-full rounded-lg border border-border-light dark:border-border-dark bg-slate-50 dark:bg-slate-900 px-3 py-2 text-sm text-text-light dark:text-text-dark focus:outline-none focus:ring-2 focus:ring-primary/50"
											bind:value={newAddress.telefono_contacto}
											placeholder="999 999 999"
										/>
									</div>
								</div>

								<div class="space-y-1">
									<label class="text-xs font-medium text-slate-600 dark:text-slate-400">Dirección *</label>
									<input
										class="w-full rounded-lg border border-border-light dark:border-border-dark bg-slate-50 dark:bg-slate-900 px-3 py-2 text-sm text-text-light dark:text-text-dark focus:outline-none focus:ring-2 focus:ring-primary/50"
										bind:value={newAddress.direccion_linea1}
										placeholder="Av. Principal 123"
										required
									/>
								</div>

								<div class="space-y-1">
									<label class="text-xs font-medium text-slate-600 dark:text-slate-400">Referencia / Dpto / Piso (opcional)</label>
									<input
										class="w-full rounded-lg border border-border-light dark:border-border-dark bg-slate-50 dark:bg-slate-900 px-3 py-2 text-sm text-text-light dark:text-text-dark focus:outline-none focus:ring-2 focus:ring-primary/50"
										bind:value={newAddress.direccion_linea2}
										placeholder="Dpto 201, Edificio B"
									/>
								</div>

								<div class="grid grid-cols-1 md:grid-cols-3 gap-3">
									<div class="space-y-1">
										<label class="text-xs font-medium text-slate-600 dark:text-slate-400">Ciudad *</label>
										<input
											class="w-full rounded-lg border border-border-light dark:border-border-dark bg-slate-50 dark:bg-slate-900 px-3 py-2 text-sm text-text-light dark:text-text-dark focus:outline-none focus:ring-2 focus:ring-primary/50"
											bind:value={newAddress.ciudad}
											placeholder="Lima"
											required
										/>
									</div>

									<div class="space-y-1">
										<label class="text-xs font-medium text-slate-600 dark:text-slate-400">Departamento *</label>
										<input
											class="w-full rounded-lg border border-border-light dark:border-border-dark bg-slate-50 dark:bg-slate-900 px-3 py-2 text-sm text-text-light dark:text-text-dark focus:outline-none focus:ring-2 focus:ring-primary/50"
											bind:value={newAddress.departamento}
											placeholder="Lima"
											required
										/>
									</div>

									<div class="space-y-1">
										<label class="text-xs font-medium text-slate-600 dark:text-slate-400">Código postal</label>
										<input
											class="w-full rounded-lg border border-border-light dark:border-border-dark bg-slate-50 dark:bg-slate-900 px-3 py-2 text-sm text-text-light dark:text-text-dark focus:outline-none focus:ring-2 focus:ring-primary/50"
											bind:value={newAddress.codigo_postal}
											placeholder="15001"
										/>
									</div>
								</div>

								<div class="space-y-1">
									<label class="text-xs font-medium text-slate-600 dark:text-slate-400">Referencia adicional</label>
									<textarea
										rows="2"
										class="w-full rounded-lg border border-border-light dark:border-border-dark bg-slate-50 dark:bg-slate-900 px-3 py-2 text-sm text-text-light dark:text-text-dark focus:outline-none focus:ring-2 focus:ring-primary/50"
										bind:value={newAddress.referencia}
										placeholder="Ej. Frente al parque, al lado de la farmacia..."
									></textarea>
								</div>

								<div class="flex items-center gap-2 pt-2">
									<input
										id="es-predeterminada"
										type="checkbox"
										class="rounded border border-border-light dark:border-border-dark bg-slate-50 dark:bg-slate-900 text-primary focus:ring-primary/50"
										bind:checked={newAddress.es_predeterminada}
									/>
									<label for="es-predeterminada" class="text-xs text-slate-600 dark:text-slate-400">
										Establecer como dirección predeterminada
									</label>
								</div>

								<button
									type="submit"
									class="w-full px-4 py-2.5 rounded-xl text-sm font-semibold bg-primary text-white hover:bg-primary/90 transition-colors disabled:opacity-60 disabled:cursor-not-allowed flex items-center justify-center gap-2"
									disabled={savingAddress}
								>
									{#if savingAddress}
										<Loader2 size={16} class="animate-spin" />
										Guardando...
									{:else}
										Guardar dirección
									{/if}
								</button>
							</form>
						</div>
					{/if}
				</section>

				<!-- Columna derecha: resumen -->
				<aside class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800 p-5 space-y-4 sticky top-24">
					<h2 class="text-lg font-semibold text-text-light dark:text-text-dark">Resumen</h2>

					<div class="space-y-2 text-sm">
						<div class="flex justify-between">
							<span class="text-slate-600 dark:text-slate-400">
								Artículos en carrito
							</span>
							<span class="font-medium text-text-light dark:text-text-dark">
								{$cartItems.length}
							</span>
						</div>

						{#if $direccionSeleccionada}
							<div class="pt-3 border-t border-border-light dark:border-border-dark">
								<p class="text-xs text-slate-500 dark:text-slate-400 mb-2">Dirección seleccionada:</p>
								<div class="rounded-lg bg-slate-50 dark:bg-slate-900/50 border border-border-light dark:border-border-dark px-3 py-2 space-y-1">
									{#if $direccionSeleccionada.nombre_completo}
										<p class="text-xs font-semibold text-text-light dark:text-text-dark">
											{$direccionSeleccionada.nombre_completo}
										</p>
									{/if}
									<p class="text-xs text-slate-700 dark:text-slate-300">
										{$direccionSeleccionada.direccion_linea1}
									</p>
									<p class="text-xs text-slate-600 dark:text-slate-400">
										{$direccionSeleccionada.ciudad}, {$direccionSeleccionada.departamento}
									</p>
								</div>
							</div>
						{/if}
					</div>

					<button
						type="button"
						class="w-full mt-2 px-4 py-3 rounded-xl text-sm font-semibold bg-primary text-white hover:bg-primary/90 transition-colors shadow-lg disabled:opacity-60 disabled:cursor-not-allowed"
						on:click={continueToShipping}
						disabled={!$direccionSeleccionada}
					>
						Continuar al envío
					</button>

					<p class="text-[11px] text-slate-500 dark:text-slate-400 mt-1">
						Podrás revisar tu pedido completo antes de realizar el pago.
					</p>
				</aside>
			</div>
		{/if}
	</div>
</div>
