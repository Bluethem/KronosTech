<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { catalogoService, type Producto, type Familia, type Categoria, type Subcategoria } from '$lib/services/api';
	import { cartService } from '$lib/services/cart';
	import { isAuthenticated } from '$lib/stores/auth';
	import { Filter, ChevronDown, X, Grid3x3, List, ShoppingCart, Star, CheckCircle } from 'lucide-svelte';

	let productos: Producto[] = [];
	let loading = true;
	let error: string | null = null;
	let addingToCart: { [key: number]: boolean } = {};
	
	// Datos para filtros
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
	let busqueda = '';
	let enStock = true;
	let destacados = false;
	let ofertas = false;
	let nuevos = false;
	let orderBy = 'relevancia';
	
	// UI state
	let vistaLista = true;
	let filtrosMovilAbiertos = false;
	
	// Paginación
	let currentPage = 1;
	let totalPages = 1;
	let totalProductos = 0;
	let limit = 12;
	
	$: offset = (currentPage - 1) * limit;
	
	// Filtros activos
	$: filtrosActivos = [
		familiaSeleccionada && familias.find(f => f.id_familia === familiaSeleccionada) && { label: familias.find(f => f.id_familia === familiaSeleccionada)!.nombre, key: 'familia' as const },
		categoriaSeleccionada && categorias.find(c => c.id_categoria === categoriaSeleccionada) && { label: categorias.find(c => c.id_categoria === categoriaSeleccionada)!.nombre, key: 'categoria' as const },
		subcategoriaSeleccionada && subcategorias.find(s => s.id_subcategoria === subcategoriaSeleccionada) && { label: subcategorias.find(s => s.id_subcategoria === subcategoriaSeleccionada)!.nombre, key: 'subcategoria' as const },
		marcaSeleccionada && marcas.find(m => m.id_marca === marcaSeleccionada) && { label: marcas.find(m => m.id_marca === marcaSeleccionada)!.nombre, key: 'marca' as const },
		enStock && { label: 'En stock', key: 'enStock' as const },
		destacados && { label: 'Destacados', key: 'destacados' as const },
		ofertas && { label: 'Ofertas', key: 'ofertas' as const },
		nuevos && { label: 'Nuevos', key: 'nuevos' as const },
	].filter((f): f is { label: string; key: string } => Boolean(f));

	async function cargarProductos() {
		loading = true;
		error = null;
		try {
			const response = await catalogoService.getProductos({
				search: busqueda || undefined,
				familia: familiaSeleccionada || undefined,
				categoria: categoriaSeleccionada || undefined,
				subcategoria: subcategoriaSeleccionada || undefined,
				marca: marcaSeleccionada || undefined,
				precio_min: precioMin > 0 ? precioMin : undefined,
				precio_max: precioMax < 5000 ? precioMax : undefined,
				en_stock: enStock || undefined,
				destacados: destacados || undefined,
				ofertas: ofertas || undefined,
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
			error = 'Error al cargar productos';
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
		switch(key) {
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
			case 'subcategoria': subcategoriaSeleccionada = null; break;
			case 'marca': marcaSeleccionada = null; break;
			case 'enStock': enStock = false; break;
			case 'destacados': destacados = false; break;
			case 'ofertas': ofertas = false; break;
			case 'nuevos': nuevos = false; break;
		}
		currentPage = 1;
		cargarProductos();
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
		ofertas = false;
		nuevos = false;
		precioMin = 0;
		precioMax = 5000;
		currentPage = 1;
		cargarProductos();
	}

	function cambiarPagina(pagina: number) {
		currentPage = pagina;
		cargarProductos();
		window.scrollTo({ top: 0, behavior: 'smooth' });
	}

	async function handleAddToCart(producto: Producto) {
		// Verificar si está autenticado
		if (!$isAuthenticated) {
			await goto('/login?redirect=' + encodeURIComponent(window.location.pathname));
			return;
		}

		if (addingToCart[producto.id_producto_detalle] || producto.stock_disponible === 0) return;

		addingToCart[producto.id_producto_detalle] = true;
		try {
			await cartService.addItem({
				id_producto_detalle: producto.id_producto_detalle,
				cantidad: 1
			});
			// Redirigir al carrito
			await goto('/carrito');
		} catch (error: any) {
			alert(error.message || 'Error al agregar al carrito');
			addingToCart[producto.id_producto_detalle] = false;
		}
	}

	onMount(() => {
		cargarFiltros();
		cargarProductos();
	});

	$: if (orderBy) {
		currentPage = 1;
		cargarProductos();
	}

	$: if (familiaSeleccionada !== null) {
		cargarCategorias();
		currentPage = 1;
		cargarProductos();
	}

	$: if (categoriaSeleccionada !== null) {
		cargarSubcategorias();
		currentPage = 1;
		cargarProductos();
	}
</script>

<svelte:head>
	<title>Catálogo de Productos - KronosTech</title>
	<meta name="description" content="Explora nuestro catálogo completo de componentes de PC, periféricos gaming y tecnología." />
</svelte:head>

<div class="w-full max-w-screen-xl mx-auto px-4 sm:px-6 lg:px-10 py-8">
	<div class="grid grid-cols-1 lg:grid-cols-4 gap-8">
		<!-- Sidebar de Filtros (Desktop) -->
		<aside class="lg:col-span-1 hidden lg:block">
			<div class="sticky top-24 space-y-6 max-h-[calc(100vh-7rem)] overflow-y-auto pr-2 pb-4">
				<h3 class="text-xl font-bold">Filtros</h3>

				<!-- Familia -->
				<div class="space-y-3 border-b border-border-light dark:border-border-dark pb-6">
					<h4 class="font-semibold">Familia</h4>
					<select 
						bind:value={familiaSeleccionada}
						class="w-full py-2 px-3 rounded-lg bg-slate-100 dark:bg-slate-700 border-none focus:ring-2 focus:ring-primary/50 text-sm"
					>
						<option value={null}>Todas las familias</option>
						{#each familias as familia}
							<option value={familia.id_familia}>{familia.nombre}</option>
						{/each}
					</select>
				</div>

				<!-- Categoría (depende de Familia) -->
				{#if familiaSeleccionada && categorias.length > 0}
					<div class="space-y-3 border-b border-border-light dark:border-border-dark pb-6">
						<h4 class="font-semibold">Categoría</h4>
						<select 
							bind:value={categoriaSeleccionada}
							class="w-full py-2 px-3 rounded-lg bg-slate-100 dark:bg-slate-700 border-none focus:ring-2 focus:ring-primary/50 text-sm"
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
					<div class="space-y-3 border-b border-border-light dark:border-border-dark pb-6">
						<h4 class="font-semibold">Subcategoría</h4>
						<select 
							bind:value={subcategoriaSeleccionada}
							on:change={() => { currentPage = 1; cargarProductos(); }}
							class="w-full py-2 px-3 rounded-lg bg-slate-100 dark:bg-slate-700 border-none focus:ring-2 focus:ring-primary/50 text-sm"
						>
							<option value={null}>Todas las subcategorías</option>
							{#each subcategorias as subcategoria}
								<option value={subcategoria.id_subcategoria}>{subcategoria.nombre}</option>
							{/each}
						</select>
					</div>
				{/if}

				<!-- Marca -->
				<div class="space-y-3 border-b border-border-light dark:border-border-dark pb-6">
					<h4 class="font-semibold">Marca</h4>
					<div class="space-y-2 max-h-48 overflow-y-auto">
						<label class="flex items-center gap-3 cursor-pointer">
							<input 
								type="radio" 
								name="marca"
								checked={marcaSeleccionada === null}
								on:change={() => { marcaSeleccionada = null; currentPage = 1; cargarProductos(); }}
								class="form-radio text-primary focus:ring-primary/50"
							/>
							<span class="text-sm">Todas las marcas</span>
						</label>
						{#each marcas as marca}
							<label class="flex items-center gap-3 cursor-pointer">
								<input 
									type="radio" 
									name="marca"
									value={marca.id_marca}
									checked={marcaSeleccionada === marca.id_marca}
									on:change={() => { marcaSeleccionada = marca.id_marca; currentPage = 1; cargarProductos(); }}
									class="form-radio text-primary focus:ring-primary/50"
								/>
								<span class="text-sm">{marca.nombre}</span>
							</label>
						{/each}
					</div>
				</div>

				<!-- Rango de Precio -->
				<div class="space-y-4 border-b border-border-light dark:border-border-dark pb-6">
					<h4 class="font-semibold">Rango de Precio</h4>
					<div class="space-y-3">
						<input 
							type="range" 
							min="0" 
							max="5000" 
							bind:value={precioMin}
							on:change={() => cargarProductos()}
							class="w-full h-2 bg-slate-200 dark:bg-surface-dark rounded-lg appearance-none cursor-pointer accent-primary"
						/>
						<input 
							type="range" 
							min="0" 
							max="5000" 
							bind:value={precioMax}
							on:change={() => cargarProductos()}
							class="w-full h-2 bg-slate-200 dark:bg-surface-dark rounded-lg appearance-none cursor-pointer accent-primary"
						/>
					</div>
					<div class="flex justify-between text-sm text-slate-600 dark:text-slate-400">
						<span>S/. {precioMin}</span>
						<span>S/. {precioMax}</span>
					</div>
				</div>


				<!-- Disponibilidad y Destacados -->
				<div class="space-y-3">
					<h4 class="font-semibold">Filtros Rápidos</h4>
					<label class="flex items-center gap-3 cursor-pointer">
						<input 
							type="checkbox" 
							bind:checked={enStock}
							on:change={() => { currentPage = 1; cargarProductos(); }}
							class="form-checkbox rounded text-primary focus:ring-primary/50"
						/>
						<span class="text-sm">Solo en stock</span>
					</label>
					<label class="flex items-center gap-3 cursor-pointer">
						<input 
							type="checkbox" 
							bind:checked={destacados}
							on:change={() => { currentPage = 1; cargarProductos(); }}
							class="form-checkbox rounded text-primary focus:ring-primary/50"
						/>
						<span class="text-sm">Destacados</span>
					</label>
					<label class="flex items-center gap-3 cursor-pointer">
						<input 
							type="checkbox" 
							bind:checked={ofertas}
							on:change={() => { currentPage = 1; cargarProductos(); }}
							class="form-checkbox rounded text-primary focus:ring-primary/50"
						/>
						<span class="text-sm">Ofertas</span>
					</label>
					<label class="flex items-center gap-3 cursor-pointer">
						<input 
							type="checkbox" 
							bind:checked={nuevos}
							on:change={() => { currentPage = 1; cargarProductos(); }}
							class="form-checkbox rounded text-primary focus:ring-primary/50"
						/>
						<span class="text-sm">Nuevos</span>
					</label>
				</div>
			</div>
		</aside>

		<!-- Contenido Principal -->
		<div class="lg:col-span-3">
			<!-- Breadcrumbs -->
			<nav class="flex flex-wrap gap-2 mb-4 text-sm">
				<a href="/" class="text-slate-600 dark:text-slate-400 hover:text-primary">Inicio</a>
				<span class="text-slate-600 dark:text-slate-400">/</span>
				<span class="text-text-light dark:text-text-dark font-medium">Catálogo</span>
			</nav>

			<!-- Título -->
			<h1 class="text-3xl md:text-4xl font-black tracking-tight mb-6">
				Catálogo de Productos
			</h1>

			<!-- Toolbar -->
			<div class="flex flex-wrap items-center justify-between gap-4 mb-4 p-3 border border-border-light dark:border-border-dark rounded-xl bg-surface-light dark:bg-surface-dark">
				<div class="flex items-center gap-2">
					<button 
						on:click={() => filtrosMovilAbiertos = !filtrosMovilAbiertos}
						class="lg:hidden flex items-center gap-2 py-2 px-4 rounded-lg bg-slate-200 dark:bg-slate-700 text-sm font-semibold"
					>
						<Filter size={18} />
						Filtros
					</button>
					
					<select 
						bind:value={orderBy}
						class="py-2 px-4 rounded-lg bg-slate-200 dark:bg-slate-700 text-sm font-semibold border-none focus:ring-2 focus:ring-primary/50"
					>
						<option value="relevancia">Relevancia</option>
						<option value="precio_asc">Precio: Menor a Mayor</option>
						<option value="precio_desc">Precio: Mayor a Menor</option>
						<option value="nombre_asc">Nombre: A-Z</option>
						<option value="nombre_desc">Nombre: Z-A</option>
						<option value="valoracion">Mejor Valorados</option>
						<option value="mas_vendidos">Más Vendidos</option>
					</select>
				</div>

				<div class="flex items-center gap-2">
					<button 
						on:click={() => vistaLista = false}
						class="p-2.5 rounded-lg transition-colors {!vistaLista ? 'bg-primary text-white' : 'bg-slate-200 dark:bg-slate-700 text-slate-600 dark:text-slate-400'}"
					>
						<Grid3x3 size={20} />
					</button>
					<button 
						on:click={() => vistaLista = true}
						class="p-2.5 rounded-lg transition-colors {vistaLista ? 'bg-primary text-white' : 'bg-slate-200 dark:bg-slate-700 text-slate-600 dark:text-slate-400'}"
					>
						<List size={20} />
					</button>
				</div>
			</div>

			<!-- Filtros Activos (Chips) -->
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

			<!-- Grid/Lista de Productos -->
			{#if loading}
				<div class="space-y-4">
					{#each Array(4) as _}
						<div class="flex gap-6 p-4 border border-border-light dark:border-border-dark rounded-xl bg-surface-light dark:bg-surface-dark animate-pulse">
							<div class="w-48 h-40 bg-slate-200 dark:bg-slate-700 rounded-lg"></div>
							<div class="flex-1 space-y-3">
								<div class="h-5 w-3/4 bg-slate-200 dark:bg-slate-700 rounded"></div>
								<div class="h-4 w-1/2 bg-slate-200 dark:bg-slate-700 rounded"></div>
								<div class="h-4 w-full bg-slate-200 dark:bg-slate-700 rounded"></div>
							</div>
						</div>
					{/each}
				</div>
			{:else if error}
				<div class="bg-red-100 dark:bg-red-900/20 text-red-700 dark:text-red-400 p-4 rounded-lg">
					{error}
				</div>
			{:else if productos.length === 0}
				<div class="text-center py-12">
					<p class="text-lg text-slate-600 dark:text-slate-400">No se encontraron productos</p>
					<button on:click={limpiarFiltros} class="mt-4 text-primary font-semibold hover:underline">
						Limpiar filtros
					</button>
				</div>
			{:else}
				<div class={vistaLista ? 'space-y-4' : 'grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6'}>
					{#each productos as producto}
						{#if vistaLista}
							<!-- Vista de Lista -->
							<a 
								href="/producto/{producto.id_producto_detalle}"
								class="flex flex-col sm:flex-row items-start gap-6 p-4 border border-border-light dark:border-border-dark rounded-xl bg-surface-light dark:bg-surface-dark hover:shadow-lg transition-all group"
							>
								<div class="w-full sm:w-48 h-40 flex-shrink-0 bg-white rounded-lg flex items-center justify-center p-2">
									<img 
										src={producto.imagen_principal || 'https://placehold.co/400x300/1e293b/94a3b8?text=Producto'} 
										alt={producto.nombre}
										class="max-h-full max-w-full object-contain group-hover:scale-105 transition-transform"
									/>
								</div>
								<div class="flex-grow w-full">
									<h3 class="text-lg font-bold group-hover:text-primary transition-colors mb-2">
										{producto.nombre}
									</h3>
									<div class="flex items-center gap-2 mb-2">
										{#if producto.valoracion_promedio}
											<div class="flex text-yellow-500">
												{#each Array(5) as _, i}
													<Star size={14} fill={i < Math.floor(producto.valoracion_promedio) ? 'currentColor' : 'none'} />
												{/each}
											</div>
											<span class="text-sm text-slate-600 dark:text-slate-400">({producto.total_valoraciones})</span>
										{/if}
									</div>
									<p class="text-sm text-slate-600 dark:text-slate-400 mb-3 line-clamp-2">
										{producto.marca} • SKU: {producto.sku}
									</p>
									{#if producto.stock_disponible > 0}
										<div class="flex items-center gap-2 text-sm font-semibold text-green-600 dark:text-green-400">
											<CheckCircle size={16} />
											En stock ({producto.stock_disponible})
										</div>
									{:else}
										<div class="flex items-center gap-2 text-sm font-semibold text-red-600 dark:text-red-400">
											Agotado
										</div>
									{/if}
								</div>
								<div class="flex-shrink-0 w-full sm:w-auto text-left sm:text-right">
									<p class="text-2xl font-bold mb-2">S/. {producto.precio_venta.toFixed(2)}</p>
									{#if producto.precio_base > producto.precio_venta}
										<p class="text-sm line-through text-slate-500 mb-4">S/. {producto.precio_base.toFixed(2)}</p>
									{/if}
									<button
										class="w-full sm:w-auto flex items-center justify-center gap-2 bg-primary text-white font-bold py-2.5 px-6 rounded-lg hover:bg-primary/90 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
										disabled={producto.stock_disponible === 0 || addingToCart[producto.id_producto_detalle]}
										on:click|preventDefault|stopPropagation={() => handleAddToCart(producto)}
									>
										<ShoppingCart size={18} />
										{#if addingToCart[producto.id_producto_detalle]}
											Agregando...
										{:else if producto.stock_disponible === 0}
											Agotado
										{:else}
											Añadir
										{/if}
									</button>
								</div>
							</a>
						{:else}
							<!-- Vista de Grid (Card compacto) -->
							<a 
								href="/producto/{producto.id_producto_detalle}"
								class="flex flex-col gap-4 rounded-xl bg-surface-light dark:bg-surface-dark shadow-md hover:shadow-xl transition-all overflow-hidden group"
							>
								<div class="relative w-full aspect-video bg-white flex items-center justify-center p-4">
									<img 
										src={producto.imagen_principal || 'https://placehold.co/400x300/1e293b/94a3b8?text=Producto'} 
										alt={producto.nombre}
										class="max-h-full max-w-full object-contain group-hover:scale-105 transition-transform"
									/>
									{#if producto.es_oferta}
										<span class="absolute top-2 left-2 bg-red-500 text-white text-xs font-bold px-2 py-1 rounded">
											OFERTA
										</span>
									{/if}
								</div>
								<div class="p-4 pt-0 flex flex-col flex-1">
									<p class="text-xs text-slate-500 dark:text-slate-400 mb-1">{producto.marca}</p>
									<h3 class="text-base font-bold mb-2 line-clamp-2 group-hover:text-primary transition-colors">
										{producto.nombre}
									</h3>
									<div class="flex items-center gap-2 mb-3">
										<p class="text-xl font-bold text-primary">S/. {producto.precio_venta.toFixed(2)}</p>
										{#if producto.precio_base > producto.precio_venta}
											<p class="text-sm line-through text-slate-500">S/. {producto.precio_base.toFixed(2)}</p>
										{/if}
									</div>
									<button
										class="w-full flex items-center justify-center gap-2 bg-primary/20 text-primary font-bold py-2 px-4 rounded-lg hover:bg-primary hover:text-white transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
										disabled={producto.stock_disponible === 0 || addingToCart[producto.id_producto_detalle]}
										on:click|preventDefault|stopPropagation={() => handleAddToCart(producto)}
									>
										<ShoppingCart size={16} />
										{#if addingToCart[producto.id_producto_detalle]}
											Agregando...
										{:else if producto.stock_disponible === 0}
											Agotado
										{:else}
											Añadir
										{/if}
									</button>
								</div>
							</a>
						{/if}
					{/each}
				</div>
			{/if}

			<!-- Paginación -->
			{#if !loading && productos.length > 0}
				<nav class="flex items-center justify-between mt-8 pt-6 border-t border-border-light dark:border-border-dark">
					<div class="hidden sm:block">
						<p class="text-sm text-slate-600 dark:text-slate-400">
							Mostrando <span class="font-medium">{offset + 1}</span> a 
							<span class="font-medium">{Math.min(offset + limit, totalProductos)}</span> de 
							<span class="font-medium">{totalProductos}</span> resultados
						</p>
					</div>
					<div class="flex-1 flex justify-between sm:justify-end gap-2">
						<button 
							on:click={() => cambiarPagina(currentPage - 1)}
							disabled={currentPage === 1}
							class="relative inline-flex items-center px-4 py-2 border border-border-light dark:border-border-dark text-sm font-medium rounded-md disabled:opacity-50 disabled:cursor-not-allowed hover:bg-slate-100 dark:hover:bg-slate-700 transition-colors"
						>
							Anterior
						</button>
						<button 
							on:click={() => cambiarPagina(currentPage + 1)}
							disabled={currentPage === totalPages}
							class="relative inline-flex items-center px-4 py-2 border border-border-light dark:border-border-dark text-sm font-medium rounded-md disabled:opacity-50 disabled:cursor-not-allowed hover:bg-slate-100 dark:hover:bg-slate-700 transition-colors"
						>
							Siguiente
						</button>
					</div>
				</nav>
			{/if}
		</div>
	</div>
</div>

<!-- Filtros Móvil (Modal) -->
{#if filtrosMovilAbiertos}
	<div class="fixed inset-0 z-50 lg:hidden">
		<div class="absolute inset-0 bg-black/50" on:click={() => filtrosMovilAbiertos = false}></div>
		<div class="absolute right-0 top-0 bottom-0 w-80 max-w-full bg-background-light dark:bg-background-dark p-6 overflow-y-auto">
			<button on:click={() => filtrosMovilAbiertos = false} class="mb-4">
				<X size={24} />
			</button>
			<h3 class="text-xl font-bold mb-6">Filtros</h3>
			
			<!-- Mismo contenido del sidebar -->
			<div class="space-y-6">
				<!-- Familia -->
				<div class="space-y-3 border-b border-border-light dark:border-border-dark pb-6">
					<h4 class="font-semibold">Familia</h4>
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

				<!-- Categoría -->
				{#if familiaSeleccionada && categorias.length > 0}
					<div class="space-y-3 border-b border-border-light dark:border-border-dark pb-6">
						<h4 class="font-semibold">Categoría</h4>
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

				<!-- Subcategoría -->
				{#if categoriaSeleccionada && subcategorias.length > 0}
					<div class="space-y-3 border-b border-border-light dark:border-border-dark pb-6">
						<h4 class="font-semibold">Subcategoría</h4>
						<select 
							bind:value={subcategoriaSeleccionada}
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
				<div class="space-y-3 border-b border-border-light dark:border-border-dark pb-6">
					<h4 class="font-semibold">Marca</h4>
					<div class="space-y-2 max-h-48 overflow-y-auto">
						<label class="flex items-center gap-3 cursor-pointer">
							<input 
								type="radio" 
								name="marca-mobile"
								checked={marcaSeleccionada === null}
								on:change={() => { marcaSeleccionada = null; }}
								class="form-radio text-primary"
							/>
							<span class="text-sm">Todas las marcas</span>
						</label>
						{#each marcas as marca}
							<label class="flex items-center gap-3 cursor-pointer">
								<input 
									type="radio" 
									name="marca-mobile"
									value={marca.id_marca}
									checked={marcaSeleccionada === marca.id_marca}
									on:change={() => { marcaSeleccionada = marca.id_marca; }}
									class="form-radio text-primary"
								/>
								<span class="text-sm">{marca.nombre}</span>
							</label>
						{/each}
					</div>
				</div>

				<!-- Rango de Precio -->
				<div class="space-y-4 border-b border-border-light dark:border-border-dark pb-6">
					<h4 class="font-semibold">Rango de Precio</h4>
					<div class="space-y-3">
						<input 
							type="range" 
							min="0" 
							max="5000" 
							bind:value={precioMin}
							class="w-full accent-primary"
						/>
						<input 
							type="range" 
							min="0" 
							max="5000" 
							bind:value={precioMax}
							class="w-full accent-primary"
						/>
					</div>
					<div class="flex justify-between text-sm">
						<span>S/. {precioMin}</span>
						<span>S/. {precioMax}</span>
					</div>
				</div>

				<!-- Filtros Rápidos -->
				<div class="space-y-3 border-b border-border-light dark:border-border-dark pb-6">
					<h4 class="font-semibold">Filtros Rápidos</h4>
					<label class="flex items-center gap-3 cursor-pointer">
						<input 
							type="checkbox" 
							bind:checked={enStock}
							class="form-checkbox rounded text-primary"
						/>
						<span class="text-sm">Solo en stock</span>
					</label>
					<label class="flex items-center gap-3 cursor-pointer">
						<input 
							type="checkbox" 
							bind:checked={destacados}
							class="form-checkbox rounded text-primary"
						/>
						<span class="text-sm">Destacados</span>
					</label>
					<label class="flex items-center gap-3 cursor-pointer">
						<input 
							type="checkbox" 
							bind:checked={ofertas}
							class="form-checkbox rounded text-primary"
						/>
						<span class="text-sm">Ofertas</span>
					</label>
					<label class="flex items-center gap-3 cursor-pointer">
						<input 
							type="checkbox" 
							bind:checked={nuevos}
							class="form-checkbox rounded text-primary"
						/>
						<span class="text-sm">Nuevos</span>
					</label>
				</div>

				<button 
					on:click={() => { currentPage = 1; cargarProductos(); filtrosMovilAbiertos = false; }}
					class="w-full bg-primary text-white font-bold py-3 rounded-lg hover:bg-primary/90 transition-colors"
				>
					Aplicar Filtros
				</button>
			</div>
		</div>
	</div>
{/if}
