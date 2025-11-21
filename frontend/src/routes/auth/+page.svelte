<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { authService } from '$lib/services/auth';
  import { setAuth } from '$lib/stores/auth';
  import type { AxiosError } from 'axios';

  type LoginForm = {
    email: string;
    password: string;
    remember: boolean;
  };

  type RegisterForm = {
    firstName: string;
    lastName: string;
    email: string;
    phone: string;
    password: string;
    confirmPassword: string;
    acceptTerms: boolean;
  };

  type ApiErrorResponse = {
    message?: string;
    error?: string;
    errors?: string[];
  };

  let activeTab: 'login' | 'register' = 'login';

  let loginForm: LoginForm = {
    email: '',
    password: '',
    remember: true
  };

  let registerForm: RegisterForm = {
    firstName: '',
    lastName: '',
    email: '',
    phone: '',
    password: '',
    confirmPassword: '',
    acceptTerms: false
  };

  let isSubmitting = false;
  let errorMessage = '';
  let redirectTarget = '/';

  const getErrorMessage = (error: unknown, fallback: string) => {
    const axiosError = error as AxiosError<ApiErrorResponse> | undefined;
    return (
      axiosError?.response?.data?.message ||
      axiosError?.response?.data?.error ||
      axiosError?.message ||
      fallback
    );
  };

  const switchTab = (tab: 'login' | 'register') => {
    if (activeTab === tab) return;
    errorMessage = '';
    activeTab = tab;
  };

  $: redirectTarget = $page?.url?.searchParams?.get('redirectTo') ?? '/';

  const handleLogin = async () => {
    isSubmitting = true;
    errorMessage = '';

    try {
      const { token, user } = await authService.login(loginForm.email.trim().toLowerCase(), loginForm.password);
      setAuth(token, user, { remember: loginForm.remember });
      await goto(redirectTarget, { replaceState: true });
    } catch (error) {
      console.error('Error al iniciar sesión:', error);
      errorMessage = getErrorMessage(error, 'No se pudo iniciar sesión. Verifica tus datos.');
    } finally {
      isSubmitting = false;
    }
  };

  const handleRegister = async () => {
    isSubmitting = true;
    errorMessage = '';

    if (registerForm.password !== registerForm.confirmPassword) {
      errorMessage = 'Las contraseñas no coinciden.';
      isSubmitting = false;
      return;
    }

    if (!registerForm.acceptTerms) {
      errorMessage = 'Debes aceptar los términos y condiciones.';
      isSubmitting = false;
      return;
    }

    try {
      const { token, user } = await authService.register({
        firstName: registerForm.firstName.trim(),
        lastName: registerForm.lastName.trim(),
        email: registerForm.email.trim().toLowerCase(),
        phone: registerForm.phone?.trim() || undefined,
        password: registerForm.password
      });

      setAuth(token, user, { remember: true });
      await goto(redirectTarget, { replaceState: true });
    } catch (error) {
      console.error('Error al registrar usuario:', error);
      errorMessage = getErrorMessage(error, 'No se pudo crear la cuenta. Intenta nuevamente.');
    } finally {
      isSubmitting = false;
    }
  };

  const onSubmit = (event: SubmitEvent) => {
    event.preventDefault();
    if (activeTab === 'login') {
      handleLogin();
    } else {
      handleRegister();
    }
  };
</script>

<svelte:head>
  <title>Accede a tu cuenta | KronosTech</title>
  <meta name="description" content="Inicia sesión o crea una cuenta para gestionar tus compras en KronosTech" />
</svelte:head>

<div class="min-h-[calc(100vh-6rem)] bg-background-light dark:bg-background-dark flex items-center">
  <div class="w-full max-w-2xl mx-auto px-4 sm:px-6 lg:px-8 py-10">
    <div class="bg-surface-light dark:bg-surface-dark rounded-2xl shadow-xl border border-slate-200/60 dark:border-slate-700/60">
      <div class="px-6 sm:px-8 pt-6 sm:pt-8 pb-4 border-b border-slate-100 dark:border-slate-800">
        <h1 class="text-2xl sm:text-3xl font-bold tracking-[-0.02em] text-slate-900 dark:text-white">
          {#if activeTab === 'login'}
            Inicia sesión en tu cuenta
          {:else}
            Crea tu cuenta
          {/if}
        </h1>
        <p class="mt-2 text-sm sm:text-base text-slate-600 dark:text-slate-400">
          Accede para completar tu compra y revisar el estado de tus pedidos.
        </p>

        <div class="mt-6 inline-flex rounded-full bg-slate-100 dark:bg-slate-800 p-1">
          <button
            type="button"
            class={`px-4 sm:px-6 py-2 text-sm font-semibold rounded-full transition
            ${activeTab === 'login'
              ? 'bg-primary text-white shadow-md shadow-primary/30'
              : 'text-slate-600 dark:text-slate-300 hover:text-slate-900 dark:hover:text-white'}`}
            on:click={() => switchTab('login')}
          >
            Iniciar sesión
          </button>
          <button
            type="button"
            class={`px-4 sm:px-6 py-2 text-sm font-semibold rounded-full transition
            ${activeTab === 'register'
              ? 'bg-primary text-white shadow-md shadow-primary/30'
              : 'text-slate-600 dark:text-slate-300 hover:text-slate-900 dark:hover:text-white'}`}
            on:click={() => switchTab('register')}
          >
            Crear cuenta
          </button>
        </div>
      </div>

      <form class="px-6 sm:px-8 py-6 sm:py-8 space-y-6" on:submit={onSubmit}>
        {#if errorMessage}
          <div class="rounded-lg border border-red-200 dark:border-red-500/30 bg-red-50 dark:bg-red-500/10 px-4 py-3 text-sm text-red-700 dark:text-red-300">
            {errorMessage}
          </div>
        {/if}

        {#if activeTab === 'login'}
          <div class="space-y-4">
            <div class="space-y-1">
              <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">
                Correo electrónico
              </label>
              <input
                type="email"
                class="form-input w-full rounded-lg border border-slate-200 dark:border-slate-700 px-3 py-2 text-sm bg-white dark:bg-slate-900 focus:outline-none focus:ring-2 focus:ring-primary/70 focus:border-transparent"
                placeholder="tucorreo@ejemplo.com"
                bind:value={loginForm.email}
                required
              />
            </div>

            <div class="space-y-1">
              <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">
                Contraseña
              </label>
              <input
                type="password"
                class="form-input w-full rounded-lg border border-slate-200 dark:border-slate-700 px-3 py-2 text-sm bg-white dark:bg-slate-900 focus:outline-none focus:ring-2 focus:ring-primary/70 focus:border-transparent"
                placeholder="••••••••"
                bind:value={loginForm.password}
                required
              />
            </div>

            <div class="flex flex-col gap-2 sm:flex-row sm:items-center sm:justify-between">
              <label class="inline-flex items-center gap-2 text-xs sm:text-sm text-slate-600 dark:text-slate-400">
                <input
                  type="checkbox"
                  class="rounded border-slate-300 dark:border-slate-600 text-primary focus:ring-primary/70"
                  bind:checked={loginForm.remember}
                />
                <span>Recordar sesión en este dispositivo</span>
              </label>
              <button
                type="button"
                class="text-xs sm:text-sm font-semibold text-primary hover:underline"
              >
                ¿Olvidaste tu contraseña?
              </button>
            </div>
          </div>
        {:else}
          <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
            <div class="space-y-1">
              <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">Nombre</label>
              <input
                type="text"
                class="form-input w-full rounded-lg border border-slate-200 dark:border-slate-700 px-3 py-2 text-sm bg-white dark:bg-slate-900 focus:outline-none focus:ring-2 focus:ring-primary/70 focus:border-transparent"
                placeholder="Juan"
                bind:value={registerForm.firstName}
                required
              />
            </div>
            <div class="space-y-1">
              <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">Apellido</label>
              <input
                type="text"
                class="form-input w-full rounded-lg border border-slate-200 dark:border-slate-700 px-3 py-2 text-sm bg-white dark:bg-slate-900 focus:outline-none focus:ring-2 focus:ring-primary/70 focus:border-transparent"
                placeholder="Pérez"
                bind:value={registerForm.lastName}
                required
              />
            </div>
            <div class="space-y-1">
              <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">
                Correo electrónico
              </label>
              <input
                type="email"
                class="form-input w-full rounded-lg border border-slate-200 dark:border-slate-700 px-3 py-2 text-sm bg-white dark:bg-slate-900 focus:outline-none focus:ring-2 focus:ring-primary/70 focus:border-transparent"
                placeholder="tucorreo@ejemplo.com"
                bind:value={registerForm.email}
                required
              />
            </div>
            <div class="space-y-1">
              <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">
                Teléfono
              </label>
              <input
                type="tel"
                class="form-input w-full rounded-lg border border-slate-200 dark:border-slate-700 px-3 py-2 text-sm bg-white dark:bg-slate-900 focus:outline-none focus:ring-2 focus:ring-primary/70 focus:border-transparent"
                placeholder="999 999 999"
                bind:value={registerForm.phone}
              />
            </div>
            <div class="space-y-1">
              <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">
                Contraseña
              </label>
              <input
                type="password"
                class="form-input w-full rounded-lg border border-slate-200 dark:border-slate-700 px-3 py-2 text-sm bg-white dark:bg-slate-900 focus:outline-none focus:ring-2 focus:ring-primary/70 focus:border-transparent"
                placeholder="••••••••"
                bind:value={registerForm.password}
                required
              />
            </div>
            <div class="space-y-1">
              <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">
                Confirmar contraseña
              </label>
              <input
                type="password"
                class="form-input w-full rounded-lg border border-slate-200 dark:border-slate-700 px-3 py-2 text-sm bg-white dark:bg-slate-900 focus:outline-none focus:ring-2 focus:ring-primary/70 focus:border-transparent"
                placeholder="••••••••"
                bind:value={registerForm.confirmPassword}
                required
              />
            </div>
          </div>

          <div class="pt-2">
            <label class="inline-flex items-start gap-2 text-xs sm:text-sm text-slate-600 dark:text-slate-400">
              <input
                type="checkbox"
                class="mt-1 rounded border-slate-300 dark:border-slate-600 text-primary focus:ring-primary/70"
                bind:checked={registerForm.acceptTerms}
              />
              <span>
                Acepto los <span class="text-primary font-semibold hover:underline cursor-pointer">
                  términos y condiciones
                </span> y la política de privacidad.
              </span>
            </label>
          </div>
        {/if}

        <div class="pt-2">
          <button
            type="submit"
            class="inline-flex items-center justify-center w-full h-12 rounded-lg bg-primary text-white font-semibold text-sm shadow-lg shadow-primary/30 hover:bg-primary/90 transition disabled:opacity-60 disabled:cursor-not-allowed"
            disabled={isSubmitting}
          >
            {#if isSubmitting}
              Procesando...
            {:else if activeTab === 'login'}
              Iniciar sesión
            {:else}
              Crear cuenta
            {/if}
          </button>
        </div>

        <p class="text-xs sm:text-[0.8rem] text-slate-500 dark:text-slate-400 pt-1">
          Al utilizar KronosTech aceptas nuestros términos de uso y políticas de privacidad. Podrás
          actualizar tus preferencias desde tu perfil más adelante.
        </p>
      </form>
    </div>
  </div>
</div>
