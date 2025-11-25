<script lang="ts">
	import { onMount } from 'svelte';
	import { catalogoService, type Producto, type Familia, type Categoria } from '$lib/services/api';
	import { Heart, Grid3x3, List, TrendingUp } from 'lucide-svelte';
	import ProductCard from '$lib/components/ProductCard.svelte';
	
	let productos: Producto[] = [];
	let familias: Familia[] = [];
	let categorias: Categoria[] = [];
	let subcategorias: any[] = [];
	let marcas: any[] = [];
	
	let loading = true;
	let error: string | null = null;
	
	// Filtros
	let familiaSeleccionada: number | null = null;
	let categoriaSeleccionada: number | null = null;
	let subcategoriaSeleccionada: number | null = null;
	let marcaSeleccionada: number | null = null;
	let precioMin: number | null = null;
	let precioMax: number | null = null;
	
	// Paginación y vista
	let currentPage = 1;
	let totalPages = 1;
	let totalProductos = 0;
	let limit = 12;
	let ordenamiento = 'mas_vendidos';
	let vistaGrid = true;
	
	$: offset = (currentPage - 1) * limit;
	
	async function cargarDatos() {
		loading = true;
		error = null;
		
		try {
			const [familiasData, marcasData] = await Promise.all([
				catalogoService.getFamilias(),
				catalogoService.getMarcas()
			]);
			
			familias = familiasData;
			marcas = marcasData;
			
			await cargarProductos();
		} catch (err) {
			error = 'Error al cargar los datos';
			console.error(err);
		} finally {
			loading = false;
		}
	}
	
	async function cargarProductos() {
		try {
			const filtros: any = {
				destacados: true,
				limit,
				offset,
				order_by: ordenamiento
			};
			
			if (familiaSeleccionada) filtros.familia = familiaSeleccionada;
			if (categoriaSeleccionada) filtros.categoria = categoriaSeleccionada;
			if (subcategoriaSeleccionada) filtros.subcategoria = subcategoriaSeleccionada;
			if (marcaSeleccionada) filtros.marca = marcaSeleccionada;
			if (precioMin) filtros.precio_min = precioMin;
			if (precioMax) filtros.precio_max = precioMax;
			
			const response = await catalogoService.getProductos(filtros);
			productos = response.data;
			totalProductos = response.pagination.total;
			totalPages = response.pagination.total_pages;
		} catch (err) {
			console.error('Error al cargar productos:', err);
		}
	}
	
	async function cargarCategorias(idFamilia: number | null) {
		if (!idFamilia) {
			categorias = [];
			subcategorias = [];
			return;
		}
		
		try {
			categorias = await catalogoService.getCategorias(idFamilia);
		} catch (err) {
			console.error('Error al cargar categorías:', err);
		}
	}
	
	async function cargarSubcategorias(idCategoria: number | null) {
		if (!idCategoria) {
			subcategorias = [];
			return;
		}
		
		try {
			subcategorias = await catalogoService.getSubcategorias(idCategoria);
		} catch (err) {
			console.error('Error al cargar subcategorías:', err);
		}
	}
	
	function limpiarFiltros() {
		familiaSeleccionada = null;
		categoriaSeleccionada = null;
		subcategoriaSeleccionada = null;
		marcaSeleccionada = null;
		precioMin = null;
		precioMax = null;
		currentPage = 1;
		cargarProductos();
	}
	
	function cambiarPagina(pagina: number) {
		currentPage = pagina;
		cargarProductos();
		window.scrollTo({ top: 0, behavior: 'smooth' });
	}
	
	onMount(() => {
		cargarDatos();
	});
	
	$: if (familiaSeleccionada !== null) {
		categoriaSeleccionada = null;
		subcategoriaSeleccionada = null;
		cargarCategorias(familiaSeleccionada);
		currentPage = 1;
		cargarProductos();
	}
	
	$: if (categoriaSeleccionada !== null) {
		subcategoriaSeleccionada = null;
		cargarSubcategorias(categoriaSeleccionada);
		currentPage = 1;
		cargarProductos();
	}
	
	$: if (subcategoriaSeleccionada || marcaSeleccionada || precioMin || precioMax) {
		currentPage = 1;
		cargarProductos();
	}
	
	$: if (ordenamiento) {
		currentPage = 1;
		cargarProductos();
	}
	
	$: filtrosActivos = [
		familiaSeleccionada && familias.find(f => f.id_familia === familiaSeleccionada),
		categoriaSeleccionada && categorias.find(c => c.id_categoria === categoriaSeleccionada),
		subcategoriaSeleccionada && subcategorias.find(s => s.id_subcategoria === subcategoriaSeleccionada),
		marcaSeleccionada && marcas.find(m => m.id_marca === marcaSeleccionada),
		(precioMin || precioMax) && { tipo: 'precio', min: precioMin, max: precioMax }
	].filter(Boolean);
</script>

<svelte:head>
	<title>Productos Destacados - KronosTech</title>
	<meta name="description" content="Los productos más vendidos y populares de KronosTech" />
</svelte:head>

<div class="w-full max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
	<!-- Breadcrumbs -->
	<nav class="flex mb-6 text-sm">
		<ol class="inline-flex items-center space-x-2">
			<li>
				<a href="/" class="text-slate-500 dark:text-slate-400 hover:text-primary">Inicio</a>
			</li>
			<li class="flex items-center">
				<span class="mx-2 text-slate-400">/</span>
				<span class="text-text-light dark:text-text-dark font-medium">Destacados</span>
			</li>
		</ol>
	</nav>
	
	<div class="grid grid-cols-1 lg:grid-cols-4 gap-8">
		<!-- Sidebar Filtros -->
		<aside class="lg:col-span-1 hidden lg:block">
			<div class="sticky top-24 max-h-[calc(100vh-7rem)] overflow-y-auto pr-2 pb-4">
				<div class="flex items-center justify-between mb-4">
					<h3 class="text-lg font-bold">Filtros</h3>
					{#if filtrosActivos.length > 0}
						<button on:click={limpiarFiltros} class="text-sm text-primary hover:underline">
							Limpiar
						</button>
					{/if}
				</div>
				
				<div class="space-y-6">
					<!-- Destacado Badge -->
					<div class="bg-primary/10 dark:bg-primary/20 rounded-lg p-4 border border-primary/30">
						<div class="flex items-center gap-2 text-primary">
							<TrendingUp size={20} />
							<span class="font-bold text-sm">Productos Destacados</span>
						</div>
						<p class="text-xs text-slate-600 dark:text-slate-400 mt-2">
							Los más vendidos y mejor valorados
						</p>
					</div>
					
					<!-- Familia -->
					<div class="border-b border-border-light dark:border-border-dark pb-6">
						<h4 class="font-semibold mb-3">Familia</h4>
						<div class="space-y-2">
							{#each familias as familia}
								<label class="flex items-center cursor-pointer">
									<input
										type="radio"
										name="familia"
										value={familia.id_familia}
										bind:group={familiaSeleccionada}
										class="w-4 h-4 text-primary focus:ring-primary focus:ring-2"
									/>
									<span class="ml-3 text-sm">{familia.nombre}</span>
								</label>
							{/each}
						</div>
					</div>
					
					<!-- Categoría -->
					{#if categorias.length > 0}
						<div class="border-b border-border-light dark:border-border-dark pb-6">
							<h4 class="font-semibold mb-3">Categoría</h4>
							<div class="space-y-2">
								{#each categorias as categoria}
									<label class="flex items-center cursor-pointer">
										<input
											type="radio"
											name="categoria"
											value={categoria.id_categoria}
											bind:group={categoriaSeleccionada}
											class="w-4 h-4 text-primary focus:ring-primary focus:ring-2"
										/>
										<span class="ml-3 text-sm">{categoria.nombre}</span>
									</label>
								{/each}
							</div>
						</div>
					{/if}
					
					<!-- Subcategoría -->
					{#if subcategorias.length > 0}
						<div class="border-b border-border-light dark:border-border-dark pb-6">
							<h4 class="font-semibold mb-3">Subcategoría</h4>
							<div class="space-y-2">
								{#each subcategorias as subcategoria}
									<label class="flex items-center cursor-pointer">
										<input
											type="radio"
											name="subcategoria"
											value={subcategoria.id_subcategoria}
											bind:group={subcategoriaSeleccionada}
											class="w-4 h-4 text-primary focus:ring-primary focus:ring-2"
										/>
										<span class="ml-3 text-sm">{subcategoria.nombre}</span>
									</label>
								{/each}
							</div>
						</div>
					{/if}
					
					<!-- Marca -->
					<div class="border-b border-border-light dark:border-border-dark pb-6">
						<h4 class="font-semibold mb-3">Marca</h4>
						<div class="space-y-2 max-h-60 overflow-y-auto">
							{#each marcas as marca}
								<label class="flex items-center cursor-pointer">
									<input
										type="radio"
										name="marca"
										value={marca.id_marca}
										bind:group={marcaSeleccionada}
										class="w-4 h-4 text-primary focus:ring-primary focus:ring-2"
									/>
									<span class="ml-3 text-sm">{marca.nombre}</span>
								</label>
							{/each}
						</div>
					</div>
					
					<!-- Rango de Precio -->
					<div>
						<h4 class="font-semibold mb-3">Rango de Precio</h4>
						<div class="flex items-center gap-2">
							<input
								type="number"
								placeholder="Min"
								bind:value={precioMin}
								class="w-full px-3 py-2 border border-border-light dark:border-border-dark rounded-lg bg-surface-light dark:bg-surface-dark text-sm focus:ring-2 focus:ring-primary/50"
							/>
							<span>-</span>
							<input
								type="number"
								placeholder="Max"
								bind:value={precioMax}
								class="w-full px-3 py-2 border border-border-light dark:border-border-dark rounded-lg bg-surface-light dark:bg-surface-dark text-sm focus:ring-2 focus:ring-primary/50"
							/>
						</div>
					</div>
				</div>
			</div>
		</aside>
		
		<!-- Contenido Principal -->
		<div class="lg:col-span-3">
			<!-- Header -->
			<div class="mb-6">
				<div class="flex items-center gap-3 mb-2">
					<TrendingUp size={32} class="text-primary" />
					<h1 class="text-3xl md:text-4xl font-black tracking-tight">Productos Destacados</h1>
				</div>
				<p class="text-slate-600 dark:text-slate-400">
					Nuestra selección de productos más vendidos y mejor valorados por nuestros clientes
				</p>
			</div>
			
			<!-- Filtros Activos -->
			{#if filtrosActivos.length > 0}
				<div class="flex flex-wrap gap-2 mb-4">
					{#each filtrosActivos as filtro}
						{#if filtro && 'nombre' in filtro}
							<span class="inline-flex items-center gap-2 px-3 py-1 bg-primary/10 dark:bg-primary/20 text-primary rounded-full text-sm">
								{filtro.nombre}
								<button on:click={() => {
									if ('id_familia' in filtro) familiaSeleccionada = null;
									else if ('id_categoria' in filtro) categoriaSeleccionada = null;
									else if ('id_subcategoria' in filtro) subcategoriaSeleccionada = null;
									else if ('id_marca' in filtro) marcaSeleccionada = null;
								}} class="hover:text-red-500">×</button>
							</span>
						{:else if filtro && 'tipo' in filtro && filtro.tipo === 'precio'}
							<span class="inline-flex items-center gap-2 px-3 py-1 bg-primary/10 dark:bg-primary/20 text-primary rounded-full text-sm">
								S/. {filtro.min || '0'} - S/. {filtro.max || '∞'}
								<button on:click={() => { precioMin = null; precioMax = null; }} class="hover:text-red-500">×</button>
							</span>
						{/if}
					{/each}
				</div>
			{/if}
			
			<!-- Toolbar -->
			<div class="flex flex-wrap items-center justify-between gap-4 p-4 mb-6 rounded-lg border border-border-light dark:border-border-dark">
				<div class="flex items-center gap-4">
					<p class="text-sm text-slate-600 dark:text-slate-400">
						<span class="font-bold text-primary">{totalProductos}</span> productos destacados
					</p>
				</div>
				
				<div class="flex items-center gap-4">
					<select
						bind:value={ordenamiento}
						class="py-2 px-4 rounded-lg border border-border-light dark:border-border-dark text-sm font-medium focus:ring-2 focus:ring-primary/50"
					>
						<option value="mas_vendidos">Más Vendidos</option>
						<option value="valoracion">Mejor Valorados</option>
						<option value="precio_asc">Precio: Menor a Mayor</option>
						<option value="precio_desc">Precio: Mayor a Menor</option>
						<option value="nombre_asc">Nombre: A-Z</option>
					</select>
					
					<div class="hidden sm:flex items-center gap-2 border border-border-light dark:border-border-dark rounded-lg overflow-hidden">
						<button
							on:click={() => vistaGrid = true}
							class="p-2 {vistaGrid ? 'bg-primary text-white' : 'hover:bg-slate-100 dark:hover:bg-slate-700'}"
						>
							<Grid3x3 size={20} />
						</button>
						<button
							on:click={() => vistaGrid = false}
							class="p-2 {!vistaGrid ? 'bg-primary text-white' : 'hover:bg-slate-100 dark:hover:bg-slate-700'}"
						>
							<List size={20} />
						</button>
					</div>
				</div>
			</div>
			
			<!-- Productos -->
			{#if loading}
				<div class="flex items-center justify-center py-20">
					<div class="animate-spin rounded-full h-12 w-12 border-b-2 border-primary"></div>
				</div>
			{:else if error}
				<div class="bg-red-100 dark:bg-red-900/20 text-red-700 dark:text-red-400 p-6 rounded-lg text-center">
					{error}
				</div>
			{:else if productos.length === 0}
				<div class="text-center py-20">
					<TrendingUp size={64} class="mx-auto text-slate-400 mb-4" />
					<h3 class="text-xl font-bold mb-2">No hay productos destacados</h3>
					<p class="text-slate-600 dark:text-slate-400 mb-6">
						Intenta ajustar los filtros
					</p>
					<button
						on:click={limpiarFiltros}
						class="bg-primary text-white font-bold py-3 px-6 rounded-lg hover:bg-primary/90 transition-colors"
					>
						Limpiar filtros
					</button>
				</div>
			{:else}
				<div class="grid grid-cols-1 {vistaGrid ? 'sm:grid-cols-2 xl:grid-cols-3' : ''} gap-6">
					{#each productos as producto}
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
			{/if}
		</div>
	</div>
</div>
