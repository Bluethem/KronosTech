<script lang="ts">
  import { goto } from '$app/navigation';
  import { login } from '$lib/services/auth';
  import { setAuthLoading, setUser } from '$lib/stores/auth';
  import { cartService } from '$lib/services/cart';
  import { Mail, Lock, Eye, EyeOff } from 'lucide-svelte';

  let email = '';
  let password = '';
  let remember = false;
  let showPassword = false;

  let loading = false;
  let apiError = '';
  let fieldErrors: { email?: string; password?: string } = {};

  const emailRegex = /^[^@\s]+@[^@\s]+\.[^@\s]+$/;

  function validate() {
    fieldErrors = {};

    if (!email) {
      fieldErrors.email = 'El correo es obligatorio.';
    } else if (!emailRegex.test(email)) {
      fieldErrors.email = 'Ingresa un correo válido.';
    }

    if (!password) {
      fieldErrors.password = 'La contraseña es obligatoria.';
    }

    return Object.keys(fieldErrors).length === 0;
  }

  async function handleSubmit() {
    apiError = '';
    if (!validate()) return;

    loading = true;
    setAuthLoading(true);

    try {
      const response = await login({ email, password, remember_me: remember });
      setUser(response.data.usuario);
      // Inicializar carrito después del login
      await cartService.initCart();
      await goto('/catalogo');
    } catch (err: any) {
      apiError = err.message || 'No se pudo iniciar sesión. Inténtalo nuevamente.';
    } finally {
      loading = false;
      setAuthLoading(false);
    }
  }
</script>

<svelte:head>
  <title>Iniciar sesión | KronosTech</title>
</svelte:head>

<div class="min-h-screen bg-surface-light dark:bg-surface-dark flex items-center justify-center px-4">
  <div class="w-full max-w-md">
    <div class="bg-surface-light dark:bg-surface-dark border border-border-light dark:border-border-dark rounded-2xl shadow-xl px-8 py-10">
      <div class="text-center mb-6">
        <h1 class="text-2xl font-semibold text-text-light dark:text-text-dark">
          Iniciar sesión
        </h1>
        <p class="mt-1 text-sm text-slate-600 dark:text-slate-300">
          Ingresa con tu cuenta de KronosTech
        </p>
      </div>

      {#if apiError}
        <div class="mb-4 rounded-lg border border-red-500 bg-red-50 text-red-700 text-sm px-4 py-3 dark:bg-red-500/10 dark:text-red-200">
          {apiError}
        </div>
      {/if}

      <form class="space-y-5" on:submit|preventDefault={handleSubmit}>
        <div>
          <label class="block text-sm font-medium text-text-light dark:text-text-dark mb-1">
            Correo electrónico *
          </label>
          <div class="relative">
            <span class="absolute inset-y-0 left-3 flex items-center text-slate-500">
              <Mail class="h-4 w-4" />
            </span>
            <input
              type="email"
              class={`form-input w-full pl-10 ${fieldErrors.email ? 'border-red-500' : ''}`}
              bind:value={email}
              placeholder="usuario@correo.com"
            />
          </div>
          {#if fieldErrors.email}
            <p class="mt-1 text-xs text-red-500">{fieldErrors.email}</p>
          {/if}
        </div>

        <div>
          <label class="block text-sm font-medium text-text-light dark:text-text-dark mb-1">
            Contraseña *
          </label>
          <div class="relative">
            <span class="absolute inset-y-0 left-3 flex items-center text-slate-500">
              <Lock class="h-4 w-4" />
            </span>
            <input
              type={showPassword ? 'text' : 'password'}
              class={`form-input w-full pl-10 pr-10 ${fieldErrors.password ? 'border-red-500' : ''}`}
              bind:value={password}
              placeholder="Ingresa tu contraseña"
            />
            <button
              type="button"
              class="absolute inset-y-0 right-3 flex items-center text-slate-500 hover:text-slate-700 dark:hover:text-slate-200 text-xs"
              on:click={() => (showPassword = !showPassword)}
            >
              {#if showPassword}
                <EyeOff class="h-4 w-4" />
              {:else}
                <Eye class="h-4 w-4" />
              {/if}
            </button>
          </div>
          {#if fieldErrors.password}
            <p class="mt-1 text-xs text-red-500">{fieldErrors.password}</p>
          {/if}
        </div>

        <div class="flex items-center justify-between text-sm">
          <label class="inline-flex items-center gap-2 text-slate-700 dark:text-slate-300">
            <input
              type="checkbox"
              class="h-4 w-4 rounded border-border-light dark:border-border-dark"
              bind:checked={remember}
            />
            <span>Mantener sesión iniciada</span>
          </label>

          <a href="/olvide-contrasena" class="text-primary hover:underline">
            ¿Olvidaste tu contraseña?
          </a>
        </div>

        <button
          type="submit"
          class="w-full bg-primary text-white hover:bg-primary/90 rounded-lg py-2.5 text-sm font-semibold disabled:opacity-60 disabled:cursor-not-allowed"
          disabled={loading}
        >
          {#if loading}Procesando...{:else}Ingresar{/if}
        </button>
      </form>

      <div class="mt-6 text-center text-sm text-slate-700 dark:text-slate-300">
        ¿No tienes cuenta?
        <a href="/registro" class="text-primary font-medium hover:underline">
          Crear cuenta
        </a>
      </div>
    </div>
  </div>
</div>
