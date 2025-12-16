<script lang="ts">
	import { onMount } from 'svelte';
	import favicon from '$lib/assets/favicon.svg';
	import Header from '$lib/components/Header.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import ScrollToTop from '$lib/components/ScrollToTop.svelte';
	import { initAuth, authLoading } from '$lib/stores/auth';
	import '../app.css';

	let { children } = $props();
	let initialized = $state(false);

	onMount(async () => {
		// Inicializar autenticación al cargar la app
		await initAuth();
		initialized = true;
	});
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
	<link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;700;800&display=swap" rel="stylesheet" />
</svelte:head>

<div class="relative flex h-auto min-h-screen w-full flex-col">
	<Header />
	<main class="flex-1">
		{#if !initialized || $authLoading}
			<!-- Loading state mientras se inicializa la autenticación -->
			<div class="flex items-center justify-center min-h-[60vh]">
				<div class="text-center">
					<div class="animate-spin rounded-full h-12 w-12 border-b-2 border-primary mx-auto mb-4"></div>
					<p class="text-slate-600 dark:text-slate-400">Cargando...</p>
				</div>
			</div>
		{:else}
			{@render children()}
		{/if}
	</main>
	<Footer />
	<ScrollToTop />
</div>
