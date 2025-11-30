<script lang="ts">
  let isSidebarOpen = false;

  function openSidebar() {
    isSidebarOpen = true;
  }

  function closeSidebar() {
    isSidebarOpen = false;
  }
</script>

<div class="relative flex h-auto min-h-screen w-full flex-col group/design-root bg-background-light dark:bg-background-dark font-display text-[#111418] dark:text-gray-200">
<div class="layout-container flex h-full grow flex-col">
<main class="flex-1 p-4 sm:p-6 lg:p-8">
<div class="mx-auto max-w-7xl">
<div class="flex flex-col sm:flex-row flex-wrap justify-between items-start gap-4 mb-6">
<div class="flex items-center gap-4">
<h1 class="text-3xl lg:text-4xl font-black tracking-tight text-[#111418] dark:text-white">Gestión de Descuentos</h1>
<div class="flex h-8 shrink-0 items-center justify-center gap-x-2 rounded-full bg-primary/10 dark:bg-primary/20 px-4">
<span class="material-symbols-outlined text-primary dark:text-primary-400 !text-base">sell</span>
<p class="text-primary dark:text-primary-400 text-sm font-medium">42 Descuentos Activos</p>
</div>
</div>
<a href="/gestion-descuentos/nuevo" class="flex items-center justify-center gap-2 min-w-[84px] cursor-pointer rounded-lg h-10 px-4 bg-primary text-white text-sm font-bold leading-normal tracking-[0.015em]">
<span class="material-symbols-outlined">add_circle</span>
<span class="truncate">Crear Descuento</span>
</a>
</div>



      <div class="mb-6">
        <h3 class="text-sm font-semibold uppercase text-gray-500 dark:text-gray-400 mb-3 tracking-wider">Estadísticas Superiores</h3>
        <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-4 gap-6">
          <div class="bg-white dark:bg-background-dark rounded-xl p-6 shadow-sm flex items-start gap-4 border border-gray-100 dark:border-slate-800">
            <div class="flex-shrink-0 size-12 rounded-lg bg-primary/10 flex items-center justify-center">
              <span class="material-symbols-outlined text-primary text-3xl">sell</span>
            </div>
            <div class="flex flex-col">
              <p class="text-gray-500 dark:text-gray-400 text-sm">Total de descuentos activos</p>
              <p class="text-gray-900 dark:text-white text-2xl font-bold">42</p>
            </div>
          </div>
          <div class="bg-white dark:bg-background-dark rounded-xl p-6 shadow-sm flex items-start gap-4 border border-gray-100 dark:border-slate-800">
            <div class="flex-shrink-0 size-12 rounded-lg bg-primary/10 flex items-center justify-center">
              <span class="material-symbols-outlined text-primary text-3xl">today</span>
            </div>
            <div class="flex flex-col">
              <p class="text-gray-500 dark:text-gray-400 text-sm">Descuentos vigentes hoy</p>
              <p class="text-gray-900 dark:text-white text-2xl font-bold">18</p>
            </div>
          </div>
          <div class="bg-white dark:bg-background-dark rounded-xl p-6 shadow-sm flex items-start gap-4 border border-gray-100 dark:border-slate-800">
            <div class="flex-shrink-0 size-12 rounded-lg bg-primary/10 flex items-center justify-center">
              <span class="material-symbols-outlined text-primary text-3xl">monitoring</span>
            </div>
            <div class="flex flex-col">
              <p class="text-gray-500 dark:text-gray-400 text-sm">Total de uso en el mes actual</p>
              <p class="text-gray-900 dark:text-white text-2xl font-bold">1,284</p>
            </div>
          </div>
          <div class="bg-white dark:bg-background-dark rounded-xl p-6 shadow-sm flex items-start gap-4 border border-gray-100 dark:border-slate-800">
            <div class="flex-shrink-0 size-12 rounded-lg bg-primary/10 flex items-center justify-center">
              <span class="material-symbols-outlined text-primary text-3xl">savings</span>
            </div>
            <div class="flex flex-col">
              <p class="text-gray-500 dark:text-gray-400 text-sm">Ahorro generado este mes</p>
              <p class="text-gray-900 dark:text-white text-2xl font-bold">$14,730.50</p>
            </div>
          </div>
        </div>
      </div>

<div class="mb-6 rounded-xl border border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-900/50 p-4">
        <div class="flex flex-col md:flex-row gap-4 mb-4">
          <div class="flex-grow">
            <label class="flex flex-col min-w-40 h-12 w-full">
              <div class="flex w-full flex-1 items-stretch rounded-lg h-full border border-gray-200 dark:border-slate-700">
                <div class="text-gray-500 dark:text-gray-400 flex bg-background-light dark:bg-slate-800 items-center justify-center pl-4 rounded-l-lg border-r-0">
                  <span class="material-symbols-outlined">search</span>
                </div>
                <input class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-r-lg text-gray-900 dark:text-white focus:outline-0 focus:ring-2 focus:ring-primary/50 border-none bg-background-light dark:bg-slate-800 focus:border-none h-full placeholder:text-gray-500 dark:placeholder:text-gray-400 px-4 pl-2 text-sm font-normal leading-normal" placeholder="Buscar por nombre de descuento..." value=""/>
              </div>
            </label>
          </div>
          <div class="flex gap-3 overflow-x-auto pb-2">
            <button class="flex h-10 shrink-0 items-center justify-center gap-x-2 rounded-lg bg-background-light dark:bg-slate-800 px-4 text-gray-800 dark:text-gray-200 hover:bg-gray-200 dark:hover:bg-slate-700 transition-colors">
              <p class="text-sm font-medium leading-normal">Tipo de descuento</p>
              <span class="material-symbols-outlined text-base">expand_more</span>
            </button>
            <button class="flex h-10 shrink-0 items-center justify-center gap-x-2 rounded-lg bg-background-light dark:bg-slate-800 px-4 text-gray-800 dark:text-gray-200 hover:bg-gray-200 dark:hover:bg-slate-700 transition-colors">
              <p class="text-sm font-medium leading-normal">Aplica a</p>
              <span class="material-symbols-outlined text-base">expand_more</span>
            </button>
            <button class="flex h-10 shrink-0 items-center justify-center gap-x-2 rounded-lg bg-background-light dark:bg-slate-800 px-4 text-gray-800 dark:text-gray-200 hover:bg-gray-200 dark:hover:bg-slate-700 transition-colors">
              <p class="text-sm font-medium leading-normal">Estado</p>
              <span class="material-symbols-outlined text-base">expand_more</span>
            </button>
            <button class="flex h-10 shrink-0 items-center justify-center gap-x-2 rounded-lg bg-background-light dark:bg-slate-800 px-4 text-gray-800 dark:text-gray-200 hover:bg-gray-200 dark:hover:bg-slate-700 transition-colors">
              <p class="text-sm font-medium leading-normal">Vigencia</p>
              <span class="material-symbols-outlined text-base">expand_more</span>
            </button>
          </div>
        </div>
        <div class="overflow-x-auto">
          <div class="border border-gray-200 dark:border-slate-800 rounded-lg">
            <table class="min-w-full divide-y divide-gray-200 dark:divide-slate-800">
              <thead class="bg-background-light dark:bg-slate-800">
                <tr>
                  <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider" scope="col">Nombre</th>
                  <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider" scope="col">Tipo</th>
                  <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider" scope="col">Valor</th>
                  <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider" scope="col">Aplica a</th>
                  <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider" scope="col">Vigencia</th>
                  <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider" scope="col">Usos</th>
                  <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider" scope="col">Estado</th>
                  <th class="relative px-6 py-3" scope="col"><span class="sr-only">Acciones</span></th>
                </tr>
              </thead>
              <tbody class="bg-white dark:bg-background-dark divide-y divide-gray-200 dark:divide-slate-800">
                <tr>
                  <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-white">CYBERMONDAY20</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">Porcentaje</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">20%</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">Global</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">25 Nov - 30 Nov</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">
                    <div class="flex items-center gap-2">
                      <div class="w-24 bg-gray-200 dark:bg-slate-700 rounded-full h-1.5"><div class="bg-primary h-1.5 rounded-full" style="width: 85%"></div></div>
                      <span class="text-gray-800 dark:text-gray-200">75/88</span>
                    </div>
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap">
                    <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 text-green-800">Vigente</span>
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                    <div class="flex items-center justify-end gap-4">
                      <button class="text-primary hover:text-primary/80" on:click={openSidebar}>Edit</button>
                      <button class="text-red-500 hover:text-red-700">
                        <span class="material-symbols-outlined text-xl">delete</span>
                      </button>
                    </div>
                  </td>
                </tr>
                <tr>
                  <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-white">NVIDIA10</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">Porcentaje</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">10%</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">Marca: Nvidia</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">01 Ene - 31 Ene</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">
                    <div class="flex items-center gap-2">
                      <div class="w-24 bg-gray-200 dark:bg-slate-700 rounded-full h-1.5"><div class="bg-primary h-1.5 rounded-full" style="width: 0%"></div></div>
                      <span class="text-gray-800 dark:text-gray-200">0/100</span>
                    </div>
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap">
                    <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-yellow-100 text-yellow-800">Próximo</span>
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                    <div class="flex items-center justify-end gap-4">
                      <button class="text-primary hover:text-primary/80" on:click={openSidebar}>Edit</button>
                      <button class="text-red-500 hover:text-red-700">
                        <span class="material-symbols-outlined text-xl">delete</span>
                      </button>
                    </div>
                  </td>
                </tr>
                <tr>
                  <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-white">50OFF-SSD</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">Monto Fijo</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">$50</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">Categoría: SSD</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">01 Dic - 15 Dic</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">
                    <div class="flex items-center gap-2">
                      <div class="w-24 bg-gray-200 dark:bg-slate-700 rounded-full h-1.5"><div class="bg-red-500 h-1.5 rounded-full" style="width: 100%"></div></div>
                      <span class="text-gray-800 dark:text-gray-200">150/150</span>
                    </div>
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap">
                    <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 text-green-800">Vigente</span>
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                    <div class="flex items-center justify-end gap-4">
                      <button class="text-primary hover:text-primary/80" on:click={openSidebar}>Edit</button>
                      <button class="text-red-500 hover:text-red-700">
                        <span class="material-symbols-outlined text-xl">delete</span>
                      </button>
                    </div>
                  </td>
                </tr>
                <tr>
                  <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-white">WELCOME_USER</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">Monto Fijo</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">$15</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">Global</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">01 Ene - 31 Oct</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">
                    <div class="flex items-center gap-2">
                      <div class="w-24 bg-gray-200 dark:bg-slate-700 rounded-full h-1.5"><div class="bg-red-500 h-1.5 rounded-full" style="width: 100%"></div></div>
                      <span class="text-gray-800 dark:text-gray-200">100/100</span>
                    </div>
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap">
                    <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-gray-100 text-gray-800">Vencido</span>
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                    <div class="flex items-center justify-end gap-4">
                      <button class="text-primary hover:text-primary/80" on:click={openSidebar}>Edit</button>
                      <button class="text-red-500 hover:text-red-700">
                        <span class="material-symbols-outlined text-xl">delete</span>
                      </button>
                    </div>
                  </td>
                </tr>
                <tr>
                  <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-white">FREESHIPPING</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">Envío Gratis</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">100%</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">Global</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">01 Sep - 31 Dic</td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">
                    <div class="flex items-center gap-2">
                      <div class="w-24 bg-gray-200 dark:bg-slate-700 rounded-full h-1.5"><div class="bg-primary h-1.5 rounded-full" style="width: 51%"></div></div>
                      <span class="text-gray-800 dark:text-gray-200">45/∞</span>
                    </div>
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap">
                    <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 text-green-800">Vigente</span>
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                    <div class="flex items-center justify-end gap-4">
                      <button class="text-primary hover:text-primary/80" on:click={openSidebar}>Edit</button>
                      <button class="text-red-500 hover:text-red-700">
                        <span class="material-symbols-outlined text-xl">delete</span>
                      </button>
                    </div>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
  </main>
</div>
</div>

<!-- Side Panel Overlay -->
{#if isSidebarOpen}
  <div class="fixed inset-0 bg-black/50 z-40 transition-opacity" on:click={closeSidebar} on:keydown={(e) => e.key === 'Escape' && closeSidebar()} role="button" tabindex="0"></div>
{/if}

<!-- Side Panel -->
<div class="fixed top-0 right-0 h-full w-full max-w-2xl bg-white dark:bg-gray-900 shadow-2xl flex flex-col transform transition-transform duration-300 ease-in-out z-50 {isSidebarOpen ? 'translate-x-0' : 'translate-x-full'}">
  <div class="flex-shrink-0 flex items-center justify-between p-4 border-b border-gray-200 dark:border-gray-700">
    <div class="flex flex-col">
      <h1 class="text-lg font-bold text-gray-900 dark:text-white">Detalle del Descuento</h1>
      <p class="text-sm font-medium text-primary">CYBERWEEK15</p>
    </div>
    <button class="p-2 rounded-full hover:bg-gray-100 dark:hover:bg-gray-800 text-gray-500 dark:text-gray-400" on:click={closeSidebar}>
      <span class="material-symbols-outlined">close</span>
    </button>
  </div>
  <div class="flex-grow overflow-y-auto p-6 space-y-6 bg-background-light dark:bg-background-dark">
    <!-- General Information Card -->
    <div class="bg-white dark:bg-gray-900/50 rounded-lg border border-gray-200 dark:border-gray-700/50">
      <div class="flex justify-between items-center px-4 pt-4 pb-2">
        <h3 class="text-lg font-bold leading-tight tracking-[-0.015em] text-[#111418] dark:text-white">Información General</h3>
        <div class="flex h-8 shrink-0 items-center justify-center gap-x-2 rounded-lg bg-green-100 dark:bg-green-900/50 px-4">
          <p class="text-green-800 dark:text-green-300 text-sm font-medium leading-normal">Activo</p>
        </div>
      </div>
      <div class="p-4 grid grid-cols-[160px_1fr] gap-x-6">
        <div class="col-span-2 grid grid-cols-subgrid border-t border-t-[#dbe0e6] dark:border-t-gray-700 py-4">
          <p class="text-[#617589] dark:text-gray-400 text-sm font-normal leading-normal">Descripción</p>
          <p class="text-[#111418] dark:text-gray-200 text-sm font-normal leading-normal">Descuento especial para la Cyber Week en componentes seleccionados.</p>
        </div>
        <div class="col-span-2 grid grid-cols-subgrid border-t border-t-[#dbe0e6] dark:border-t-gray-700 py-4">
          <p class="text-[#617589] dark:text-gray-400 text-sm font-normal leading-normal">Código</p>
          <p class="text-[#111418] dark:text-gray-200 text-sm font-normal leading-normal">CYBERWEEK15</p>
        </div>
        <div class="col-span-2 grid grid-cols-subgrid border-t border-t-[#dbe0e6] dark:border-t-gray-700 py-4">
          <p class="text-[#617589] dark:text-gray-400 text-sm font-normal leading-normal">Tipo</p>
          <p class="text-[#111418] dark:text-gray-200 text-sm font-normal leading-normal">Porcentaje</p>
        </div>
        <div class="col-span-2 grid grid-cols-subgrid border-t border-t-[#dbe0e6] dark:border-t-gray-700 py-4">
          <p class="text-[#617589] dark:text-gray-400 text-sm font-normal leading-normal">Valor</p>
          <p class="text-[#111418] dark:text-gray-200 text-sm font-normal leading-normal">15%</p>
        </div>
        <div class="col-span-2 grid grid-cols-subgrid border-t border-t-[#dbe0e6] dark:border-t-gray-700 py-4">
          <p class="text-[#617589] dark:text-gray-400 text-sm font-normal leading-normal">Aplica a</p>
          <p class="text-[#111418] dark:text-gray-200 text-sm font-normal leading-normal">Categoría: Tarjetas Gráficas</p>
        </div>
        <div class="col-span-2 grid grid-cols-subgrid border-t border-t-[#dbe0e6] dark:border-t-gray-700 py-4">
          <p class="text-[#617589] dark:text-gray-400 text-sm font-normal leading-normal">Fechas de Validez</p>
          <p class="text-[#111418] dark:text-gray-200 text-sm font-normal leading-normal">01/11/2024 - 07/11/2024</p>
        </div>
        <div class="col-span-2 grid grid-cols-subgrid border-t border-t-[#dbe0e6] dark:border-t-gray-700 py-4">
          <p class="text-[#617589] dark:text-gray-400 text-sm font-normal leading-normal">Condiciones</p>
          <p class="text-[#111418] dark:text-gray-200 text-sm font-normal leading-normal">Compra mínima de $500</p>
        </div>
      </div>
    </div>
    <!-- Statistics Card -->
    <div class="bg-white dark:bg-gray-900/50 rounded-lg border border-gray-200 dark:border-gray-700/50">
      <h3 class="text-[#111418] dark:text-white text-lg font-bold leading-tight tracking-[-0.015em] px-4 pb-2 pt-4">Estadísticas Clave</h3>
      <div class="grid grid-cols-2 p-4">
        <div class="p-4">
          <p class="text-sm text-gray-500 dark:text-gray-400 mb-1">Total de Usos</p>
          <p class="text-3xl font-bold text-gray-900 dark:text-white">1,284</p>
        </div>
        <div class="p-4 border-l border-gray-200 dark:border-gray-700">
          <p class="text-sm text-gray-500 dark:text-gray-400 mb-1">Tasa de Conversión</p>
          <p class="text-3xl font-bold text-gray-900 dark:text-white">72%</p>
        </div>
      </div>
    </div>
    <!-- Usage Chart Card -->
    <div class="bg-white dark:bg-gray-900/50 rounded-lg border border-gray-200 dark:border-gray-700/50">
      <h3 class="text-[#111418] dark:text-white text-lg font-bold leading-tight tracking-[-0.015em] px-4 pb-2 pt-4">Historial de Usos</h3>
      <div class="p-4">
        <img class="w-full h-auto rounded-md" data-alt="Un gráfico de líneas que muestra los usos del descuento a lo largo del tiempo, con picos y valles que indican la frecuencia de aplicación." src="https://lh3.googleusercontent.com/aida-public/AB6AXuA77T2KAAwbKEEasFTbnPhzjXCno17hzKqVq2STqbW-qDbqwlquyZ0tCCW1zgKhr8_A2CVidlRQyZ_1lMb6wXl5Gj_FuwVxFpnU1lvJeHjZ_rf_DXsfXaPjq5Ojtejp94xqpMT28hKjqkbKY6nrjo4TroxW5vzzpCDBqRZitGiUW8f3gYO6g4MY8SmvkiCTyWGAQYqSI67E9ms_y0gHXpaEPYZiy4P73hRbzpEl6qLHeCvyZM-bAMeDtEem4Y8Q1JaTaXQMe8os-wk"/>
      </div>
    </div>
    <!-- Recent Applications Card -->
    <div class="bg-white dark:bg-gray-900/50 rounded-lg border border-gray-200 dark:border-gray-700/50">
      <h3 class="text-[#111418] dark:text-white text-lg font-bold leading-tight tracking-[-0.015em] px-4 pb-2 pt-4">Últimas 10 Aplicaciones</h3>
      <div class="overflow-x-auto">
        <table class="w-full text-sm text-left text-gray-500 dark:text-gray-400">
          <thead class="text-xs text-gray-700 dark:text-gray-400 uppercase bg-gray-50 dark:bg-gray-800">
            <tr>
              <th class="px-6 py-3" scope="col">ID Pedido</th>
              <th class="px-6 py-3" scope="col">Cliente</th>
              <th class="px-6 py-3" scope="col">Fecha</th>
              <th class="px-6 py-3" scope="col">Monto</th>
            </tr>
          </thead>
          <tbody>
            <tr class="bg-white dark:bg-gray-900/50 border-b dark:border-gray-700">
              <th class="px-6 py-4 font-medium text-gray-900 dark:text-white whitespace-nowrap" scope="row">#98765</th>
              <td class="px-6 py-4">Ana Torres</td>
              <td class="px-6 py-4">06/11/2024</td>
              <td class="px-6 py-4">$1,150.00</td>
            </tr>
            <tr class="bg-white dark:bg-gray-900/50 border-b dark:border-gray-700">
              <th class="px-6 py-4 font-medium text-gray-900 dark:text-white whitespace-nowrap" scope="row">#98764</th>
              <td class="px-6 py-4">Carlos Ruiz</td>
              <td class="px-6 py-4">06/11/2024</td>
              <td class="px-6 py-4">$890.50</td>
            </tr>
            <tr class="bg-white dark:bg-gray-900/50 border-b dark:border-gray-700">
              <th class="px-6 py-4 font-medium text-gray-900 dark:text-white whitespace-nowrap" scope="row">#98763</th>
              <td class="px-6 py-4">Lucía Gómez</td>
              <td class="px-6 py-4">05/11/2024</td>
              <td class="px-6 py-4">$620.00</td>
            </tr>
            <tr class="bg-white dark:bg-gray-900/50 border-b dark:border-gray-700">
              <th class="px-6 py-4 font-medium text-gray-900 dark:text-white whitespace-nowrap" scope="row">#98762</th>
              <td class="px-6 py-4">Javier Fernández</td>
              <td class="px-6 py-4">05/11/2024</td>
              <td class="px-6 py-4">$1,300.00</td>
            </tr>
            <tr class="bg-white dark:bg-gray-900/50">
              <th class="px-6 py-4 font-medium text-gray-900 dark:text-white whitespace-nowrap" scope="row">#98761</th>
              <td class="px-6 py-4">Sofía Martínez</td>
              <td class="px-6 py-4">04/11/2024</td>
              <td class="px-6 py-4">$950.75</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
  <div class="flex-shrink-0 flex items-center justify-end gap-4 p-4 border-t border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-900">
    <button class="flex min-w-[84px] max-w-[480px] cursor-pointer items-center justify-center overflow-hidden rounded-lg h-10 px-4 bg-gray-200 dark:bg-gray-700 text-gray-800 dark:text-gray-200 text-sm font-bold leading-normal tracking-[0.015em] hover:bg-gray-300 dark:hover:bg-gray-600">
      <span class="truncate">Archivar</span>
    </button>
    <button class="flex min-w-[84px] max-w-[480px] cursor-pointer items-center justify-center overflow-hidden rounded-lg h-10 px-4 bg-primary text-white text-sm font-bold leading-normal tracking-[0.015em] hover:bg-blue-600 dark:hover:bg-blue-500">
      <span class="truncate">Editar Descuento</span>
    </button>
  </div>
</div>
