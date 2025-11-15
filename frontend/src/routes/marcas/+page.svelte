<script lang="ts">
	import { onMount } from 'svelte';
	import { catalogoService } from '$lib/services/api';
	import { Search, Package } from 'lucide-svelte';
	
	interface Marca {
		id_marca: number;
		nombre: string;
		descripcion?: string;
		logo_url?: string;
		pais_origen?: string;
		sitio_web?: string;
		total_productos?: number;
	}
	
	let marcas: Marca[] = [];
	let marcasFiltradas: Marca[] = [];
	let loading = true;
	let error: string | null = null;
	let searchQuery = '';
	
	// Imágenes de marcas populares (puedes reemplazar con logos reales)
	const marcasLogos: Record<string, string> = {
		'NVIDIA': 'https://images.unsplash.com/photo-1591488320449-011701bb6704?w=400',
		'AMD': 'https://images.unsplash.com/photo-1587202372616-b43abea06c2a?w=400',
		'Intel': 'https://images.unsplash.com/photo-1555617981-dac3880eac6e?w=400',
		'Corsair': 'https://images.unsplash.com/photo-1587829741301-dc798b83add3?w=400',
		'ASUS': 'https://images.unsplash.com/photo-1591799264318-7e6ef8ddb7ea?w=400',
		'MSI': 'https://images.unsplash.com/photo-1593640495253-23196b27a87f?w=400',
		'Gigabyte': 'https://images.unsplash.com/photo-1587202372634-32705e3bf49c?w=400',
		'Logitech': 'https://images.unsplash.com/photo-1612287230202-1ff1d85d1bdf?w=400',
		'Razer': 'https://images.unsplash.com/photo-1625948515291-69613efd103f?w=400',
		'Kingston': 'https://images.unsplash.com/photo-1597872200969-2b65d56bd16b?w=400',
		'Samsung': 'https://images.unsplash.com/photo-1551808525-51a94da548ce?w=400',
		'Western Digital': 'https://images.unsplash.com/photo-1597872200969-2b65d56bd16b?w=400',
		'Seagate': 'https://images.unsplash.com/photo-1597872200969-2b65d56bd16b?w=400'
	};
	
	async function cargarMarcas() {
		loading = true;
		error = null;
		
		try {
			marcas = await catalogoService.getMarcas();
			marcasFiltradas = marcas;
		} catch (err) {
			error = 'Error al cargar las marcas';
			console.error(err);
		} finally {
			loading = false;
		}
	}
	
	function filtrarMarcas() {
		if (!searchQuery.trim()) {
			marcasFiltradas = marcas;
		} else {
			marcasFiltradas = marcas.filter(marca =>
				marca.nombre.toLowerCase().includes(searchQuery.toLowerCase())
			);
		}
	}
	
	onMount(() => {
		cargarMarcas();
	});
	
	$: if (searchQuery !== undefined) {
		filtrarMarcas();
	}
</script>

<svelte:head>
	<title>Marcas - KronosTech</title>
	<meta name="description" content="Explora todas las marcas disponibles en KronosTech. Las mejores marcas de componentes de PC y gaming." />
</svelte:head>

<div class="w-full max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
	<!-- Breadcrumbs -->
	<nav class="flex mb-6 text-sm">
		<ol class="inline-flex items-center space-x-2">
			<li>
				<a href="/" class="text-slate-500 dark:text-slate-400 hover:text-primary">Inicio</a>
			</li>
			<li class="flex items-center">
				<span class="mx-2 text-slate-400">/</span>
				<span class="text-text-light dark:text-text-dark font-medium">Marcas</span>
			</li>
		</ol>
	</nav>
	
	<!-- Header -->
	<div class="mb-8">
		<div class="flex items-center gap-3 mb-3">
			<Package size={32} class="text-primary" />
			<h1 class="text-3xl md:text-4xl font-black tracking-tight">Todas las Marcas</h1>
		</div>
		<p class="text-slate-600 dark:text-slate-400 mb-6">
			Explora productos de las mejores marcas del mercado
		</p>
		
		<!-- Buscador -->
		<div class="max-w-md">
			<div class="relative">
				<div class="absolute inset-y-0 left-0 flex items-center pl-3 pointer-events-none">
					<Search size={20} class="text-slate-400" />
				</div>
				<input
					type="text"
					bind:value={searchQuery}
					placeholder="Buscar marca..."
					class="w-full pl-10 pr-4 py-3 border border-border-light dark:border-border-dark rounded-lg bg-surface-light dark:bg-surface-dark focus:ring-2 focus:ring-primary/50 focus:border-primary transition-colors"
				/>
			</div>
		</div>
	</div>
	
	<!-- Stats -->
	{#if !loading && marcas.length > 0}
		<div class="bg-primary/10 dark:bg-primary/20 rounded-lg p-4 mb-8 inline-block">
			<p class="text-sm font-semibold text-primary">
				{marcasFiltradas.length} marca{marcasFiltradas.length !== 1 ? 's' : ''} disponible{marcasFiltradas.length !== 1 ? 's' : ''}
			</p>
		</div>
	{/if}
	
	<!-- Grid de Marcas -->
	{#if loading}
		<div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-6">
			{#each Array(12) as _}
				<div class="aspect-square rounded-xl bg-slate-200 dark:bg-surface-dark animate-pulse"></div>
			{/each}
		</div>
	{:else if error}
		<div class="bg-red-100 dark:bg-red-900/20 text-red-700 dark:text-red-400 p-6 rounded-lg text-center">
			{error}
		</div>
	{:else if marcasFiltradas.length === 0}
		<div class="text-center py-16">
			<Search size={64} class="mx-auto text-slate-400 mb-4" />
			<h3 class="text-xl font-bold mb-2">No se encontraron marcas</h3>
			<p class="text-slate-600 dark:text-slate-400 mb-6">
				{#if searchQuery}
					No hay marcas que coincidan con "{searchQuery}"
				{:else}
					No hay marcas disponibles en este momento
				{/if}
			</p>
			{#if searchQuery}
				<button
					on:click={() => searchQuery = ''}
					class="bg-primary text-white font-bold py-3 px-6 rounded-lg hover:bg-primary/90 transition-colors"
				>
					Limpiar búsqueda
				</button>
			{/if}
		</div>
	{:else}
		<div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-6">
			{#each marcasFiltradas as marca}
				<a
					href="/catalogo?marca={marca.id_marca}"
					class="group relative bg-surface-light dark:bg-surface-dark border border-border-light dark:border-border-dark rounded-xl overflow-hidden hover:shadow-lg hover:scale-105 transition-all duration-300"
				>
					<!-- Imagen/Logo -->
					<div class="aspect-square bg-cover bg-center relative overflow-hidden"
						style="background-image: url('{marcasLogos[marca.nombre] || 'https://placehold.co/400/1e293b/94a3b8?text=' + encodeURIComponent(marca.nombre)}');"
					>
						<div class="absolute inset-0 bg-gradient-to-t from-black/80 via-black/40 to-transparent group-hover:from-black/90 transition-all"></div>
						
						<!-- Nombre de la marca -->
						<div class="absolute bottom-0 left-0 right-0 p-4">
							<p class="text-white font-bold text-center text-sm md:text-base leading-tight drop-shadow-lg">
								{marca.nombre}
							</p>
						</div>
						
						<!-- Badge de productos -->
						{#if marca.total_productos && marca.total_productos > 0}
							<div class="absolute top-2 right-2 bg-primary text-white text-xs font-bold px-2 py-1 rounded-full shadow-md">
								{marca.total_productos}
							</div>
						{/if}
					</div>
					
					<!-- Hover overlay -->
					<div class="absolute inset-0 bg-primary/0 group-hover:bg-primary/10 transition-colors pointer-events-none"></div>
				</a>
			{/each}
		</div>
	{/if}
	
	<!-- CTA Section -->
	{#if !loading && marcasFiltradas.length > 0}
		<div class="mt-16 bg-gradient-to-r from-primary/10 to-primary/5 dark:from-primary/20 dark:to-primary/10 rounded-xl p-8 text-center border border-primary/20">
			<h2 class="text-2xl font-bold mb-3">¿No encuentras tu marca favorita?</h2>
			<p class="text-slate-600 dark:text-slate-400 mb-6">
				Contáctanos y te ayudaremos a encontrar el producto que buscas
			</p>
			<a
				href="/catalogo"
				class="inline-block bg-primary text-white font-bold py-3 px-8 rounded-lg hover:bg-primary/90 transition-colors shadow-lg"
			>
				Ver Catálogo Completo
			</a>
		</div>
	{/if}
</div>
