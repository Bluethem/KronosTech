<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { authUser, isAuthenticated } from '$lib/stores/auth';

	// Verificar autenticación y rol de administrador
	onMount(() => {
		if (!$isAuthenticated) {
			goto('/login?redirect=/admin');
		} else if ($authUser && $authUser.rol !== 'administrador' && $authUser.rol !== 'super_admin') {
			// Si no es admin, redirigir a cuenta o home
			goto('/');
		}
	});
</script>

<slot />
