<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { catalogoService, type Producto, type Familia, type Categoria } from '$lib/services/api';
	import { Search, Heart } from 'lucide-svelte';
	import ProductCard from '$lib/components/ProductCard.svelte';
	
	let productos: Producto[] = [];
	let categorias: Categoria[] = [];
	let familias: Familia[] = [];
	let marcas: any[] = [];
	let loading = true;
	let error: string | null = null;
	
	let tabActiva: 'todos' | 'productos' | 'categorias' | 'marcas' = 'todos';
	let terminoBusqueda = '';
	let ordenamiento = 'relevancia';
	
	// Paginación
	let currentPage = 1;
	let totalPages = 1;
	let totalProductos = 0;
	let limit = 12;
	
	$: terminoBusqueda = $page.url.searchParams.get('q') || '';
	$: offset = (currentPage - 1) * limit;
	
	const busquedasPopulares = [
		'RTX 4090',
		'AMD Ryzen 9',
		'Memoria RAM DDR5',
		'SSD NVMe 2TB',
		'Intel Core i9',
		'Tarjeta Gráfica',
	];
	
	function highlightText(text: string, search: string): string {
		if (!search.trim()) return text;
		const regex = new RegExp(`(${search.trim()})`, 'gi');
		return text.replace(regex, '<mark class="bg-yellow-200 dark:bg-yellow-600/50">$1</mark>');
	}
	
	async function buscar() {
		if (!terminoBusqueda.trim()) return;
		
		loading = true;
		error = null;
		
		try {
			const orderByParam = ordenamiento !== 'relevancia' ? ordenamiento : undefined;
			
			const [productosData, categoriasData, familiasData, marcasData] = await Promise.all([
				catalogoService.getProductos({ 
					search: terminoBusqueda, 
					limit,
					offset,
					order_by: orderByParam
				}),
				catalogoService.getCategorias(),
				catalogoService.getFamilias(),
				catalogoService.getMarcas()
			]);
			
			productos = productosData.data;
			totalProductos = productosData.pagination.total;
			totalPages = productosData.pagination.total_pages;
			
			categorias = categoriasData.filter(c => 
				c.nombre.toLowerCase().includes(terminoBusqueda.toLowerCase())
			);
			familias = familiasData.filter(f => 
				f.nombre.toLowerCase().includes(terminoBusqueda.toLowerCase())
			);
			marcas = marcasData.filter(m => 
				m.nombre.toLowerCase().includes(terminoBusqueda.toLowerCase())
			);
		} catch (err) {
			error = 'Error al realizar la búsqueda';
			console.error(err);
		} finally {
			loading = false;
		}
	}
	
	function cambiarPagina(pagina: number) {
		currentPage = pagina;
		buscar();
		window.scrollTo({ top: 0, behavior: 'smooth' });
	}
	
	onMount(() => {
		if (terminoBusqueda) {
			buscar();
		} else {
			loading = false;
		}
	});
	
	$: if (terminoBusqueda) {
		currentPage = 1;
		buscar();
	}
	
	$: if (ordenamiento) {
		currentPage = 1;
		if (terminoBusqueda) buscar();
	}
	
	$: totalResultados = productos.length + categorias.length + familias.length + marcas.length;
</script>

<svelte:head>
	<title>Resultados de Búsqueda: {terminoBusqueda} - KronosTech</title>
	<meta name="description" content="Resultados de búsqueda para {terminoBusqueda}" />
</svelte:head>

<div class="w-full max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
	<div class="flex flex-col lg:flex-row gap-8">
		<!-- Contenido Principal -->
		<div class="flex-grow">
			<!-- Título y Contador -->
			<div class="mb-6">
				{#if terminoBusqueda}
					<h1 class="text-3xl md:text-4xl font-black tracking-tight mb-2">
						Resultados para: <span class="text-primary">"{terminoBusqueda}"</span>
					</h1>
					{#if !loading}
						<p class="text-slate-600 dark:text-slate-400 text-sm">
							{totalResultados} resultados encontrados
						</p>
					{/if}
				{:else}
					<h1 class="text-3xl md:text-4xl font-black tracking-tight">
						Buscar Productos
					</h1>
					<p class="text-slate-600 dark:text-slate-400 text-sm mt-2">
						Ingresa un término de búsqueda para encontrar productos
					</p>
				{/if}
			</div>

			{#if terminoBusqueda}
				<!-- Toolbar -->
				<div class="flex flex-wrap items-center justify-between gap-4 mb-6">
					<div class="flex items-center gap-4">
						<p class="text-sm text-slate-600 dark:text-slate-400">
							{totalProductos} productos
						</p>
					</div>
					<select 
						bind:value={ordenamiento}
						class="py-2 px-4 rounded-lg border border-border-light dark:border-border-dark text-sm font-medium focus:ring-2 focus:ring-primary/50"
					>
						<option value="relevancia">Más Relevantes</option>
						<option value="precio_asc">Precio: Menor a Mayor</option>
						<option value="precio_desc">Precio: Mayor a Menor</option>
						<option value="nombre_asc">Nombre: A-Z</option>
						<option value="valoracion">Mejor Valorados</option>
					</select>
				</div>
				
				<!-- Tabs de Filtrado -->
				<div class="border-b border-border-light dark:border-border-dark mb-6">
					<div class="flex gap-4 sm:gap-8 overflow-x-auto">
						<button
							on:click={() => tabActiva = 'todos'}
							class="flex flex-col items-center justify-center border-b-[3px] pb-3 px-2 whitespace-nowrap transition-colors {tabActiva === 'todos' 
								? 'border-primary text-text-light dark:text-text-dark' 
								: 'border-transparent text-slate-500 dark:text-slate-400 hover:border-primary/50 hover:text-text-light dark:hover:text-text-dark'}"
						>
							<p class="text-sm font-bold">Todos</p>
						</button>
						<button
							on:click={() => tabActiva = 'productos'}
							class="flex flex-col items-center justify-center border-b-[3px] pb-3 px-2 whitespace-nowrap transition-colors {tabActiva === 'productos' 
								? 'border-primary text-text-light dark:text-text-dark' 
								: 'border-transparent text-slate-500 dark:text-slate-400 hover:border-primary/50 hover:text-text-light dark:hover:text-text-dark'}"
						>
							<p class="text-sm font-bold">Productos ({productos.length})</p>
						</button>
						<button
							on:click={() => tabActiva = 'categorias'}
							class="flex flex-col items-center justify-center border-b-[3px] pb-3 px-2 whitespace-nowrap transition-colors {tabActiva === 'categorias' 
								? 'border-primary text-text-light dark:text-text-dark' 
								: 'border-transparent text-slate-500 dark:text-slate-400 hover:border-primary/50 hover:text-text-light dark:hover:text-text-dark'}"
						>
							<p class="text-sm font-bold">Categorías ({categorias.length + familias.length})</p>
						</button>
						<button
							on:click={() => tabActiva = 'marcas'}
							class="flex flex-col items-center justify-center border-b-[3px] pb-3 px-2 whitespace-nowrap transition-colors {tabActiva === 'marcas' 
								? 'border-primary text-text-light dark:text-text-dark' 
								: 'border-transparent text-slate-500 dark:text-slate-400 hover:border-primary/50 hover:text-text-light dark:hover:text-text-dark'}"
						>
							<p class="text-sm font-bold">Marcas ({marcas.length})</p>
						</button>
					</div>
				</div>

				<!-- Contenido según Tab Activa -->
				{#if loading}
					<div class="flex items-center justify-center py-12">
						<div class="animate-spin rounded-full h-12 w-12 border-b-2 border-primary"></div>
					</div>
				{:else if error}
					<div class="bg-red-100 dark:bg-red-900/20 text-red-700 dark:text-red-400 p-6 rounded-lg text-center">
						{error}
					</div>
				{:else}
					<!-- Tab Todos -->
					{#if tabActiva === 'todos' || tabActiva === 'productos'}
						{#if productos.length > 0}
							<section class="mb-12">
								<h2 class="text-2xl font-bold mb-4">Productos Encontrados</h2>
								<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
									{#each productos as producto}
										<div class="relative">
											<button class="absolute top-3 right-3 z-20 p-2 rounded-full bg-white/90 dark:bg-surface-dark/90 backdrop-blur-sm text-slate-600 dark:text-slate-400 hover:text-red-500 dark:hover:text-red-500 transition-colors shadow-md">
												<Heart size={18} />
											</button>
											<ProductCard {producto} />
										</div>
									{/each}
								</div>
								
								<!-- Paginación -->
								{#if totalPages > 1}
									<nav class="mt-8 flex items-center justify-center">
										<ul class="inline-flex items-center -space-x-px rounded-lg overflow-hidden border border-border-light dark:border-border-dark">
											<li>
												<button
													on:click={() => cambiarPagina(currentPage - 1)}
													disabled={currentPage === 1}
													class="py-2 px-3 leading-tight bg-surface-light dark:bg-surface-dark hover:bg-slate-100 dark:hover:bg-slate-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
												>
													Anterior
												</button>
											</li>
											
											{#each Array(Math.min(totalPages, 5)) as _, i}
												{@const pageNum = i + 1}
												<li>
													<button
														on:click={() => cambiarPagina(pageNum)}
														class="py-2 px-3 leading-tight transition-colors {currentPage === pageNum 
															? 'z-10 bg-primary text-white' 
															: 'bg-surface-light dark:bg-surface-dark hover:bg-slate-100 dark:hover:bg-slate-700'}"
													>
														{pageNum}
													</button>
												</li>
											{/each}
											
											{#if totalPages > 5}
												<li>
													<span class="py-2 px-3 leading-tight bg-surface-light dark:bg-surface-dark">...</span>
												</li>
												<li>
													<button
														on:click={() => cambiarPagina(totalPages)}
														class="py-2 px-3 leading-tight bg-surface-light dark:bg-surface-dark hover:bg-slate-100 dark:hover:bg-slate-700 transition-colors"
													>
														{totalPages}
													</button>
												</li>
											{/if}
											
											<li>
												<button
													on:click={() => cambiarPagina(currentPage + 1)}
													disabled={currentPage === totalPages}
													class="py-2 px-3 leading-tight bg-surface-light dark:bg-surface-dark hover:bg-slate-100 dark:hover:bg-slate-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
												>
													Siguiente
												</button>
											</li>
										</ul>
									</nav>
								{/if}
							</section>
						{:else if tabActiva === 'productos'}
							<div class="text-center py-12">
								<Search size={48} class="mx-auto text-slate-400 mb-4" />
								<p class="text-lg text-slate-600 dark:text-slate-400">No se encontraron productos</p>
							</div>
						{/if}
					{/if}

					<!-- Tab Categorías -->
					{#if tabActiva === 'todos' || tabActiva === 'categorias'}
						{#if (categorias.length > 0 || familias.length > 0)}
							<section class="mb-12">
								<h2 class="text-2xl font-bold mb-4">Categorías Relacionadas</h2>
								<div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-4">
									{#each familias as familia}
										<a 
											href="/catalogo?familia={familia.id_familia}"
											class="flex flex-col items-center justify-center gap-3 p-6 rounded-lg text-center hover:bg-primary/10 dark:hover:bg-primary/20 transition-colors border border-border-light dark:border-border-dark"
										>
											<div class="text-4xl">🏷️</div>
											<p class="font-semibold text-sm">{@html highlightText(familia.nombre, terminoBusqueda)}</p>
											{#if familia.total_productos}
												<span class="text-xs text-slate-500 dark:text-slate-400">{familia.total_productos} productos</span>
											{/if}
										</a>
									{/each}
									{#each categorias as categoria}
										<a 
											href="/catalogo?categoria={categoria.id_categoria}"
											class="flex flex-col items-center justify-center gap-3 p-6 bg-surface-light dark:bg-surface-dark rounded-lg text-center hover:bg-primary/10 dark:hover:bg-primary/20 transition-colors border border-border-light dark:border-border-dark"
										>
											<div class="text-4xl">📦</div>
											<p class="font-semibold text-sm">{categoria.nombre}</p>
											{#if categoria.total_productos}
												<span class="text-xs text-slate-500 dark:text-slate-400">{categoria.total_productos} productos</span>
											{/if}
										</a>
									{/each}
								</div>
							</section>
						{:else if tabActiva === 'categorias'}
							<div class="text-center py-12">
								<Search size={48} class="mx-auto text-slate-400 mb-4" />
								<p class="text-lg text-slate-600 dark:text-slate-400">No se encontraron categorías</p>
							</div>
						{/if}
					{/if}

					<!-- Tab Marcas -->
					{#if tabActiva === 'todos' || tabActiva === 'marcas'}
						{#if marcas.length > 0}
							<section class="mb-12">
								<h2 class="text-2xl font-bold mb-4">Marcas Relacionadas</h2>
								<div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-4">
									{#each marcas as marca}
										<a 
											href="/catalogo?marca={marca.id_marca}"
											class="flex flex-col items-center justify-center p-6 rounded-lg hover:bg-primary/10 dark:hover:bg-primary/20 transition-colors border border-border-light dark:border-border-dark"
										>
											<p class="font-bold text-lg">{@html highlightText(marca.nombre, terminoBusqueda)}</p>
											{#if marca.pais_origen}
												<span class="text-xs text-slate-500 dark:text-slate-400 mt-1">{marca.pais_origen}</span>
											{/if}
										</a>
									{/each}
								</div>
							</section>
						{:else if tabActiva === 'marcas'}
							<div class="text-center py-12">
								<Search size={48} class="mx-auto text-slate-400 mb-4" />
								<p class="text-lg text-slate-600 dark:text-slate-400">No se encontraron marcas</p>
							</div>
						{/if}
					{/if}

					<!-- Sin Resultados (solo en tab Todos) -->
					{#if totalResultados === 0 && tabActiva === 'todos'}
						<div class="text-center py-12">
							<Search size={64} class="mx-auto text-slate-400 mb-4" />
							<h3 class="text-xl font-bold mb-2">No se encontraron resultados</h3>
							<p class="text-slate-600 dark:text-slate-400 mb-6">
								Intenta con otros términos de búsqueda
							</p>
							<a 
								href="/catalogo" 
								class="inline-block bg-primary text-white font-bold py-3 px-6 rounded-lg hover:bg-primary/90 transition-colors"
							>
								Ver todo el catálogo
							</a>
						</div>
					{/if}
				{/if}
			{:else}
				<!-- Sin término de búsqueda -->
				<div class="text-center py-12">
					<Search size={64} class="mx-auto text-slate-400 mb-4" />
					<h3 class="text-xl font-bold mb-2">¿Qué estás buscando?</h3>
					<p class="text-slate-600 dark:text-slate-400">
						Usa el buscador del header para encontrar productos
					</p>
				</div>
			{/if}
		</div>

		<!-- Sidebar -->
		<aside class="w-full lg:w-72 flex-shrink-0">
			<div class="sticky top-24">
				<div class="rounded-lg p-6 border border-border-light dark:border-border-dark">
					<h3 class="text-lg font-bold mb-4">Búsquedas Populares</h3>
					<ul class="space-y-3">
						{#each busquedasPopulares as busqueda}
							<li>
								<a 
									href="/busqueda?q={encodeURIComponent(busqueda)}" 
									class="text-primary hover:underline text-sm"
								>
									{busqueda}
								</a>
							</li>
						{/each}
					</ul>
				</div>
			</div>
		</aside>
	</div>
</div>
