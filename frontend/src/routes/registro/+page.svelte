<script lang="ts">
  import { goto } from '$app/navigation';
  import { register } from '$lib/services/auth';
  import { setAuthLoading, setUser } from '$lib/stores/auth';
  import { UserPlus, Mail, Phone, IdCard, ArrowLeft } from 'lucide-svelte';

  type Step = 1 | 2;
  let step: Step = 1;

  let nombre = '';
  let apellido = '';
  let email = '';
  let telefono = '';
  let dni = '';
  let aceptaTerminos = false;

  let password = '';
  let confirmPassword = '';
  let aceptaMarketing = false;

  let loading = false;
  let apiError = '';
  let success = false;
  let errors: Record<string, string> = {};

  const emailRegex = /^[^@\s]+@[^@\s]+\.[^@\s]+$/;

  function validateStep1() {
    errors = {};
    if (!nombre.trim()) errors.nombre = 'Los nombres son obligatorios.';
    if (!apellido.trim()) errors.apellido = 'Los apellidos son obligatorios.';
    if (!email.trim()) {
      errors.email = 'El correo es obligatorio.';
    } else if (!emailRegex.test(email)) {
      errors.email = 'Ingresa un correo válido.';
    }
    if (!aceptaTerminos) errors.terminos = 'Debes aceptar los términos y condiciones.';
    return Object.keys(errors).length === 0;
  }

  function validateStep2() {
    errors = {};
    if (!password) {
      errors.password = 'La contraseña es obligatoria.';
    } else if (password.length < 6) {
      errors.password = 'Debe tener al menos 6 caracteres.';
    }

    if (!confirmPassword) {
      errors.confirmPassword = 'Confirma tu contraseña.';
    } else if (password !== confirmPassword) {
      errors.confirmPassword = 'Las contraseñas no coinciden.';
    }
    return Object.keys(errors).length === 0;
  }

  function goToStep2() {
    apiError = '';
    if (validateStep1()) step = 2;
  }

  function goToStep1() {
    apiError = '';
    errors = {};
    step = 1;
  }

  async function handleRegister() {
    apiError = '';
    if (!validateStep2()) return;

    loading = true;
    setAuthLoading(true);

    try {
      const response = await register({
        nombre,
        apellido,
        email,
        telefono: telefono || undefined,
        dni: dni || undefined,
        password,
      });

      setUser(response.data.usuario);
      success = true;

      setTimeout(() => {
        goto('/login');
      }, 1500);
    } catch (err: any) {
      apiError = err.message || 'No se pudo completar el registro. Inténtalo más tarde.';
    } finally {
      loading = false;
      setAuthLoading(false);
    }
  }
</script>

<svelte:head>
  <title>Registro | KronosTech</title>
</svelte:head>

<div class="min-h-screen bg-surface-light dark:bg-surface-dark flex items-center justify-center px-4">
  <div class="w-full max-w-2xl">
    <div class="bg-surface-light dark:bg-surface-dark border border-border-light dark:border-border-dark rounded-2xl shadow-xl px-8 py-10">
      <div class="flex items-center justify-between mb-4">
        <div>
          <h1 class="text-2xl font-semibold text-text-light dark:text-text-dark">
            Registro de Nuevo Usuario
          </h1>
          <p class="mt-1 text-sm text-slate-600 dark:text-slate-300">
            {step === 1
              ? 'Paso 1 de 2: Ingresa tus datos personales.'
              : 'Paso 2 de 2: Configura la seguridad de tu cuenta.'}
          </p>
        </div>

        <div class="hidden md:flex h-10 w-10 items-center justify-center rounded-full bg-primary/10 text-primary">
          <UserPlus class="h-5 w-5" />
        </div>
      </div>

      <div class="flex gap-2 mb-6">
        <div class={`h-1.5 flex-1 rounded-full ${step >= 1 ? 'bg-primary' : 'bg-slate-300 dark:bg-slate-700'}`}></div>
        <div class={`h-1.5 flex-1 rounded-full ${step >= 2 ? 'bg-primary' : 'bg-slate-300 dark:bg-slate-700'}`}></div>
      </div>

      {#if apiError}
        <div class="mb-4 rounded-lg border border-red-500 bg-red-50 text-red-700 text-sm px-4 py-3 dark:bg-red-500/10 dark:text-red-200">
          {apiError}
        </div>
      {/if}

      {#if success}
        <div class="rounded-xl border border-emerald-500 bg-emerald-50 text-emerald-700 text-sm px-4 py-4 text-center dark:bg-emerald-500/10 dark:text-emerald-200">
          Cuenta creada correctamente. Redirigiendo...
        </div>
      {:else}
        {#if step === 1}
          <form class="space-y-4" on:submit|preventDefault={goToStep2}>
            <div class="grid gap-4 md:grid-cols-2">
              <div>
                <label class="block text-sm font-medium text-text-light dark:text-text-dark mb-1">
                  Nombres *
                </label>
                <input
                  class={`form-input w-full ${errors.nombre ? 'border-red-500' : ''}`}
                  bind:value={nombre}
                  placeholder="Ingresa tus nombres"
                />
                {#if errors.nombre}
                  <p class="mt-1 text-xs text-red-500">{errors.nombre}</p>
                {/if}
              </div>

              <div>
                <label class="block text-sm font-medium text-text-light dark:text-text-dark mb-1">
                  Apellidos *
                </label>
                <input
                  class={`form-input w-full ${errors.apellido ? 'border-red-500' : ''}`}
                  bind:value={apellido}
                  placeholder="Ingresa tus apellidos"
                />
                {#if errors.apellido}
                  <p class="mt-1 text-xs text-red-500">{errors.apellido}</p>
                {/if}
              </div>
            </div>

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
                  class={`form-input w-full pl-10 ${errors.email ? 'border-red-500' : ''}`}
                  bind:value={email}
                  placeholder="usuario@correo.com"
                />
              </div>
              {#if errors.email}
                <p class="mt-1 text-xs text-red-500">{errors.email}</p>
              {/if}
            </div>

            <div class="grid gap-4 md:grid-cols-2">
              <div>
                <label class="block text-sm font-medium text-text-light dark:text-text-dark mb-1">
                  Teléfono / Celular <span class="text-slate-500 text-xs">(Opcional)</span>
                </label>
                <div class="relative">
                  <span class="absolute inset-y-0 left-3 flex items-center text-slate-500">
                    <Phone class="h-4 w-4" />
                  </span>
                  <input
                    class="form-input w-full pl-10"
                    bind:value={telefono}
                    placeholder="Ej: 987654321"
                  />
                </div>
              </div>

              <div>
                <label class="block text-sm font-medium text-text-light dark:text-text-dark mb-1">
                  DNI <span class="text-slate-500 text-xs">(Opcional)</span>
                </label>
                <div class="relative">
                  <span class="absolute inset-y-0 left-3 flex items-center text-slate-500">
                    <IdCard class="h-4 w-4" />
                  </span>
                  <input
                    class="form-input w-full pl-10"
                    bind:value={dni}
                    placeholder="Ej: 12345678"
                  />
                </div>
              </div>
            </div>

            <div class="pt-2">
              <label class="inline-flex items-start gap-2 text-sm text-slate-700 dark:text-slate-300">
                <input
                  type="checkbox"
                  class="mt-1 h-4 w-4 rounded border-border-light dark:border-border-dark"
                  bind:checked={aceptaTerminos}
                />
                <span>
                  Acepto los
                  <a href="/terminos" class="text-primary hover:underline">
                    términos y condiciones
                  </a>.
                </span>
              </label>
              {#if errors.terminos}
                <p class="mt-1 text-xs text-red-500">{errors.terminos}</p>
              {/if}
            </div>

            <div class="mt-4 flex justify-between">
              <a
                href="/"
                class="px-4 py-2.5 rounded-lg bg-slate-200 dark:bg-surface-dark text-sm text-text-light dark:text-text-dark hover:bg-slate-300"
              >
                Cancelar
              </a>

              <button
                type="submit"
                class="px-5 py-2.5 rounded-lg bg-primary text-white hover:bg-primary/90 text-sm font-semibold"
              >
                Siguiente
              </button>
            </div>
          </form>
        {:else}
          <form class="space-y-4" on:submit|preventDefault={handleRegister}>
            <div>
              <label class="block text-sm font-medium text-text-light dark:text-text-dark mb-1">
                Contraseña *
              </label>
              <input
                type="password"
                class={`form-input w-full ${errors.password ? 'border-red-500' : ''}`}
                bind:value={password}
                placeholder="Mínimo 6 caracteres"
              />
              {#if errors.password}
                <p class="mt-1 text-xs text-red-500">{errors.password}</p>
              {/if}
            </div>

            <div>
              <label class="block text-sm font-medium text-text-light dark:text-text-dark mb-1">
                Confirmar contraseña *
              </label>
              <input
                type="password"
                class={`form-input w-full ${errors.confirmPassword ? 'border-red-500' : ''}`}
                bind:value={confirmPassword}
                placeholder="Repite tu contraseña"
              />
              {#if errors.confirmPassword}
                <p class="mt-1 text-xs text-red-500">{errors.confirmPassword}</p>
              {/if}
            </div>

            <div class="pt-2">
              <label class="inline-flex items-start gap-2 text-sm text-slate-700 dark:text-slate-300">
                <input
                  type="checkbox"
                  class="mt-1 h-4 w-4 rounded border-border-light dark:border-border-dark"
                  bind:checked={aceptaMarketing}
                />
                <span>
                  Quiero recibir novedades, ofertas y recomendaciones de KronosTech.
                </span>
              </label>
            </div>

            <div class="mt-4 flex justify-between">
              <button
                type="button"
                on:click={goToStep1}
                class="inline-flex items-center gap-2 px-4 py-2.5 rounded-lg bg-slate-200 dark:bg-surface-dark text-sm text-text-light dark:text-text-dark hover:bg-slate-300"
              >
                <ArrowLeft class="h-4 w-4" />
                Anterior
              </button>

              <button
                type="submit"
                class="px-5 py-2.5 rounded-lg bg-primary text-white hover:bg-primary/90 text-sm font-semibold disabled:opacity-60 disabled:cursor-not-allowed"
                disabled={loading}
              >
                {#if loading}Creando cuenta...{:else}Crear cuenta{/if}
              </button>
            </div>
          </form>
        {/if}
      {/if}

      <div class="mt-6 text-center text-sm text-slate-700 dark:text-slate-300">
        ¿Ya tienes una cuenta?
        <a href="/login" class="text-primary font-medium hover:underline">
          Iniciar sesión
        </a>
      </div>
    </div>
  </div>
</div>
