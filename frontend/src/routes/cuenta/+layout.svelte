<script lang="ts">
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { authUser, isAuthenticated, logout as logoutUser } from '$lib/stores/auth';
	import {
		LayoutDashboard,
		User,
		Package,
		MapPin,
		CreditCard,
		Heart,
		Star,
		Bell,
		Shield,
		LogOut,
		ChevronRight,
		Menu,
		X
	} from 'lucide-svelte';

	let mobileMenuOpen = false;

	// Verificar autenticación
	onMount(() => {
		if (!$isAuthenticated) {
			goto('/login?redirect=/cuenta');
		} else if ($authUser && ($authUser.rol === 'administrador' || $authUser.rol === 'super_admin')) {
			// Si es admin, redirigir a panel de administración
			goto('/admin');
		}
	});

	$: currentPath = $page.url.pathname;

	import { RotateCcw } from 'lucide-svelte';

	const menuItems = [
		{ path: '/cuenta', label: 'Panel General', icon: LayoutDashboard },
		{ path: '/cuenta/perfil', label: 'Mi Perfil', icon: User },
		{ path: '/cuenta/pedidos', label: 'Mis Pedidos', icon: Package },
		{ path: '/cuenta/direcciones', label: 'Mis Direcciones', icon: MapPin },
		{ path: '/cuenta/tarjetas', label: 'Mis Tarjetas', icon: CreditCard },
		{ path: '/cuenta/cupones', label: 'Mis Cupones', icon: CreditCard },
		{ path: '/cuenta/reembolsos', label: 'Mis Reembolsos', icon: RotateCcw },
		{ path: '/cuenta/seguridad', label: 'Seguridad', icon: Shield }
	];

	function isActive(path: string): boolean {
		if (path === '/cuenta') {
			return currentPath === path;
		}
		return currentPath.startsWith(path);
	}

	function toggleMobileMenu() {
		mobileMenuOpen = !mobileMenuOpen;
	}

	async function handleLogout() {
		if (confirm('¿Estás seguro de que deseas cerrar sesión?')) {
			await logoutUser();
		}
	}
</script>

<div class="min-h-[calc(100vh-4rem)] bg-surface-light dark:bg-surface-dark">
	<div class="max-w-7xl mx-auto px-4 lg:px-6 py-8">

		<!-- Header Mobile -->
		<div class="lg:hidden mb-6 flex items-center justify-between">
			<div>
				<h1 class="text-2xl font-bold text-text-light dark:text-text-dark">
					Mi Cuenta
				</h1>
				<p class="text-sm text-slate-600 dark:text-slate-400 mt-1">
					{$authUser?.nombre} {$authUser?.apellido}
				</p>
			</div>
			<button
				type="button"
				class="p-2 rounded-lg border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800 hover:bg-slate-100 dark:hover:bg-slate-700 transition-colors"
				on:click={toggleMobileMenu}
				aria-label="Abrir menú"
			>
				{#if mobileMenuOpen}
					<X size={24} />
				{:else}
					<Menu size={24} />
				{/if}
			</button>
		</div>

		<!-- Grid Principal -->
		<div class="grid grid-cols-1 lg:grid-cols-[280px,1fr] gap-6">

			<!-- Menú Lateral -->
			<aside
				class="lg:block {mobileMenuOpen ? 'block' : 'hidden'} space-y-4"
				aria-label="Menú de cuenta"
			>
				<!-- Info Usuario -->
				<div class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 p-5 shadow-sm">
					<div class="flex items-center gap-3">
						<div class="w-12 h-12 rounded-full bg-primary/10 dark:bg-primary/20 flex items-center justify-center text-primary font-bold text-lg">
							{$authUser?.nombre?.charAt(0)}{$authUser?.apellido?.charAt(0)}
						</div>
						<div class="flex-1 min-w-0">
							<p class="font-semibold text-text-light dark:text-text-dark truncate">
								{$authUser?.nombre} {$authUser?.apellido}
							</p>
							<p class="text-xs text-slate-600 dark:text-slate-400 truncate">
								{$authUser?.email}
							</p>
						</div>
					</div>
				</div>

				<!-- Navegación -->
				<nav class="rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 shadow-sm overflow-hidden">
					<ul class="divide-y divide-border-light dark:divide-border-dark">
						{#each menuItems as item}
							<li>
								<a
									href={item.path}
									class="flex items-center gap-3 px-4 py-3 text-sm transition-colors {isActive(item.path)
										? 'bg-primary/10 dark:bg-primary/20 text-primary font-semibold border-l-4 border-primary'
										: 'text-slate-700 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-700/50 border-l-4 border-transparent'}"
									on:click={() => mobileMenuOpen = false}
								>
									<svelte:component this={item.icon} size={20} />
									<span class="flex-1">{item.label}</span>
									{#if isActive(item.path)}
										<ChevronRight size={16} />
									{/if}
								</a>
							</li>
						{/each}
					</ul>
				</nav>

				<!-- Cerrar Sesión -->
				<button
					type="button"
					class="w-full rounded-xl border border-border-light dark:border-border-dark bg-surface-light dark:bg-slate-800/50 px-4 py-3 text-sm text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/20 transition-colors flex items-center gap-3 shadow-sm"
					on:click={handleLogout}
				>
					<LogOut size={20} />
					<span class="font-medium">Cerrar Sesión</span>
				</button>
			</aside>

			<!-- Contenido Principal -->
			<main class="min-w-0 space-y-6 p-2 sm:p-4 lg:p-6">
				<slot />
			</main>
		</div>
	</div>
</div>
