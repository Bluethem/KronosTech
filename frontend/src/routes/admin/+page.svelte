<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { authUser, isAuthenticated } from '$lib/stores/auth';
	import {
		LayoutDashboard,
		Package,
		ShoppingCart,
		Users,
		TrendingUp,
		Percent,
		Tag,
		RotateCcw,
		FileText,
		BarChart3,
		ArrowRight,
		PackageSearch,
		PackagePlus
	} from 'lucide-svelte';

	$: user = $authUser;

	// Verificar autenticación y rol
	onMount(() => {
		if (!$isAuthenticated) {
			goto('/login?redirect=/admin');
		} else if ($authUser && $authUser.rol !== 'administrador' && $authUser.rol !== 'super_admin') {
			// Si no es admin, redirigir a cuenta
			goto('/cuenta');
		}
	});

	// Datos de ejemplo - estos deberían venir del backend
	let stats = {
		ventasHoy: 0,
		pedidosPendientes: 0,
		productosBajoStock: 0,
		usuariosActivos: 0
	};

	let loading = true;

	onMount(async () => {
		// TODO: Cargar estadísticas reales del backend
		await loadDashboardData();
	});

	async function loadDashboardData() {
		loading = true;
		try {
			// Simular carga de datos
			await new Promise(resolve => setTimeout(resolve, 500));

			// Datos de ejemplo
			stats = {
				ventasHoy: 25,
				pedidosPendientes: 8,
				productosBajoStock: 12,
				usuariosActivos: 156
			};
		} catch (error) {
			console.error('Error al cargar dashboard:', error);
		} finally {
			loading = false;
		}
	}

	// Secciones de gestión según rol
	$: sections = user?.rol === 'super_admin'
		? [
				// SUPER ADMIN: Solo gestión de usuarios y sistema
				{
					title: 'Gestión de Usuarios',
					description: 'Administra usuarios y sus permisos',
					icon: Users,
					href: '/admin/usuarios',
					color: 'purple'
				},
				{
					title: 'Administradores',
					description: 'Habilita o deshabilita administradores',
					icon: Users,
					href: '/admin/administradores',
					color: 'blue'
				},
				{
					title: 'Configuración del Sistema',
					description: 'Configura parámetros globales',
					icon: Package,
					href: '/admin/configuracion',
					color: 'indigo'
				},
				{
					title: 'Logs y Auditoría',
					description: 'Revisa la actividad del sistema',
					icon: FileText,
					href: '/admin/logs',
					color: 'cyan'
				}
		  ]
		: [
				// ADMINISTRADOR: Operaciones del día a día
				{
					title: 'Gestión de Pedidos',
					description: 'Administra y procesa los pedidos de clientes',
					icon: ShoppingCart,
					href: '/gestion-pedidos',
					color: 'blue'
				},
				{
					title: 'Inventario',
					description: 'Controla el stock y movimientos de inventario',
					icon: PackageSearch,
					href: '/inventario',
					color: 'purple'
				},
				{
					title: 'Descuentos',
					description: 'Configura descuentos y promociones',
					icon: Percent,
					href: '/gestion-descuentos',
					color: 'amber'
				},
				{
					title: 'Cupones',
					description: 'Gestiona cupones de descuento',
					icon: Tag,
					href: '/gestion-cupones',
					color: 'pink'
				},
				{
					title: 'Reembolsos',
					description: 'Procesa solicitudes de reembolso',
					icon: RotateCcw,
					href: '/gestion-reembolsos',
					color: 'orange'
				},
				{
					title: 'Reportes de Ventas',
					description: 'Analiza el rendimiento de ventas',
					icon: BarChart3,
					href: '/reportes-ventas',
					color: 'indigo'
				},
				{
					title: 'Reportes de Inventario',
					description: 'Revisa reportes de inventario',
					icon: FileText,
					href: '/reportes-inventario',
					color: 'cyan'
				}
		  ];

	function getColorClasses(color: string) {
		const colorMap: Record<string, { bg: string; text: string; hover: string }> = {
			blue: {
				bg: 'bg-blue-500/10 dark:bg-blue-500/20',
				text: 'text-blue-600 dark:text-blue-400',
				hover: 'hover:border-blue-500/50'
			},
			emerald: {
				bg: 'bg-emerald-500/10 dark:bg-emerald-500/20',
				text: 'text-emerald-600 dark:text-emerald-400',
				hover: 'hover:border-emerald-500/50'
			},
			purple: {
				bg: 'bg-purple-500/10 dark:bg-purple-500/20',
				text: 'text-purple-600 dark:text-purple-400',
				hover: 'hover:border-purple-500/50'
			},
			amber: {
				bg: 'bg-amber-500/10 dark:bg-amber-500/20',
				text: 'text-amber-600 dark:text-amber-400',
				hover: 'hover:border-amber-500/50'
			},
			pink: {
				bg: 'bg-pink-500/10 dark:bg-pink-500/20',
				text: 'text-pink-600 dark:text-pink-400',
				hover: 'hover:border-pink-500/50'
			},
			orange: {
				bg: 'bg-orange-500/10 dark:bg-orange-500/20',
				text: 'text-orange-600 dark:text-orange-400',
				hover: 'hover:border-orange-500/50'
			},
			indigo: {
				bg: 'bg-indigo-500/10 dark:bg-indigo-500/20',
				text: 'text-indigo-600 dark:text-indigo-400',
				hover: 'hover:border-indigo-500/50'
			},
			cyan: {
				bg: 'bg-cyan-500/10 dark:bg-cyan-500/20',
				text: 'text-cyan-600 dark:text-cyan-400',
				hover: 'hover:border-cyan-500/50'
			}
		};
		return colorMap[color] || colorMap.blue;
	}
</script>

<svelte:head>
	<title>Panel de Administración | KronosTech</title>
</svelte:head>

<div class="min-h-[calc(100vh-4rem)] bg-surface-light dark:bg-surface-dark">
	<div class="max-w-7xl mx-auto px-4 lg:px-6 py-8">
		<!-- Header -->
		<div class="mb-8">
			<div class="flex items-center justify-between flex-wrap gap-4">
				<div class="flex items-center gap-3">
					<div class="w-12 h-12 rounded-xl bg-primary/10 dark:bg-primary/20 flex items-center justify-center">
						<LayoutDashboard size={28} class="text-primary" />
					</div>
					<div>
						<h1 class="text-3xl font-bold tracking-tight text-text-light dark:text-text-dark">
							Panel de Administración
						</h1>
						<p class="text-sm text-slate-600 dark:text-slate-400 mt-1">
							Bienvenido, {user?.nombre} {user?.apellido}
						</p>
					</div>
				</div>
				<!-- Badge de Rol -->
				<div class="flex items-center gap-2">
					{#if user?.rol === 'super_admin'}
						<span class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-full text-xs font-semibold bg-gradient-to-r from-purple-500/10 to-pink-500/10 dark:from-purple-500/20 dark:to-pink-500/20 text-purple-700 dark:text-purple-300 border border-purple-500/30 dark:border-purple-400/30">
							<svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
								<path d="M12 2L2 7l10 5 10-5-10-5z"></path>
								<path d="M2 17l10 5 10-5"></path>
								<path d="M2 12l10 5 10-5"></path>
							</svg>
							Super Administrador
						</span>
					{:else if user?.rol === 'administrador'}
						<span class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-full text-xs font-semibold bg-blue-500/10 dark:bg-blue-500/20 text-blue-700 dark:text-blue-300 border border-blue-500/30 dark:border-blue-400/30">
							<svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
								<path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"></path>
								<circle cx="9" cy="7" r="4"></circle>
								<path d="M22 21v-2a4 4 0 0 0-3-3.87"></path>
								<path d="M16 3.13a4 4 0 0 1 0 7.75"></path>
							</svg>
							Administrador
						</span>
					{/if}
				</div>
			</div>
		</div>

		{#if loading}
			<!-- Loading State -->
			<div class="flex items-center justify-center py-20">
				<div class="text-center">
					<div class="animate-spin rounded-full h-12 w-12 border-b-2 border-primary mx-auto mb-4"></div>
					<p class="text-slate-600 dark:text-slate-400">Cargando información...</p>
				</div>
			</div>
		{:else}
			<!-- Estadísticas Rápidas -->
			<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
				<!-- Ventas Hoy -->
				<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 p-5 shadow-sm">
					<div class="flex items-center justify-between mb-3">
						<div class="w-12 h-12 rounded-lg bg-green-500/10 dark:bg-green-500/20 flex items-center justify-center text-green-600 dark:text-green-400">
							<TrendingUp size={24} />
						</div>
					</div>
					<div>
						<p class="text-2xl font-bold text-text-light dark:text-text-dark">
							{stats.ventasHoy}
						</p>
						<p class="text-sm text-slate-600 dark:text-slate-400">
							Ventas hoy
						</p>
					</div>
				</div>

				<!-- Pedidos Pendientes -->
				<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 p-5 shadow-sm">
					<div class="flex items-center justify-between mb-3">
						<div class="w-12 h-12 rounded-lg bg-amber-500/10 dark:bg-amber-500/20 flex items-center justify-center text-amber-600 dark:text-amber-400">
							<ShoppingCart size={24} />
						</div>
					</div>
					<div>
						<p class="text-2xl font-bold text-text-light dark:text-text-dark">
							{stats.pedidosPendientes}
						</p>
						<p class="text-sm text-slate-600 dark:text-slate-400">
							Pedidos pendientes
						</p>
					</div>
				</div>

				<!-- Productos Bajo Stock -->
				<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 p-5 shadow-sm">
					<div class="flex items-center justify-between mb-3">
						<div class="w-12 h-12 rounded-lg bg-red-500/10 dark:bg-red-500/20 flex items-center justify-center text-red-600 dark:text-red-400">
							<Package size={24} />
						</div>
					</div>
					<div>
						<p class="text-2xl font-bold text-text-light dark:text-text-dark">
							{stats.productosBajoStock}
						</p>
						<p class="text-sm text-slate-600 dark:text-slate-400">
							Productos bajo stock
						</p>
					</div>
				</div>

				<!-- Usuarios Activos -->
				<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 p-5 shadow-sm">
					<div class="flex items-center justify-between mb-3">
						<div class="w-12 h-12 rounded-lg bg-blue-500/10 dark:bg-blue-500/20 flex items-center justify-center text-blue-600 dark:text-blue-400">
							<Users size={24} />
						</div>
					</div>
					<div>
						<p class="text-2xl font-bold text-text-light dark:text-text-dark">
							{stats.usuariosActivos}
						</p>
						<p class="text-sm text-slate-600 dark:text-slate-400">
							Usuarios activos
						</p>
					</div>
				</div>
			</div>

			<!-- Secciones de Gestión -->
			<div>
				<h2 class="text-xl font-semibold text-text-light dark:text-text-dark mb-4">
					Accesos Rápidos
				</h2>
				<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
					{#each sections as section}
						{@const colors = getColorClasses(section.color)}
						<a
							href={section.href}
							class="group rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 p-5 shadow-sm hover:shadow-md {colors.hover} dark:{colors.hover} transition-all"
						>
							<div class="flex items-center justify-between mb-3">
								<div class="w-12 h-12 rounded-lg {colors.bg} flex items-center justify-center {colors.text} group-hover:scale-110 transition-transform">
									<svelte:component this={section.icon} size={24} />
								</div>
								<ArrowRight size={20} class="text-slate-400 group-hover:text-primary group-hover:translate-x-1 transition-all" />
							</div>
							<div>
								<p class="font-semibold text-text-light dark:text-text-dark mb-1">
									{section.title}
								</p>
								<p class="text-xs text-slate-600 dark:text-slate-400">
									{section.description}
								</p>
							</div>
						</a>
					{/each}
				</div>
			</div>
		{/if}
	</div>
</div>
