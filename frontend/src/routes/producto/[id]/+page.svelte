<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { catalogoService, type ProductoDetalle, type Valoracion, type CrearValoracionRequest } from '$lib/services/api';
	import { cartService } from '$lib/services/cart';
	import { isAuthenticated } from '$lib/stores/auth';
	import { ShoppingCart, Heart, Truck, Shield, Star, Minus, Plus, ThumbsUp, BadgeCheck } from 'lucide-svelte';

	let producto: ProductoDetalle | null = null;
	let valoraciones: Valoracion[] = [];
	let loading = true;
	let loadingValoraciones = false;
	let error: string | null = null;
	let cantidad = 1;
	let imagenSeleccionada = 0;
	let tabActiva: 'especificaciones' | 'descripcion' | 'resenas' = 'especificaciones';
	let isAddingToCart = false;
	let creandoResena = false;
	let calificacionNueva = 5;
	let tituloNuevo = '';
	let comentarioNuevo = '';
	let errorResena: string | null = null;
	
	$: productId = parseInt($page.params.id || '0');
	$: imagenes = producto?.imagenes || [producto?.imagen_principal || 'https://placehold.co/800x600/1e293b/94a3b8?text=Producto'];
	$: descuentoPorcentaje = producto && producto.precio_base > producto.precio_venta
		? Math.round(((producto.precio_base - producto.precio_venta) / producto.precio_base) * 100)
		: 0;
	
	async function cargarProducto() {
		loading = true;
		error = null;
		try {
			producto = await catalogoService.getProductoDetalle(productId);
			// Cargar valoraciones cuando cambiamos a la pestaña de reseñas
		} catch (err) {
			error = 'Error al cargar el producto';
			console.error(err);
		} finally {
			loading = false;
		}
	}
	
	async function cargarValoraciones() {
		if (!producto || valoraciones.length > 0) return;
		
		loadingValoraciones = true;
		try {
			// Necesitamos el id_producto, no el id_producto_detalle
			// El backend debe enviarlo o buscar por el producto_detalle
			valoraciones = await catalogoService.getValoraciones(productId);
		} catch (err) {
			console.error('Error al cargar valoraciones:', err);
		} finally {
			loadingValoraciones = false;
		}
	}
	
	// Calcular distribución de estrellas
	$: distribucionEstrellas = valoraciones.length > 0 ? {
		5: valoraciones.filter(v => v.calificacion === 5).length,
		4: valoraciones.filter(v => v.calificacion === 4).length,
		3: valoraciones.filter(v => v.calificacion === 3).length,
		2: valoraciones.filter(v => v.calificacion === 2).length,
		1: valoraciones.filter(v => v.calificacion === 1).length,
	} : null;
	
	// Cargar valoraciones cuando se cambia a la pestaña de reseñas
	$: if (tabActiva === 'resenas' && producto) {
		cargarValoraciones();
	}
	
	function incrementarCantidad() {
		if (producto && cantidad < producto.stock_disponible) {
			cantidad++;
		}
	}
	
	function decrementarCantidad() {
		if (cantidad > 1) {
			cantidad--;
		}
	}
	
	async function agregarAlCarrito() {
		// Verificar si está autenticado
		if (!$isAuthenticated) {
			await goto('/login?redirect=/producto/' + productId);
			return;
		}

		if (!producto || isAddingToCart || producto.stock_disponible === 0) return;

		isAddingToCart = true;
		try {
			await cartService.addItem({
				id_producto_detalle: producto.id_producto_detalle,
				cantidad: cantidad
			});
			// Redirigir al carrito
			await goto('/carrito');
		} catch (error: any) {
			alert(error.message || 'Error al agregar al carrito');
			isAddingToCart = false;
		}
	}
	
	onMount(() => {
		cargarProducto();
	});

	async function enviarResena() {
		if (!$isAuthenticated) {
			await goto(`/login?redirect=/producto/${productId}`);
			return;
		}

		if (!producto) return;

		if (calificacionNueva < 1 || calificacionNueva > 5) {
			errorResena = 'La calificación debe estar entre 1 y 5 estrellas.';
			return;
		}

		if (!comentarioNuevo.trim()) {
			errorResena = 'Por favor escribe un comentario sobre el producto.';
			return;
		}

		creandoResena = true;
		errorResena = null;

		const payload: CrearValoracionRequest = {
			calificacion: calificacionNueva,
			titulo: tituloNuevo.trim() || undefined,
			comentario: comentarioNuevo.trim() || undefined
		};

		try {
			const nueva = await catalogoService.crearValoracion(productId, payload);
			valoraciones = [nueva, ...valoraciones];
			// Actualizar contadores básicos del producto en el frontend
			producto = {
				...producto,
				valoracion_promedio:
					((producto.valoracion_promedio || 0) * producto.total_valoraciones + nueva.calificacion) /
					(producto.total_valoraciones + 1),
				total_valoraciones: producto.total_valoraciones + 1
			};
			// Reset de formulario
			calificacionNueva = 5;
			tituloNuevo = '';
			comentarioNuevo = '';
		} catch (err: any) {
			console.error('Error al crear reseña:', err);
			errorResena = err?.message || 'No se pudo enviar tu reseña. Intenta nuevamente.';
		} finally {
			creandoResena = false;
		}
	}
</script>

<svelte:head>
	<title>{producto?.nombre || 'Producto'} - KronosTech</title>
	<meta name="description" content={producto?.descripcion || 'Detalle del producto'} />
</svelte:head>

<div class="w-full max-w-7xl mx-auto px-4 sm:px-6 lg:px-10 py-5">
	<!-- Breadcrumbs -->
	<nav class="flex flex-wrap gap-2 mb-6 text-sm">
		<a href="/" class="text-slate-600 dark:text-slate-400 hover:text-primary">Inicio</a>
		<span class="text-slate-600 dark:text-slate-400">/</span>
		<a href="/catalogo" class="text-slate-600 dark:text-slate-400 hover:text-primary">Catálogo</a>
		{#if producto}
			<span class="text-slate-600 dark:text-slate-400">/</span>
			<span class="text-slate-600 dark:text-slate-400">{producto.categoria_nombre}</span>
			<span class="text-slate-600 dark:text-slate-400">/</span>
			<span class="text-text-light dark:text-text-dark font-medium">{producto.nombre}</span>
		{/if}
	</nav>

	{#if loading}
		<div class="grid grid-cols-1 lg:grid-cols-2 gap-8 lg:gap-12 mt-8">
			<div class="space-y-4">
				<div class="w-full aspect-[4/3] max-h-[500px] bg-slate-200 dark:bg-surface-dark rounded-xl animate-pulse"></div>
				<div class="grid grid-cols-4 gap-3">
					{#each Array(4) as _}
						<div class="aspect-square bg-slate-200 dark:bg-surface-dark rounded-lg animate-pulse"></div>
					{/each}
				</div>
			</div>
			<div class="space-y-6">
				<div class="h-8 w-3/4 bg-slate-200 dark:bg-surface-dark rounded animate-pulse"></div>
				<div class="h-6 w-1/2 bg-slate-200 dark:bg-surface-dark rounded animate-pulse"></div>
				<div class="h-20 bg-slate-200 dark:bg-surface-dark rounded-xl animate-pulse"></div>
			</div>
		</div>
	{:else if error || !producto}
		<div class="bg-red-100 dark:bg-red-900/20 text-red-700 dark:text-red-400 p-6 rounded-lg text-center">
			<p class="text-lg font-semibold mb-2">{error || 'Producto no encontrado'}</p>
			<a href="/catalogo" class="text-primary font-semibold hover:underline">Volver al catálogo</a>
		</div>
	{:else}
		<!-- Main Product Section -->
		<main class="grid grid-cols-1 lg:grid-cols-2 gap-8 lg:gap-12 mt-4">
			<!-- Galería de Imágenes -->
			<div class="flex flex-col gap-4">
				<div class="w-full rounded-xl shadow-sm aspect-[4/3] max-h-[500px] flex items-center justify-center p-6 overflow-hidden">
					<img 
						src={imagenes[imagenSeleccionada]} 
						alt={producto.nombre}
						class="w-full h-full object-contain"
					/>
				</div>
				<div class="grid grid-cols-4 gap-3">
					{#each imagenes.slice(0, 4) as imagen, index}
						<button
							on:click={() => imagenSeleccionada = index}
							class="w-full aspect-square bg-white dark:bg-surface-dark rounded-lg overflow-hidden border-2 transition-all p-2 {imagenSeleccionada === index ? 'border-primary ring-2 ring-primary/20' : 'border-border-light dark:border-border-dark opacity-70 hover:opacity-100 hover:border-primary/50'}"
						>
							<img 
								src={imagen} 
								alt={`${producto.nombre} vista ${index + 1}`} 
								class="w-full h-full object-contain"
							/>
						</button>
					{/each}
				</div>
			</div>

			<!-- Información del Producto -->
			<div class="flex flex-col gap-6">
				<div class="flex flex-col gap-2">
					<p class="text-sm font-medium text-primary">Por {producto.marca}</p>
					<h1 class="text-3xl md:text-4xl font-black leading-tight tracking-tight">
						{producto.nombre}
					</h1>
					<div class="flex items-center gap-2 mt-2">
						{#if producto.valoracion_promedio}
							<div class="flex gap-0.5 text-primary">
								{#each Array(5) as _, i}
									<Star 
										size={20} 
										fill={i < Math.floor(producto.valoracion_promedio) ? 'currentColor' : 'none'}
									/>
								{/each}
							</div>
							<p class="text-sm text-slate-600 dark:text-slate-400 font-medium">
								{producto.valoracion_promedio.toFixed(1)} ({producto.total_valoraciones} valoraciones)
							</p>
						{/if}
					</div>
				</div>

				<!-- Precio y Stock -->
				<div class="flex flex-col gap-4 p-4 rounded-xl border border-border-light dark:border-border-dark">
					<div class="flex items-baseline gap-3">
						<p class="text-4xl font-bold">S/. {producto.precio_venta.toFixed(2)}</p>
						{#if producto.precio_base > producto.precio_venta}
							<p class="text-lg line-through text-slate-500">S/. {producto.precio_base.toFixed(2)}</p>
							{#if descuentoPorcentaje > 0}
								<span class="text-sm font-semibold text-green-600 dark:text-green-400 bg-green-100 dark:bg-green-900/50 px-2 py-1 rounded">
									-{descuentoPorcentaje}%
								</span>
							{/if}
						{/if}
					</div>
					{#if producto.stock_disponible > 0}
						<span class="inline-block text-sm font-semibold text-green-600 dark:text-green-400 bg-green-100 dark:bg-green-900/50 px-3 py-1 rounded-full w-fit">
							En Stock ({producto.stock_disponible} disponibles)
						</span>
					{:else}
						<span class="inline-block text-sm font-semibold text-red-600 dark:text-red-400 bg-red-100 dark:bg-red-900/50 px-3 py-1 rounded-full w-fit">
							Agotado
						</span>
					{/if}
				</div>

				<!-- Cantidad y Acciones -->
				<div class="flex flex-col sm:flex-row gap-3">
					<div class="flex items-center border border-border-light dark:border-border-dark rounded-lg">
						<button 
							on:click={decrementarCantidad}
							disabled={cantidad <= 1}
							class="px-3 py-2 text-slate-600 dark:text-slate-400 hover:text-primary disabled:opacity-50"
						>
							<Minus size={18} />
						</button>
						<input 
							type="number" 
							bind:value={cantidad}
							min="1"
							max={producto.stock_disponible}
							class="w-12 text-center bg-transparent border-0 focus:ring-0"
						/>
						<button 
							on:click={incrementarCantidad}
							disabled={cantidad >= producto.stock_disponible}
							class="px-3 py-2 text-slate-600 dark:text-slate-400 hover:text-primary disabled:opacity-50"
						>
							<Plus size={18} />
						</button>
					</div>
					<button
						on:click={agregarAlCarrito}
						disabled={producto.stock_disponible === 0 || isAddingToCart}
						class="flex-1 flex items-center justify-center gap-2 px-6 py-3 bg-primary text-white text-base font-semibold rounded-lg shadow-sm hover:bg-primary/90 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
					>
						<ShoppingCart size={20} />
						{#if isAddingToCart}
							Agregando...
						{:else if producto.stock_disponible === 0}
							Agotado
						{:else}
							Añadir al carrito
						{/if}
					</button>
					<button class="p-3 border border-border-light dark:border-border-dark text-slate-600 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-surface-dark rounded-lg transition-colors">
						<Heart size={20} />
					</button>
				</div>

				<!-- Beneficios -->
				<div class="border-t border-border-light dark:border-border-dark pt-6 flex flex-col gap-4">
					<div class="flex items-center gap-3">
						<div class="flex h-10 w-10 items-center justify-center rounded-full bg-primary/10 text-primary">
							<Truck size={20} />
						</div>
						<div>
							<p class="font-semibold">Envío Gratis</p>
							<p class="text-sm text-slate-600 dark:text-slate-400">Entrega estimada en 3-5 días hábiles</p>
						</div>
					</div>
					<div class="flex items-center gap-3">
						<div class="flex h-10 w-10 items-center justify-center rounded-full bg-primary/10 text-primary">
							<Shield size={20} />
						</div>
						<div>
							<p class="font-semibold">{producto.garantia_meses} Meses de Garantía</p>
							<p class="text-sm text-slate-600 dark:text-slate-400">Garantía del fabricante incluida</p>
						</div>
					</div>
				</div>
			</div>
		</main>

		<!-- Sección de Tabs -->
		<section class="mt-12 lg:mt-16">
			<div class="border-b border-border-light dark:border-border-dark">
				<nav class="-mb-px flex space-x-6">
					<button 
						on:click={() => tabActiva = 'especificaciones'}
						class="whitespace-nowrap py-4 px-1 border-b-2 font-medium text-base transition-colors {tabActiva === 'especificaciones' ? 'text-primary border-primary' : 'text-slate-600 dark:text-slate-400 hover:text-primary border-transparent'}"
					>
						Especificaciones
					</button>
					<button 
						on:click={() => tabActiva = 'descripcion'}
						class="whitespace-nowrap py-4 px-1 border-b-2 font-medium text-base transition-colors {tabActiva === 'descripcion' ? 'text-primary border-primary' : 'text-slate-600 dark:text-slate-400 hover:text-primary border-transparent'}"
					>
						Descripción
					</button>
					<button 
						on:click={() => tabActiva = 'resenas'}
						class="whitespace-nowrap py-4 px-1 border-b-2 font-medium text-base transition-colors {tabActiva === 'resenas' ? 'text-primary border-primary' : 'text-slate-600 dark:text-slate-400 hover:text-primary border-transparent'}"
					>
						Reseñas ({producto.total_valoraciones})
					</button>
				</nav>
			</div>

			<div class="py-8">
				{#if tabActiva === 'especificaciones'}
					<!-- Especificaciones -->
					<div class="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-4 text-base">
						{#if producto.especificaciones_base}
							{#each Object.entries(producto.especificaciones_base) as [key, value]}
								<div class="flex justify-between py-3 border-b border-border-light dark:border-border-dark">
									<span class="text-slate-600 dark:text-slate-400 capitalize">{key.replace(/_/g, ' ')}</span>
									<span class="font-medium">{value}</span>
								</div>
							{/each}
						{:else}
							<div class="col-span-2">
								<p class="text-slate-600 dark:text-slate-400">No hay especificaciones disponibles para este producto.</p>
							</div>
						{/if}
						<div class="flex justify-between py-3 border-b border-border-light dark:border-border-dark">
							<span class="text-slate-600 dark:text-slate-400">SKU</span>
							<span class="font-medium">{producto.sku}</span>
						</div>
						<div class="flex justify-between py-3 border-b border-border-light dark:border-border-dark">
							<span class="text-slate-600 dark:text-slate-400">Marca</span>
							<span class="font-medium">{producto.marca}</span>
						</div>
						{#if producto.peso}
							<div class="flex justify-between py-3 border-b border-border-light dark:border-border-dark">
								<span class="text-slate-600 dark:text-slate-400">Peso</span>
								<span class="font-medium">{producto.peso} kg</span>
							</div>
						{/if}
						{#if producto.modelo}
							<div class="flex justify-between py-3 border-b border-border-light dark:border-border-dark">
								<span class="text-slate-600 dark:text-slate-400">Modelo</span>
								<span class="font-medium">{producto.modelo}</span>
							</div>
						{/if}
					</div>
				{:else if tabActiva === 'descripcion'}
					<!-- Descripción del Producto -->
					<div class="prose dark:prose-invert max-w-none">
						<div class="p-6 rounded-lg mb-4 border border-border-light dark:border-border-dark">
							<h3 class="text-lg font-bold mb-3 text-primary">Información del Producto</h3>
							<p class="text-slate-700 dark:text-slate-300 leading-relaxed mb-4">
								<strong>{producto.marca} {producto.producto_nombre || producto.nombre}</strong> es un producto de alta calidad 
								diseñado para ofrecer el mejor rendimiento y durabilidad.
							</p>
							
							<div class="grid grid-cols-1 md:grid-cols-2 gap-4 text-sm">
								<div>
									<span class="font-semibold text-slate-600 dark:text-slate-400">Marca:</span>
									<span class="ml-2">{producto.marca}</span>
								</div>
								{#if producto.modelo}
									<div>
										<span class="font-semibold text-slate-600 dark:text-slate-400">Modelo:</span>
										<span class="ml-2">{producto.modelo}</span>
									</div>
								{/if}
								<div>
									<span class="font-semibold text-slate-600 dark:text-slate-400">SKU:</span>
									<span class="ml-2">{producto.sku}</span>
								</div>
								<div>
									<span class="font-semibold text-slate-600 dark:text-slate-400">Garantía:</span>
									<span class="ml-2">{producto.garantia_meses} meses</span>
								</div>
							</div>
						</div>
					</div>
				{:else if tabActiva === 'resenas'}
					<!-- Reseñas -->
					<div class="space-y-8">
						<!-- Formulario para nueva reseña -->
						{#if $isAuthenticated}
							<div class="border border-border-light dark:border-border-dark rounded-lg p-6 bg-slate-50 dark:bg-surface-dark/40">
								<h3 class="text-lg font-semibold mb-3">Escribe una reseña</h3>
								{#if errorResena}
									<p class="mb-3 text-sm text-red-600 dark:text-red-400">{errorResena}</p>
								{/if}
								<form class="space-y-4" on:submit|preventDefault={enviarResena}>
									<div class="space-y-1">
										<label class="text-sm font-medium text-slate-700 dark:text-slate-300">Calificación</label>
										<div class="flex items-center gap-2 text-primary">
											{#each [1, 2, 3, 4, 5] as estrella}
												<button
													type="button"
													class="p-0.5"
													on:click={() => (calificacionNueva = estrella)}
												>
													<Star
														size={22}
														fill={estrella <= calificacionNueva ? 'currentColor' : 'none'}
													/>
												</button>
											{/each}
											<span class="ml-2 text-sm text-slate-600 dark:text-slate-400">{calificacionNueva} / 5</span>
										</div>
									</div>
									<div class="space-y-1">
										<label class="text-sm font-medium text-slate-700 dark:text-slate-300">Título (opcional)</label>
										<input
											type="text"
											bind:value={tituloNuevo}
											class="w-full border border-border-light dark:border-border-dark rounded-lg px-3 py-2 text-sm bg-white dark:bg-surface-dark"
											maxlength={120}
											placeholder="Lo que más te gustó del producto"
										/>
									</div>
									<div class="space-y-1">
										<label class="text-sm font-medium text-slate-700 dark:text-slate-300">Tu reseña</label>
										<textarea
											bind:value={comentarioNuevo}
											class="w-full border border-border-light dark:border-border-dark rounded-lg px-3 py-2 text-sm bg-white dark:bg-surface-dark min-h-[100px]"
											maxlength={800}
											placeholder="Comparte tu experiencia con este producto"
										></textarea>
									</div>
									<div class="flex justify-end">
										<button
											 type="submit"
											 class="px-4 py-2 rounded-lg bg-primary text-white text-sm font-semibold hover:bg-primary/90 disabled:opacity-60"
											 disabled={creandoResena}
										>
											{#if creandoResena}
												Enviando reseña...
											{:else}
												Enviar reseña
											{/if}
										</button>
									</div>
								</form>
							</div>
						{:else}
							<div class="border border-border-light dark:border-border-dark rounded-lg p-6 bg-slate-50 dark:bg-surface-dark/40 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-3">
								<div>
									<p class="text-sm font-semibold text-slate-700 dark:text-slate-200">¿Has comprado este producto?</p>
									<p class="text-sm text-slate-600 dark:text-slate-400">Inicia sesión para dejar tu reseña.</p>
								</div>
								<a
									href={`/login?redirect=/producto/${productId}`}
									class="px-4 py-2 rounded-lg bg-primary text-white text-sm font-semibold hover:bg-primary/90"
								>
									Iniciar sesión
								</a>
							</div>
						{/if}

						<!-- Resumen de valoraciones -->
						<div class="flex flex-wrap gap-x-12 gap-y-8">
							{#if producto.valoracion_promedio && producto.total_valoraciones > 0}
								<div class="flex flex-col gap-2">
									<p class="text-5xl font-black">{producto.valoracion_promedio.toFixed(1)}</p>
									<div class="flex gap-0.5 text-primary">
										{#each Array(5) as _, i}
											<Star 
												size={20} 
												fill={i < Math.floor(producto.valoracion_promedio) ? 'currentColor' : 'none'}
											/>
										{/each}
									</div>
									<p class="text-slate-600 dark:text-slate-400">Basado en {producto.total_valoraciones} reseñas</p>
								</div>
								
								{#if distribucionEstrellas}
									<div class="grid min-w-[200px] max-w-[400px] flex-1 grid-cols-[20px_1fr_60px] items-center gap-x-3 gap-y-3">
										{#each [5, 4, 3, 2, 1] as estrellas}
											{@const count = distribucionEstrellas[estrellas]}
											{@const porcentaje = producto.total_valoraciones > 0 ? (count / producto.total_valoraciones) * 100 : 0}
											<p class="text-sm font-medium">{estrellas}</p>
											<div class="flex h-2 overflow-hidden rounded-full bg-slate-200 dark:bg-slate-700">
												<div class="rounded-full bg-primary" style="width: {porcentaje}%;"></div>
											</div>
											<p class="text-sm text-slate-600 dark:text-slate-400 text-right">{porcentaje.toFixed(0)}%</p>
										{/each}
									</div>
								{/if}
							{:else}
								<div class="text-center py-8 w-full">
									<Star size={48} class="mx-auto text-slate-300 dark:text-slate-600 mb-4" />
									<p class="text-lg font-semibold text-slate-600 dark:text-slate-400 mb-2">
										Este producto aún no tiene valoraciones
									</p>
									<p class="text-sm text-slate-500 dark:text-slate-500">
										Sé el primero en dejar una reseña
									</p>
								</div>
							{/if}
						</div>
						
						<!-- Lista de valoraciones -->
						{#if loadingValoraciones}
							<div class="space-y-6">
								{#each Array(3) as _}
									<div class="border border-border-light dark:border-border-dark rounded-lg p-6 animate-pulse">
										<div class="h-4 bg-slate-200 dark:bg-slate-700 rounded w-1/4 mb-4"></div>
										<div class="h-3 bg-slate-200 dark:bg-slate-700 rounded w-full mb-2"></div>
										<div class="h-3 bg-slate-200 dark:bg-slate-700 rounded w-3/4"></div>
									</div>
								{/each}
							</div>
						{:else if valoraciones.length > 0}
							<div class="space-y-6">
								{#each valoraciones as valoracion}
									<div class="border border-border-light dark:border-border-dark rounded-lg p-6 hover:shadow-md transition-shadow">
										<!-- Header de la valoración -->
										<div class="flex items-start justify-between mb-4">
											<div class="flex-1">
												<div class="flex items-center gap-3 mb-2">
													<!-- Avatar -->
													<div class="flex h-10 w-10 items-center justify-center rounded-full bg-primary/10 text-primary font-bold">
														{valoracion.usuario_nombre?.[0] || 'U'}
													</div>
													<div>
														<div class="flex items-center gap-2">
															<p class="font-semibold">
																{valoracion.usuario_nombre || 'Usuario'} {valoracion.usuario_apellido?.[0] || ''}.
															</p>
															{#if valoracion.compra_verificada}
																<span class="inline-flex items-center gap-1 text-xs bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-400 px-2 py-0.5 rounded-full">
																	<BadgeCheck size={12} />
																	Compra verificada
																</span>
															{/if}
														</div>
														<p class="text-xs text-slate-500 dark:text-slate-400">
															{new Date(valoracion.fecha_creacion).toLocaleDateString('es-PE', { 
																year: 'numeric', 
																month: 'long', 
																day: 'numeric' 
															})}
														</p>
													</div>
												</div>
												
												<!-- Estrellas -->
												<div class="flex gap-0.5 text-primary mb-2">
													{#each Array(5) as _, i}
														<Star 
															size={16} 
															fill={i < valoracion.calificacion ? 'currentColor' : 'none'}
														/>
													{/each}
												</div>
											</div>
										</div>
										
										<!-- Título de la valoración -->
										{#if valoracion.titulo}
											<h4 class="font-semibold text-lg mb-2">{valoracion.titulo}</h4>
										{/if}
										
										<!-- Comentario -->
										{#if valoracion.comentario}
											<p class="text-slate-700 dark:text-slate-300 leading-relaxed mb-4">
												{valoracion.comentario}
											</p>
										{/if}
										
										<!-- Imágenes de la valoración -->
										{#if valoracion.imagenes && valoracion.imagenes.length > 0}
											<div class="flex gap-2 mb-4">
												{#each valoracion.imagenes as imagen}
													<img 
														src={imagen} 
														alt="Imagen de valoración" 
														class="w-20 h-20 object-cover rounded-lg border border-border-light dark:border-border-dark"
													/>
												{/each}
											</div>
										{/if}
										
										<!-- Footer: ¿Fue útil? -->
										<div class="flex items-center gap-4 pt-4 border-t border-border-light dark:border-border-dark">
											<p class="text-sm text-slate-600 dark:text-slate-400">¿Te resultó útil?</p>
											<button class="inline-flex items-center gap-1 text-sm text-slate-600 dark:text-slate-400 hover:text-primary transition-colors">
												<ThumbsUp size={16} />
												Sí ({valoracion.votos_util})
											</button>
											<button class="inline-flex items-center gap-1 text-sm text-slate-600 dark:text-slate-400 hover:text-primary transition-colors">
												No ({valoracion.votos_no_util})
											</button>
										</div>
									</div>
								{/each}
							</div>
						{:else if !loadingValoraciones && producto.total_valoraciones === 0}
							<div class="text-center py-8 rounded-lg border border-border-light dark:border-border-dark">
								<p class="text-slate-600 dark:text-slate-400">
									Aún no hay reseñas para este producto.
								</p>
							</div>
						{/if}
					</div>
				{/if}
			</div>
		</section>
	{/if}
</div>
