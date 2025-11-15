<script lang="ts">
	import favicon from '$lib/assets/favicon.svg';
	import Header from '$lib/components/Header.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import ScrollToTop from '$lib/components/ScrollToTop.svelte';
	import '../app.css';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
	<link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;700;800&display=swap" rel="stylesheet" />
</svelte:head>

<div class="relative flex h-auto min-h-screen w-full flex-col bg-background-light dark:bg-background-dark text-text-light dark:text-text-dark">
	<Header />
	<main class="flex-1">
		{@render children()}
	</main>
	<Footer />
	<ScrollToTop />
</div>
