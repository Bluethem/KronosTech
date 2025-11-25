<script lang="ts">
	import { onMount } from 'svelte';
	import { catalogoService, type Producto, type Familia, type Categoria, type Subcategoria } from '$lib/services/api';
	import { Percent, Clock, ChevronRight, Heart, X, Filter } from 'lucide-svelte';
	import ProductCard from '$lib/components/ProductCard.svelte';
	
	let productos: Producto[] = [];
	let loading = true;
	let error: string | null = null;
	
	// Datos para filtros (reutilizados del catálogo)
	let familias: Familia[] = [];
	let categorias: Categoria[] = [];
	let subcategorias: Subcategoria[] = [];
	let marcas: any[] = [];
	
	// Estado de filtros
	let familiaSeleccionada: number | null = null;
	let categoriaSeleccionada: number | null = null;
	let subcategoriaSeleccionada: number | null = null;
	let marcaSeleccionada: number | null = null;
	let precioMin = 0;
	let precioMax = 5000;
	let enStock = true;
	let destacados = false;
	let nuevos = false;
	let orderBy = 'relevancia';
	
	// UI state
	let filtrosMovilAbiertos = false;
	
	// Paginación
	let currentPage = 1;
	let totalPages = 1;
	let totalProductos = 0;
	let limit = 12;
	
	$: offset = (currentPage - 1) * limit;
	
	// Filtros activos (sin incluir "ofertas" porque esta página ya es solo de ofertas)
	$: filtrosActivos = [
		familiaSeleccionada && familias.find((f) => f.id_familia === familiaSeleccionada) && {
			label: familias.find((f) => f.id_familia === familiaSeleccionada)!.nombre,
			key: 'familia' as const
		},
		categoriaSeleccionada && categorias.find((c) => c.id_categoria === categoriaSeleccionada) && {
			label: categorias.find((c) => c.id_categoria === categoriaSeleccionada)!.nombre,
			key: 'categoria' as const
		},
		subcategoriaSeleccionada && subcategorias.find((s) => s.id_subcategoria === subcategoriaSeleccionada) && {
			label: subcategorias.find((s) => s.id_subcategoria === subcategoriaSeleccionada)!.nombre,
			key: 'subcategoria' as const
		},
		marcaSeleccionada && marcas.find((m) => m.id_marca === marcaSeleccionada) && {
			label: marcas.find((m) => m.id_marca === marcaSeleccionada)!.nombre,
			key: 'marca' as const
		},
		enStock && { label: 'En stock', key: 'enStock' as const },
		destacados && { label: 'Destacados', key: 'destacados' as const },
		nuevos && { label: 'Nuevos', key: 'nuevos' as const }
	].filter((f): f is { label: string; key: string } => Boolean(f));

	async function cargarOfertas() {
		loading = true;
		error = null;
		try {
			const response = await catalogoService.getProductos({
				familia: familiaSeleccionada || undefined,
				categoria: categoriaSeleccionada || undefined,
				subcategoria: subcategoriaSeleccionada || undefined,
				marca: marcaSeleccionada || undefined,
				precio_min: precioMin > 0 ? precioMin : undefined,
				precio_max: precioMax < 5000 ? precioMax : undefined,
				en_stock: enStock || undefined,
				destacados: destacados || undefined,
				ofertas: true,
				nuevos: nuevos || undefined,
				order_by: orderBy !== 'relevancia' ? orderBy : undefined,
				limit,
				offset
			});
			
			productos = response.data;
			totalProductos = response.pagination.total;
			totalPages = response.pagination.total_pages;
			currentPage = response.pagination.current_page;
		} catch (err) {
			error = 'Error al cargar las ofertas';
			console.error(err);
		} finally {
			loading = false;
		}
	}

	async function cargarFiltros() {
		try {
			const [familiasData, marcasData] = await Promise.all([
				catalogoService.getFamilias(),
				catalogoService.getMarcas()
			]);
			familias = familiasData;
			marcas = marcasData;
		} catch (err) {
			console.error('Error al cargar filtros:', err);
		}
	}

	async function cargarCategorias() {
		if (familiaSeleccionada) {
			categorias = await catalogoService.getCategorias(familiaSeleccionada);
		} else {
			categorias = [];
			categoriaSeleccionada = null;
		}
		subcategorias = [];
		subcategoriaSeleccionada = null;
	}

	async function cargarSubcategorias() {
		if (categoriaSeleccionada) {
			subcategorias = await catalogoService.getSubcategorias(categoriaSeleccionada);
		} else {
			subcategorias = [];
			subcategoriaSeleccionada = null;
		}
	}

	function quitarFiltro(key: string) {
		switch (key) {
			case 'familia':
				familiaSeleccionada = null;
				categoriaSeleccionada = null;
				subcategoriaSeleccionada = null;
				categorias = [];
				subcategorias = [];
				break;
			case 'categoria':
				categoriaSeleccionada = null;
				subcategoriaSeleccionada = null;
				subcategorias = [];
				break;
			case 'subcategoria':
				subcategoriaSeleccionada = null;
				break;
			case 'marca':
				marcaSeleccionada = null;
				break;
			case 'enStock':
				enStock = false;
				break;
			case 'destacados':
				destacados = false;
				break;
			case 'nuevos':
				nuevos = false;
				break;
		}
		currentPage = 1;
		cargarOfertas();
	}

	function limpiarFiltros() {
		familiaSeleccionada = null;
		categoriaSeleccionada = null;
		subcategoriaSeleccionada = null;
		marcaSeleccionada = null;
		categorias = [];
		subcategorias = [];
		enStock = false;
		destacados = false;
		nuevos = false;
		precioMin = 0;
		precioMax = 5000;
		currentPage = 1;
		cargarOfertas();
	}

	function cambiarPagina(pagina: number) {
		currentPage = pagina;
		cargarOfertas();
		window.scrollTo({ top: 0, behavior: 'smooth' });
	}

	onMount(() => {
		cargarFiltros();
		cargarOfertas();
	});

	$: if (orderBy) {
		currentPage = 1;
		cargarOfertas();
	}

	$: if (familiaSeleccionada !== null) {
		cargarCategorias();
		currentPage = 1;
		cargarOfertas();
	}

	$: if (categoriaSeleccionada !== null) {
		cargarSubcategorias();
		currentPage = 1;
		cargarOfertas();
	}
</script>

<svelte:head>
	<title>Ofertas Imperdibles - KronosTech</title>
	<meta name="description" content="Las mejores ofertas en componentes de PC, periféricos gaming y tecnología. ¡Descuentos de hasta 40%!" />
</svelte:head>

<div class="w-full max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
	<!-- Breadcrumbs -->
	<nav class="flex flex-wrap gap-2 mb-6 text-sm">
		<a href="/" class="text-slate-600 dark:text-slate-400 hover:text-primary">Inicio</a>
		<span class="text-slate-600 dark:text-slate-400">/</span>
		<span class="text-text-light dark:text-text-dark font-medium">Ofertas</span>
	</nav>

	<!-- Título -->
	<div class="mb-6">
		<h1 class="text-3xl md:text-4xl font-black tracking-tight mb-2">
			🔥 Ofertas Imperdibles
		</h1>
		<p class="text-slate-600 dark:text-slate-400">
			Los mejores precios en componentes de PC y tecnología
		</p>
	</div>

	<!-- Banner Hero -->
	<div class="relative w-full rounded-xl overflow-hidden mb-8 group h-64 sm:h-80">
		<div class="absolute inset-0 bg-gradient-to-r from-black/80 via-black/60 to-transparent z-10"></div>
		<img 
			src="https://images.unsplash.com/photo-1587202372634-32705e3bf49c?w=1200&h=400&fit=crop" 
			alt="Banner de ofertas especiales"
			class="w-full h-full object-cover"
		/>
		<div class="absolute inset-0 z-20 flex items-center p-6 sm:p-10">
			<div class="max-w-md text-white">
				<h2 class="text-3xl sm:text-4xl font-black leading-tight mb-3">
					Ofertas de la Semana
				</h2>
				<p class="text-base sm:text-lg font-medium text-slate-200 mb-6">
					Equipa tu PC con los mejores componentes a precios increíbles. ¡Descuentos de hasta el 40%!
				</p>
				<a 
					href="#ofertas" 
					class="inline-flex items-center justify-center rounded-lg h-11 px-6 text-sm font-bold bg-primary text-white hover:bg-primary/90 transition-colors shadow-lg"
				>
					<span>Ver todas las ofertas</span>
					<ChevronRight size={18} class="ml-2" />
				</a>
			</div>
		</div>
	</div>

	<!-- Contenido Principal -->
	<div class="flex flex-col lg:flex-row gap-8" id="ofertas">
		<!-- Sidebar de Filtros -->
		<aside class="lg:col-span-1 hidden lg:block">
			<div class="sticky top-24 space-y-6 max-h-[calc(100vh-7rem)] overflow-y-auto pr-2 pb-4">
				<!-- Stats Box -->
				<div class="bg-gradient-to-br from-primary/10 to-primary/5 dark:from-primary/20 dark:to-primary/10 p-5 rounded-lg border border-primary/20">
					<div class="flex items-center gap-3 mb-2">
						<Clock size={24} class="text-primary" />
						<h4 class="font-bold text-primary">Ofertas Flash</h4>
					</div>
					<p class="text-sm text-slate-700 dark:text-slate-300">
						¡Descuentos especiales por tiempo limitado! No te pierdas estas increíbles ofertas.
					</p>
				</div>
				
				<div class="p-5 rounded-lg border border-border-light dark:border-border-dark">
					<h3 class="text-lg font-bold mb-4">Filtros</h3>
					<div class="space-y-5">
						<!-- Indicador En Oferta (solo informativo) -->
						<div>
							<label class="flex items-center bg-primary/10 dark:bg-primary/20 p-3 rounded-lg cursor-not-allowed">
								<input 
									type="checkbox" 
									checked 
									disabled
									class="h-4 w-4 rounded border-primary/50 text-primary"
								/>
								<div class="ml-3 flex items-center gap-2">
									<Percent size={16} class="text-primary" />
									<span class="text-sm font-semibold text-primary">En Oferta</span>
								</div>
							</label>
						</div>

						<!-- Familia -->
						<div>
							<h4 class="font-semibold mb-3 text-sm uppercase tracking-wide">Familia</h4>
							<select
								bind:value={familiaSeleccionada}
								class="w-full py-2 px-3 rounded-lg bg-slate-100 dark:bg-slate-700 border-none text-sm"
							>
								<option value={null}>Todas las familias</option>
								{#each familias as familia}
									<option value={familia.id_familia}>{familia.nombre}</option>
								{/each}
							</select>
						</div>

						<!-- Categoría (depende de Familia) -->
						{#if familiaSeleccionada && categorias.length > 0}
							<div>
								<h4 class="font-semibold mb-3 text-sm uppercase tracking-wide">Categoría</h4>
								<select
									bind:value={categoriaSeleccionada}
									class="w-full py-2 px-3 rounded-lg bg-slate-100 dark:bg-slate-700 border-none text-sm"
								>
									<option value={null}>Todas las categorías</option>
									{#each categorias as categoria}
										<option value={categoria.id_categoria}>{categoria.nombre}</option>
									{/each}
								</select>
							</div>
						{/if}

						<!-- Subcategoría (depende de Categoría) -->
						{#if categoriaSeleccionada && subcategorias.length > 0}
							<div>
								<h4 class="font-semibold mb-3 text-sm uppercase tracking-wide">Subcategoría</h4>
								<select
									bind:value={subcategoriaSeleccionada}
									on:change={() => { currentPage = 1; cargarOfertas(); }}
									class="w-full py-2 px-3 rounded-lg bg-slate-100 dark:bg-slate-700 border-none text-sm"
								>
									<option value={null}>Todas las subcategorías</option>
									{#each subcategorias as subcategoria}
										<option value={subcategoria.id_subcategoria}>{subcategoria.nombre}</option>
									{/each}
								</select>
							</div>
						{/if}

						<!-- Marca -->
						<div>
							<h4 class="font-semibold mb-3 text-sm uppercase tracking-wide">Marca</h4>
							<div class="space-y-2">
								<label class="flex items-center cursor-pointer hover:bg-slate-100 dark:hover:bg-slate-800 p-2 rounded transition-colors">
									<input
										type="radio"
										name="marca"
										checked={marcaSeleccionada === null}
										on:change={() => { marcaSeleccionada = null; currentPage = 1; cargarOfertas(); }}
										class="h-4 w-4 text-primary focus:ring-primary"
									/>
									<span class="ml-2 text-sm">Todas las marcas</span>
								</label>
								{#each marcas as marca}
									<label class="flex items-center cursor-pointer hover:bg-slate-100 dark:hover:bg-slate-800 p-2 rounded transition-colors">
										<input
											type="radio"
											name="marca"
											value={marca.id_marca}
											checked={marcaSeleccionada === marca.id_marca}
											on:change={() => { marcaSeleccionada = marca.id_marca; currentPage = 1; cargarOfertas(); }}
											class="h-4 w-4 text-primary focus:ring-primary"
										/>
										<span class="ml-2 text-sm">{marca.nombre}</span>
									</label>
								{/each}
							</div>
						</div>

						<!-- Precio -->
						<div>
							<h4 class="font-semibold mb-3 text-sm uppercase tracking-wide">Rango de Precio</h4>
							<div class="space-y-3">
								<input
									type="range"
									min="0"
									max="5000"
									bind:value={precioMin}
									on:change={() => cargarOfertas()}
									class="w-full h-2 bg-slate-200 dark:bg-slate-700 rounded-lg appearance-none cursor-pointer accent-primary"
								/>
								<input
									type="range"
									min="0"
									max="5000"
									bind:value={precioMax}
									on:change={() => cargarOfertas()}
									class="w-full h-2 bg-slate-200 dark:bg-slate-700 rounded-lg appearance-none cursor-pointer accent-primary"
								/>
							</div>
							<div class="flex justify-between text-sm text-slate-600 dark:text-slate-400 mt-2">
								<span>S/. {precioMin}</span>
								<span>S/. {precioMax}</span>
							</div>
						</div>

						<!-- Filtros rápidos -->
						<div>
							<h4 class="font-semibold mb-3 text-sm uppercase tracking-wide">Filtros Rápidos</h4>
							<label class="flex items-center gap-3 cursor-pointer">
								<input
									type="checkbox"
									bind:checked={enStock}
									on:change={() => { currentPage = 1; cargarOfertas(); }}
									class="form-checkbox rounded text-primary focus:ring-primary/50"
								/>
								<span class="text-sm">Solo en stock</span>
							</label>
							<label class="flex items-center gap-3 cursor-pointer mt-2">
								<input
									type="checkbox"
									bind:checked={destacados}
									on:change={() => { currentPage = 1; cargarOfertas(); }}
									class="form-checkbox rounded text-primary focus:ring-primary/50"
								/>
								<span class="text-sm">Destacados</span>
							</label>
							<label class="flex items-center gap-3 cursor-pointer mt-2">
								<input
									type="checkbox"
									bind:checked={nuevos}
									on:change={() => { currentPage = 1; cargarOfertas(); }}
									class="form-checkbox rounded text-primary focus:ring-primary/50"
								/>
								<span class="text-sm">Nuevos</span>
							</label>
						</div>
					</div>
				</div>
			</div>
		</aside>

		<!-- Grid de Productos -->
		<div class="w-full lg:w-3/4">
			<!-- Toolbar -->
			<div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 mb-4">
				<div class="flex items-center gap-3">
					<button
						on:click={() => (filtrosMovilAbiertos = !filtrosMovilAbiertos)}
						class="lg:hidden flex items-center gap-2 py-2 px-4 rounded-lg bg-slate-200 dark:bg-slate-700 text-sm font-semibold"
					>
						<Filter size={18} />
						Filtros
					</button>
					<p class="text-slate-600 dark:text-slate-400 text-sm">
						{#if !loading}
							Mostrando {offset + 1}-{Math.min(offset + limit, totalProductos)} de {totalProductos} ofertas
						{/if}
					</p>
				</div>
				<select
					bind:value={orderBy}
					class="py-2 px-4 rounded-lg border border-border-light dark:border-border-dark text-sm font-medium focus:ring-2 focus:ring-primary/50"
				>
					<option value="relevancia">Más Relevantes</option>
					<option value="precio_asc">Precio: Menor a Mayor</option>
					<option value="precio_desc">Precio: Mayor a Menor</option>
					<option value="valoracion">Mejor Valorados</option>
				</select>
			</div>

			<!-- Filtros activos (chips) -->
			{#if filtrosActivos.length > 0}
				<div class="flex flex-wrap items-center gap-3 mb-6">
					<span class="text-sm font-medium text-slate-600 dark:text-slate-400">Filtros activos:</span>
					{#each filtrosActivos as filtro}
						<div class="flex h-8 items-center justify-center gap-2 rounded-full bg-primary/20 pl-4 pr-2">
							<p class="text-primary text-sm font-medium">{filtro.label}</p>
							<button on:click={() => quitarFiltro(filtro.key)} class="text-primary">
								<X size={16} />
							</button>
						</div>
					{/each}
					<button on:click={limpiarFiltros} class="text-sm font-medium text-slate-600 dark:text-slate-400 hover:text-primary">
						Limpiar todos
					</button>
				</div>
			{/if}
			
			<!-- Grid de Productos -->
			{#if loading}
				<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
					{#each Array(6) as _}
						<div class="h-96 rounded-xl bg-slate-200 dark:bg-surface-dark animate-pulse"></div>
					{/each}
				</div>
			{:else if error}
				<div class="bg-red-100 dark:bg-red-900/20 text-red-700 dark:text-red-400 p-6 rounded-lg text-center">
					{error}
				</div>
			{:else if productos.length === 0}
				<div class="text-center py-12">
					<Percent size={48} class="mx-auto text-slate-400 mb-4" />
					<p class="text-lg text-slate-600 dark:text-slate-400 mb-2">No hay ofertas disponibles en este momento</p>
					<p class="text-sm text-slate-500 dark:text-slate-500">Vuelve pronto para ver nuevas promociones</p>
					<a href="/catalogo" class="inline-block mt-4 text-primary font-semibold hover:underline">
						Ver catálogo completo
					</a>
				</div>
			{:else}
				<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
					{#each productos as producto}
						<div class="relative">
							<!-- Badge de descuento grande -->
							{#if producto.precio_base > producto.precio_venta}
								{@const descuento = Math.round(((producto.precio_base - producto.precio_venta) / producto.precio_base) * 100)}
								<div class="absolute top-3 left-3 z-20">
									<span class="inline-flex items-center px-3 py-1 rounded-full text-sm font-bold bg-red-600 text-white shadow-lg">
										-{descuento}%
									</span>
								</div>
							{/if}
							
							<!-- Botón de favoritos -->
							<button class="absolute top-3 right-3 z-20 p-2 rounded-full bg-white/90 dark:bg-surface-dark/90 backdrop-blur-sm text-slate-600 dark:text-slate-400 hover:text-red-500 dark:hover:text-red-500 transition-colors shadow-md">
								<Heart size={18} />
							</button>
							
							<ProductCard {producto} />
						</div>
					{/each}
				</div>
			{/if}
			
			<!-- Paginación -->
			{#if !loading && productos.length > 0 && totalPages > 1}
				<nav class="mt-10 flex items-center justify-center">
					<ul class="inline-flex items-center -space-x-px rounded-lg overflow-hidden border border-border-light dark:border-border-dark">
						<li>
							<button
								on:click={() => cambiarPagina(currentPage - 1)}
								disabled={currentPage === 1}
								class="py-2 px-3 leading-tight hover:bg-slate-100 dark:hover:bg-slate-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
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
										: 'hover:bg-slate-100 dark:hover:bg-slate-700'}"
								>
									{pageNum}
								</button>
							</li>
						{/each}
						
						{#if totalPages > 5}
							<li>
								<span class="py-2 px-3 leading-tight">...</span>
							</li>
							<li>
								<button
									on:click={() => cambiarPagina(totalPages)}
									class="py-2 px-3 leading-tight hover:bg-slate-100 dark:hover:bg-slate-700 transition-colors"
								>
									{totalPages}
								</button>
							</li>
						{/if}
						
						<li>
							<button
								on:click={() => cambiarPagina(currentPage + 1)}
								disabled={currentPage === totalPages}
								class="py-2 px-3 leading-tight hover:bg-slate-100 dark:hover:bg-slate-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
							>
								Siguiente
							</button>
						</li>
					</ul>
				</nav>
			{/if}
		</div>
	</div>
</div>
