<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { authUser } from '$lib/stores/auth';
	import { Package, MapPin, CreditCard, Heart, ArrowRight, TrendingUp, ShoppingCart } from 'lucide-svelte';
	import { checkoutService, type Pedido } from '$lib/services/checkout';
	import { direccionService } from '$lib/services/direccion';
	import { tarjetaService } from '$lib/services/tarjeta';

	$: user = $authUser;

	// Datos del dashboard - se cargan desde la BD
	let stats = {
		totalPedidos: 0,
		pedidosPendientes: 0,
		direccionesGuardadas: 0,
		tarjetasGuardadas: 0
	};

	let recentOrders: Pedido[] = [];
	let loading = true;

	onMount(async () => {
		await loadDashboardData();
	});

	async function loadDashboardData() {
		loading = true;
		try {
			// Cargar datos en paralelo desde la BD
			const [pedidos, direcciones, tarjetas] = await Promise.all([
				checkoutService.getPedidos().catch(() => []),
				direccionService.getDirecciones().catch(() => []),
				tarjetaService.getMetodosPago().catch(() => [])
			]);

			// Calcular estadísticas
			const pedidosPendientes = pedidos.filter(p => 
				p.estado_pedido === 'pendiente' || 
				p.estado_pedido === 'confirmado' || 
				p.estado_pedido === 'preparando' ||
				p.estado_pedido === 'enviado'
			);

			stats = {
				totalPedidos: pedidos.length,
				pedidosPendientes: pedidosPendientes.length,
				direccionesGuardadas: direcciones.length,
				tarjetasGuardadas: tarjetas.length
			};

			// Últimos 3 pedidos
			recentOrders = pedidos.slice(0, 3);
		} catch (error) {
			console.error('Error al cargar dashboard:', error);
		} finally {
			loading = false;
		}
	}

	function formatDate(dateString: string): string {
		const date = new Date(dateString);
		return date.toLocaleDateString('es-PE', {
			day: '2-digit',
			month: 'short',
			year: 'numeric'
		});
	}

	function formatPrice(price: number): string {
		return new Intl.NumberFormat('es-PE', {
			style: 'currency',
			currency: 'PEN'
		}).format(price);
	}

	function getEstadoBadgeClass(estado: string): string {
		const classes: Record<string, string> = {
			pendiente: 'bg-yellow-100 dark:bg-yellow-900/40 text-yellow-700 dark:text-yellow-300',
			confirmado: 'bg-blue-100 dark:bg-blue-900/40 text-blue-700 dark:text-blue-300',
			preparando: 'bg-purple-100 dark:bg-purple-900/40 text-purple-700 dark:text-purple-300',
			enviado: 'bg-indigo-100 dark:bg-indigo-900/40 text-indigo-700 dark:text-indigo-300',
			entregado: 'bg-green-100 dark:bg-green-900/40 text-green-700 dark:text-green-300',
			cancelado: 'bg-red-100 dark:bg-red-900/40 text-red-700 dark:text-red-300'
		};
		return classes[estado] || classes.pendiente;
	}

	function getEstadoLabel(estado: string): string {
		const labels: Record<string, string> = {
			pendiente: 'Pendiente',
			confirmado: 'Confirmado',
			preparando: 'Preparando',
			enviado: 'Enviado',
			entregado: 'Entregado',
			cancelado: 'Cancelado'
		};
		return labels[estado] || estado;
	}
</script>

<svelte:head>
	<title>Mi Cuenta | KronosTech</title>
</svelte:head>

<div class="space-y-6">
	<!-- Header -->
	<div>
		<h1 class="text-3xl font-bold tracking-tight text-text-light dark:text-text-dark">
			¡Hola, {user?.nombre}!
		</h1>
		<p class="text-sm text-slate-600 dark:text-slate-400 mt-1">
			Bienvenido a tu panel de control personal
		</p>
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
		<!-- Stats Grid -->
		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
			<!-- Total Pedidos -->
			<a
				href="/cuenta/pedidos"
				class="group rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 p-5 shadow-sm hover:shadow-md hover:border-primary/50 dark:hover:border-primary/50 transition-all"
			>
				<div class="flex items-center justify-between mb-3">
					<div class="w-12 h-12 rounded-lg bg-blue-500/10 dark:bg-blue-500/20 flex items-center justify-center text-blue-600 dark:text-blue-400 group-hover:scale-110 transition-transform">
						<Package size={24} />
					</div>
					<ArrowRight size={20} class="text-slate-400 group-hover:text-primary group-hover:translate-x-1 transition-all" />
				</div>
				<div>
					<p class="text-2xl font-bold text-text-light dark:text-text-dark">
						{stats.totalPedidos}
					</p>
					<p class="text-sm text-slate-600 dark:text-slate-400">
						Pedidos realizados
					</p>
					{#if stats.pedidosPendientes > 0}
						<p class="text-xs text-amber-600 dark:text-amber-400 mt-1">
							{stats.pedidosPendientes} en proceso
						</p>
					{/if}
				</div>
			</a>

			<!-- Direcciones -->
			<a
				href="/cuenta/direcciones"
				class="group rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 p-5 shadow-sm hover:shadow-md hover:border-primary/50 dark:hover:border-primary/50 transition-all"
			>
				<div class="flex items-center justify-between mb-3">
					<div class="w-12 h-12 rounded-lg bg-emerald-500/10 dark:bg-emerald-500/20 flex items-center justify-center text-emerald-600 dark:text-emerald-400 group-hover:scale-110 transition-transform">
						<MapPin size={24} />
					</div>
					<ArrowRight size={20} class="text-slate-400 group-hover:text-primary group-hover:translate-x-1 transition-all" />
				</div>
				<div>
					<p class="text-2xl font-bold text-text-light dark:text-text-dark">
						{stats.direccionesGuardadas}
					</p>
					<p class="text-sm text-slate-600 dark:text-slate-400">
						Direcciones guardadas
					</p>
				</div>
			</a>

			<!-- Tarjetas -->
			<a
				href="/cuenta/tarjetas"
				class="group rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 p-5 shadow-sm hover:shadow-md hover:border-primary/50 dark:hover:border-primary/50 transition-all"
			>
				<div class="flex items-center justify-between mb-3">
					<div class="w-12 h-12 rounded-lg bg-purple-500/10 dark:bg-purple-500/20 flex items-center justify-center text-purple-600 dark:text-purple-400 group-hover:scale-110 transition-transform">
						<CreditCard size={24} />
					</div>
					<ArrowRight size={20} class="text-slate-400 group-hover:text-primary group-hover:translate-x-1 transition-all" />
				</div>
				<div>
					<p class="text-2xl font-bold text-text-light dark:text-text-dark">
						{stats.tarjetasGuardadas}
					</p>
					<p class="text-sm text-slate-600 dark:text-slate-400">
						Métodos de pago
					</p>
				</div>
			</a>

			<!-- Continuar comprando -->
			<a
				href="/catalogo"
				class="group rounded-xl border border-border-light dark:border-border-dark bg-gradient-to-br from-primary/10 to-primary/5 dark:from-primary/20 dark:to-primary/10 p-5 shadow-sm hover:shadow-md hover:border-primary dark:hover:border-primary transition-all"
			>
				<div class="flex items-center justify-between mb-3">
					<div class="w-12 h-12 rounded-lg bg-primary/20 dark:bg-primary/30 flex items-center justify-center text-primary group-hover:scale-110 transition-transform">
						<ShoppingCart size={24} />
					</div>
					<ArrowRight size={20} class="text-primary group-hover:translate-x-1 transition-all" />
				</div>
				<div>
					<p class="text-lg font-bold text-text-light dark:text-text-dark">
						Ir al catálogo
					</p>
					<p class="text-sm text-slate-600 dark:text-slate-400">
						Descubre productos
					</p>
				</div>
			</a>
		</div>

		<!-- Pedidos Recientes -->
		<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 shadow-sm overflow-hidden">
			<div class="px-6 py-4 border-b border-border-light dark:border-border-dark flex items-center justify-between">
				<div>
					<h2 class="text-lg font-semibold text-text-light dark:text-text-dark">
						Pedidos Recientes
					</h2>
					<p class="text-xs text-slate-600 dark:text-slate-400 mt-0.5">
						Tus últimas compras
					</p>
				</div>
				<a
					href="/cuenta/pedidos"
					class="text-sm font-medium text-primary hover:text-primary/80 transition-colors flex items-center gap-1"
				>
					Ver todos
					<ArrowRight size={16} />
				</a>
			</div>

			<div class="p-6">
				{#if recentOrders.length === 0}
					<div class="text-center py-12">
						<div class="w-16 h-16 rounded-xl border border-border-light dark:border-border-dark bg-slate-100 dark:bg-slate-800 flex items-center justify-center mx-auto mb-4">
							<Package size={32} class="text-slate-400 dark:text-slate-500" />
						</div>
						<h3 class="text-lg font-semibold text-text-light dark:text-text-dark mb-2">
							No tienes pedidos recientes
						</h3>
						<p class="text-sm text-slate-600 dark:text-slate-400 mb-4 max-w-md mx-auto">
							Explora nuestro catálogo y realiza tu primera compra
						</p>
						<a
							href="/catalogo"
							class="inline-flex items-center gap-2 px-5 py-2.5 rounded-lg text-sm font-semibold bg-primary text-white hover:bg-primary/90 transition-colors"
						>
							<ShoppingCart size={16} />
							Ir al catálogo
						</a>
					</div>
				{:else}
					<!-- Lista de pedidos -->
					<div class="space-y-3">
						{#each recentOrders as order (order.id_venta)}
							<a
								href="/pedido/{order.id_venta}"
								class="block rounded-xl border border-border-light dark:border-border-dark p-4 hover:bg-slate-50 dark:hover:bg-slate-700/30 transition-colors group"
							>
								<div class="flex items-center justify-between">
									<div>
										<p class="font-semibold text-text-light dark:text-text-dark group-hover:text-primary transition-colors">
											{order.numero_pedido}
										</p>
										<p class="text-sm text-slate-600 dark:text-slate-400">
											{formatDate(order.fecha_pedido)}
										</p>
									</div>
									<div class="text-right">
										<p class="font-bold text-primary text-lg">
											{formatPrice(order.total)}
										</p>
										<span class="text-xs px-2.5 py-1 rounded-full {getEstadoBadgeClass(order.estado_pedido)}">
											{getEstadoLabel(order.estado_pedido)}
										</span>
									</div>
								</div>
							</a>
						{/each}
					</div>
				{/if}
			</div>
		</div>

		<!-- Accesos Rápidos -->
		<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
			<!-- Perfil -->
			<a
				href="/cuenta/perfil"
				class="group rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 p-5 shadow-sm hover:shadow-md hover:border-primary/50 dark:hover:border-primary/50 transition-all flex items-center justify-between"
			>
				<div class="flex items-center gap-4">
					<div class="w-12 h-12 rounded-lg bg-slate-100 dark:bg-slate-700 flex items-center justify-center group-hover:scale-110 transition-transform">
						<span class="text-2xl font-bold text-primary">
							{user?.nombre?.charAt(0)}{user?.apellido?.charAt(0)}
						</span>
					</div>
					<div>
						<p class="font-semibold text-text-light dark:text-text-dark">
							Ver mi perfil
						</p>
						<p class="text-sm text-slate-600 dark:text-slate-400">
							Información personal
						</p>
					</div>
				</div>
				<ArrowRight size={20} class="text-slate-400 group-hover:text-primary group-hover:translate-x-1 transition-all" />
			</a>

			<!-- Seguridad -->
			<a
				href="/cuenta/seguridad"
				class="group rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 p-5 shadow-sm hover:shadow-md hover:border-primary/50 dark:hover:border-primary/50 transition-all flex items-center justify-between"
			>
				<div class="flex items-center gap-4">
					<div class="w-12 h-12 rounded-lg bg-slate-100 dark:bg-slate-700 flex items-center justify-center text-slate-600 dark:text-slate-300 group-hover:scale-110 transition-transform">
						🔒
					</div>
					<div>
						<p class="font-semibold text-text-light dark:text-text-dark">
							Seguridad
						</p>
						<p class="text-sm text-slate-600 dark:text-slate-400">
							Contraseña y privacidad
						</p>
					</div>
				</div>
				<ArrowRight size={20} class="text-slate-400 group-hover:text-primary group-hover:translate-x-1 transition-all" />
			</a>
		</div>
	{/if}
</div>
