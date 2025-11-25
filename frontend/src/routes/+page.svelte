<script lang="ts">
	import { onMount } from 'svelte';
	import { catalogoService, type Familia, type Producto } from '$lib/services/api';
	import ProductCard from '$lib/components/ProductCard.svelte';
	import { ChevronRight, TrendingUp, Sparkles, Percent, Heart } from 'lucide-svelte';

	let familias: Familia[] = [];
	let productosDestacados: Producto[] = [];
	let productosOfertas: Producto[] = [];
	let productosNovedades: Producto[] = [];
	let loading = true;
	let error: string | null = null;

	// Imágenes para categorías (mapeo con las familias de la BD)
	const categoriasImagenes: Record<string, string> = {
		'Componentes Internos': 'https://images.unsplash.com/photo-1591799264318-7e6ef8ddb7ea?w=400',
		'Almacenamiento': 'https://images.unsplash.com/photo-1597872200969-2b65d56bd16b?w=400',
		'Periféricos': 'https://images.unsplash.com/photo-1587829741301-dc798b83add3?w=400',
		'Monitores': 'https://images.unsplash.com/photo-1527443224154-c4a3942d3acf?w=400',
		'Componentes de Red': 'https://images.unsplash.com/photo-1606904825846-647eb07f5be2?w=400',
		'Accesorios': 'https://images.unsplash.com/photo-1625948515291-69613efd103f?w=400'
	};

	onMount(async () => {
		try {
			const [familiasData, destacadosData, ofertasData, novedadesData] = await Promise.all([
				catalogoService.getFamilias(),
				catalogoService.getProductos({ destacados: true, limit: 8 }),
				catalogoService.getProductos({ ofertas: true, limit: 4 }),
				catalogoService.getProductos({ nuevos: true, limit: 4 })
			]);

			familias = familiasData.slice(0, 6); // Máximo 6 categorías
			productosDestacados = destacadosData.data;
			productosOfertas = ofertasData.data;
			productosNovedades = novedadesData.data;
		} catch (err) {
			error = 'Error al cargar los datos. Por favor, intenta de nuevo más tarde.';
			console.error(err);
		} finally {
			loading = false;
		}
	});
</script>

<svelte:head>
	<title>KronosTech - Tu Tienda de Componentes de PC y Gaming</title>
	<meta name="description" content="La mejor tienda de componentes de PC, gaming y tecnología en Perú. Procesadores, tarjetas gráficas, memorias RAM y más." />
</svelte:head>

<div class="px-4 sm:px-6 lg:px-10 py-8 max-w-screen-xl mx-auto">
	<!-- Hero Carousel Section -->
	<section class="mb-16">
		<div class="relative w-full h-[400px] md:h-[500px] rounded-xl overflow-hidden">
			<div class="absolute inset-0 bg-cover bg-center" style="background-image: url('https://images.unsplash.com/photo-1591488320449-011701bb6704?w=1200');">
				<div class="absolute inset-0 bg-gradient-to-r from-black/80 via-black/50 to-transparent"></div>
			</div>
			<div class="relative z-10 flex flex-col items-start justify-center h-full p-8 md:p-16 text-white max-w-2xl">
				<h1 class="text-3xl md:text-5xl lg:text-6xl font-extrabold mb-4 leading-tight">
					Potencia tu Setup Gaming
				</h1>
				<p class="text-base md:text-xl mb-6 text-slate-200">
					Los mejores componentes para PC, periféricos gaming y tecnología de última generación. 
					Arma tu PC ideal con nosotros.
				</p>
				<a 
					href="/catalogo"
					class="flex min-w-[120px] cursor-pointer items-center justify-center gap-2 overflow-hidden rounded-lg h-12 px-6 bg-primary text-white text-base font-bold leading-normal tracking-[0.015em] hover:bg-primary/90 transition-colors shadow-lg hover:shadow-xl"
				>
					Explorar Catálogo
					<ChevronRight size={20} />
				</a>
			</div>
		</div>
	</section>

	<!-- Categorías -->
	<section class="mb-16">
		<h2 class="text-2xl md:text-3xl font-bold leading-tight tracking-[-0.015em] mb-6">
			Compra por Categoría
		</h2>
		
		{#if loading}
			<div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-4">
				{#each Array(6) as _}
					<div class="aspect-square rounded-xl bg-slate-200 dark:bg-surface-dark animate-pulse"></div>
				{/each}
			</div>
		{:else if error}
			<div class="bg-red-100 dark:bg-red-900/20 text-red-700 dark:text-red-400 p-4 rounded-lg">
				{error}
			</div>
		{:else}
			<div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-4">
				{#each familias as familia}
					<a 
						href="/catalogo?familia={familia.id_familia}"
						class="group relative bg-cover bg-center flex flex-col gap-3 rounded-xl justify-end p-4 aspect-square overflow-hidden cursor-pointer"
						style="background-image: url('{categoriasImagenes[familia.nombre] || 'https://placehold.co/400/1e293b/94a3b8'}');"
					>
						<div class="absolute inset-0 bg-gradient-to-t from-black/70 via-black/30 to-transparent group-hover:from-black/85 transition-all"></div>
						<p class="text-white text-base font-bold leading-tight relative z-10 drop-shadow-lg">
							{familia.nombre}
						</p>
						{#if familia.total_productos}
							<p class="text-white/80 text-xs relative z-10">
								{familia.total_productos} productos
							</p>
						{/if}
					</a>
				{/each}
			</div>
		{/if}
	</section>

	<!-- Novedades -->
	{#if !loading && productosNovedades.length > 0}
		<section class="mb-16">
			<div class="flex items-center justify-between mb-6">
				<div class="flex items-center gap-3">
					<Sparkles size={28} class="text-purple-600 dark:text-purple-400" />
					<h2 class="text-2xl md:text-3xl font-bold leading-tight tracking-[-0.015em]">
						Novedades
					</h2>
				</div>
				<a 
					href="/novedades"
					class="text-primary font-semibold hover:underline flex items-center gap-1"
				>
					Ver todas
					<ChevronRight size={18} />
				</a>
			</div>
			<p class="text-slate-600 dark:text-slate-400 mb-6">
				Descubre los últimos lanzamientos en tecnología
			</p>
			<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
				{#each productosNovedades as producto}
					<div class="relative">
						<!-- Badge Nuevo -->
						<div class="absolute top-3 left-3 z-20 bg-gradient-to-r from-purple-600 to-pink-600 text-white text-xs font-bold px-3 py-1 rounded-full shadow-md flex items-center gap-1">
							<Sparkles size={14} />
							Nuevo
						</div>
						<button class="absolute top-3 right-3 z-20 p-2 rounded-full bg-white/90 dark:bg-surface-dark/90 backdrop-blur-sm text-slate-600 dark:text-slate-400 hover:text-red-500 dark:hover:text-red-500 transition-colors shadow-md">
							<Heart size={18} />
						</button>
						<ProductCard {producto} />
					</div>
				{/each}
			</div>
		</section>
	{/if}

	<!-- Ofertas Destacadas -->
	{#if !loading && productosOfertas.length > 0}
		<section class="mb-16">
			<div class="flex items-center justify-between mb-6">
				<div class="flex items-center gap-3">
					<Percent size={28} class="text-red-600 dark:text-red-400" />
					<h2 class="text-2xl md:text-3xl font-bold leading-tight tracking-[-0.015em]">
						Ofertas Imperdibles
					</h2>
				</div>
				<a 
					href="/ofertas"
					class="text-primary font-semibold hover:underline flex items-center gap-1"
				>
					Ver todas
					<ChevronRight size={18} />
				</a>
			</div>
			<p class="text-slate-600 dark:text-slate-400 mb-6">
				Aprovecha estos descuentos por tiempo limitado
			</p>
			<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
				{#each productosOfertas as producto}
					<div class="relative">
						<!-- Badge Descuento -->
						{#if producto.precio_base > producto.precio_venta}
							{@const descuento = Math.round(((producto.precio_base - producto.precio_venta) / producto.precio_base) * 100)}
							<div class="absolute top-3 left-3 z-20">
								<span class="inline-flex items-center px-3 py-1 rounded-full text-sm font-bold bg-red-600 text-white shadow-lg">
									-{descuento}%
								</span>
							</div>
						{/if}
						<button class="absolute top-3 right-3 z-20 p-2 rounded-full bg-white/90 dark:bg-surface-dark/90 backdrop-blur-sm text-slate-600 dark:text-slate-400 hover:text-red-500 dark:hover:text-red-500 transition-colors shadow-md">
							<Heart size={18} />
						</button>
						<ProductCard {producto} />
					</div>
				{/each}
			</div>
		</section>
	{/if}

	<!-- Productos Destacados -->
	<section class="mb-16">
		<div class="flex items-center justify-between mb-6">
			<div class="flex items-center gap-3">
				<TrendingUp size={28} class="text-primary" />
				<h2 class="text-2xl md:text-3xl font-bold leading-tight tracking-[-0.015em]">
					Productos Destacados
				</h2>
			</div>
			<a 
				href="/destacados"
				class="text-primary font-semibold hover:underline flex items-center gap-1"
			>
				Ver todos
				<ChevronRight size={18} />
			</a>
		</div>
		<p class="text-slate-600 dark:text-slate-400 mb-6">
			Los productos más vendidos y mejor valorados por nuestros clientes
		</p>
		
		{#if loading}
			<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
				{#each Array(8) as _}
					<div class="h-96 rounded-xl bg-slate-200 dark:bg-surface-dark animate-pulse"></div>
				{/each}
			</div>
		{:else if productosDestacados.length > 0}
			<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
				{#each productosDestacados as producto}
					<div class="relative">
						<!-- Badge Destacado -->
						<div class="absolute top-3 left-3 z-20 bg-primary text-white text-xs font-bold px-3 py-1 rounded-full shadow-md flex items-center gap-1">
							<TrendingUp size={14} />
							Destacado
						</div>
						<button class="absolute top-3 right-3 z-20 p-2 rounded-full bg-white/90 dark:bg-surface-dark/90 backdrop-blur-sm text-slate-600 dark:text-slate-400 hover:text-red-500 dark:hover:text-red-500 transition-colors shadow-md">
							<Heart size={18} />
						</button>
						<ProductCard {producto} />
					</div>
				{/each}
			</div>
		{:else}
			<p class="text-center text-slate-500 dark:text-slate-400 py-8">
				No hay productos destacados disponibles en este momento.
			</p>
		{/if}
	</section>

	<!-- Banner de Garantía/Envíos -->
	<section class="mb-16">
		<div class="grid grid-cols-1 md:grid-cols-3 gap-6">
			<div class="p-6 rounded-xl text-center">
				<div class="text-4xl mb-3">🚚</div>
				<h3 class="font-bold text-lg mb-2">Envío Gratis</h3>
				<p class="text-sm text-slate-600 dark:text-slate-400">
					En compras mayores a S/. 200
				</p>
			</div>
			<div class="p-6 rounded-xl text-center">
				<div class="text-4xl mb-3">🛡️</div>
				<h3 class="font-bold text-lg mb-2">Garantía Oficial</h3>
				<p class="text-sm text-slate-600 dark:text-slate-400">
					Todos nuestros productos con garantía
				</p>
			</div>
			<div class="p-6 rounded-xl text-center">
				<div class="text-4xl mb-3">💳</div>
				<h3 class="font-bold text-lg mb-2">Pago Seguro</h3>
				<p class="text-sm text-slate-600 dark:text-slate-400">
					Múltiples métodos de pago disponibles
				</p>
			</div>
		</div>
	</section>
</div>
