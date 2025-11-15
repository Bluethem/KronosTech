<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { ChevronUp } from 'lucide-svelte';
	
	let visible = false;
	
	function handleScroll() {
		if (browser) {
			visible = window.scrollY > 300;
		}
	}
	
	function scrollToTop() {
		if (browser) {
			window.scrollTo({
				top: 0,
				behavior: 'smooth'
			});
		}
	}
	
	onMount(() => {
		handleScroll();
		window.addEventListener('scroll', handleScroll);
		return () => window.removeEventListener('scroll', handleScroll);
	});
</script>

{#if visible}
	<button
		on:click={scrollToTop}
		aria-label="Volver arriba"
		class="fixed bottom-24 right-6 z-40 p-3 bg-primary hover:bg-primary/90 text-white rounded-full shadow-lg hover:shadow-xl transition-all duration-300 hover:scale-110"
	>
		<ChevronUp size={24} />
	</button>
{/if}
