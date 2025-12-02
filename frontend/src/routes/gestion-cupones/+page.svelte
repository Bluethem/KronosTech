<script lang="ts">
  let isCreateSidebarOpen = false;

  function openCreateSidebar() {
    isCreateSidebarOpen = true;
  }

  function closeCreateSidebar() {
    isCreateSidebarOpen = false;
  }

  let isDetailsSidebarOpen = false;

  function openDetailsSidebar() {
    isDetailsSidebarOpen = true;
  }

  function closeDetailsSidebar() {
    isDetailsSidebarOpen = false;
  }

  let activeTab = 'info';

  let isMassAssignModalOpen = false;
  let massAssignStep = 1;

  function openMassAssignModal() {
    isMassAssignModalOpen = true;
    massAssignStep = 1;
  }

  function closeMassAssignModal() {
    isMassAssignModalOpen = false;
  }
</script>

<div class="relative flex h-auto min-h-screen w-full flex-col group/design-root bg-background-light dark:bg-background-dark font-display text-[#111418] dark:text-gray-200">
  <div class="layout-container flex h-full grow flex-col">
    <main class="flex-1 p-6 lg:p-10">
      <div class="max-w-7xl mx-auto">
        <!-- PageHeading -->
        <div class="flex flex-wrap justify-between items-center gap-4 mb-8">
          <div class="flex flex-col">
            <h1 class="text-slate-900 dark:text-white text-4xl font-black leading-tight tracking-[-0.033em]">Gestión de Cupones</h1>
            <p class="text-slate-500 dark:text-slate-400 text-base font-normal leading-normal mt-2">Crea, administra y monitorea el rendimiento de tus cupones de descuento.</p>
          </div>
          <button class="flex min-w-[84px] cursor-pointer items-center justify-center gap-2 overflow-hidden rounded-lg h-10 px-4 bg-primary text-white text-sm font-bold leading-normal tracking-[0.015em] hover:bg-primary/90" on:click={openCreateSidebar}>
            <span class="material-symbols-outlined" style="font-variation-settings: 'wght' 600;">add</span>
            <span class="truncate">Crear Cupón</span>
          </button>
        </div>
        <!-- Stats -->
        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
          <div class="flex flex-col gap-2 rounded-xl p-6 bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800">
            <p class="text-slate-600 dark:text-slate-300 text-sm font-medium">Cupones activos ahora</p>
            <p class="text-slate-900 dark:text-white tracking-light text-3xl font-bold">84</p>
            <p class="text-green-600 dark:text-green-500 text-sm font-medium">+5% vs ayer</p>
          </div>
          <div class="flex flex-col gap-2 rounded-xl p-6 bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800">
            <p class="text-slate-600 dark:text-slate-300 text-sm font-medium">Usos totales hoy</p>
            <p class="text-slate-900 dark:text-white tracking-light text-3xl font-bold">1,230</p>
            <p class="text-green-600 dark:text-green-500 text-sm font-medium">+12% vs ayer</p>
          </div>
          <div class="flex flex-col gap-2 rounded-xl p-6 bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800">
            <p class="text-slate-600 dark:text-slate-300 text-sm font-medium">Descuento total (mes)</p>
            <p class="text-slate-900 dark:text-white tracking-light text-3xl font-bold">$4,500.50</p>
            <p class="text-green-600 dark:text-green-500 text-sm font-medium">+8% vs mes anterior</p>
          </div>
          <div class="flex flex-col gap-2 rounded-xl p-6 bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800">
            <p class="text-slate-600 dark:text-slate-300 text-sm font-medium">Cupón más usado</p>
            <p class="text-primary tracking-light text-2xl font-bold font-mono">PROCESADOR15</p>
            <p class="text-slate-500 dark:text-slate-400 text-sm font-medium">254 usos hoy</p>
          </div>
        </div>
        <!-- Toolbar and Search -->
        <div class="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-xl p-4 mb-6">
          <div class="flex flex-wrap justify-between items-center gap-4">
            <div class="flex-1 min-w-[250px]">
              <label class="flex flex-col w-full">
                <div class="flex w-full flex-1 items-stretch rounded-lg h-10">
                  <div class="text-slate-500 dark:text-slate-400 flex bg-background-light dark:bg-background-dark items-center justify-center pl-3 rounded-l-lg border border-r-0 border-slate-300 dark:border-slate-700">
                    <span class="material-symbols-outlined text-base">search</span>
                  </div>
                  <input class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-slate-800 dark:text-slate-200 focus:outline-0 focus:ring-2 focus:ring-primary/50 border border-l-0 border-slate-300 dark:border-slate-700 bg-background-light dark:bg-background-dark h-full placeholder:text-slate-500 dark:placeholder:text-slate-400 px-4 rounded-l-none pl-2 text-sm font-normal" placeholder="Buscar por código o nombre de cupón..." value=""/>
                </div>
              </label>
            </div>
            <div class="flex items-center gap-2">
              <button class="p-2 text-slate-500 dark:text-slate-400 rounded-lg hover:bg-slate-100 dark:hover:bg-slate-800 hover:text-primary">
                <span class="material-symbols-outlined">filter_list</span>
              </button>
              <button class="p-2 text-slate-500 dark:text-slate-400 rounded-lg hover:bg-slate-100 dark:hover:bg-slate-800 hover:text-primary">
                <span class="material-symbols-outlined">download</span>
              </button>
              <button class="flex cursor-pointer items-center justify-center overflow-hidden rounded-lg h-10 bg-primary/10 dark:bg-primary/20 text-primary gap-2 text-sm font-bold leading-normal tracking-[0.015em] min-w-0 px-4 hover:bg-primary/20 dark:hover:bg-primary/30" on:click={openMassAssignModal}>
                <span class="material-symbols-outlined text-base" style="font-variation-settings: 'FILL' 1;">groups</span>
                <span class="truncate">Asignación Masiva</span>
              </button>
            </div>
          </div>
        </div>
        <!-- Table -->
        <div class="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-xl overflow-hidden">
          <div class="overflow-x-auto">
            <table class="w-full text-sm text-left">
              <thead class="bg-slate-50 dark:bg-slate-800 text-xs text-slate-500 dark:text-slate-400 uppercase">
                <tr>
                  <th class="px-6 py-3" scope="col">Código</th>
                  <th class="px-6 py-3" scope="col">Nombre</th>
                  <th class="px-6 py-3" scope="col">Tipo</th>
                  <th class="px-6 py-3" scope="col">Valor</th>
                  <th class="px-6 py-3" scope="col">Vigencia</th>
                  <th class="px-6 py-3" scope="col">Usos</th>
                  <th class="px-6 py-3" scope="col">Restricciones</th>
                  <th class="px-6 py-3" scope="col"><span class="sr-only">Acciones</span></th>
                </tr>
              </thead>
              <tbody>
                <tr class="border-b dark:border-slate-800 hover:bg-slate-50 dark:hover:bg-slate-800/50">
                  <td class="px-6 py-4">
                    <div class="group relative flex items-center">
                      <span class="font-mono bg-slate-100 dark:bg-slate-800 text-slate-700 dark:text-slate-300 rounded px-2 py-1 text-xs font-semibold">GAMING2024</span>
                      <button class="absolute -right-7 opacity-0 group-hover:opacity-100 transition-opacity text-slate-400 hover:text-primary">
                        <span class="material-symbols-outlined text-base">content_copy</span>
                      </button>
                    </div>
                  </td>
                  <td class="px-6 py-4 font-medium text-slate-900 dark:text-white whitespace-nowrap">Descuento Gaming</td>
                  <td class="px-6 py-4"><span class="bg-blue-100 text-blue-800 text-xs font-medium mr-2 px-2.5 py-0.5 rounded-full dark:bg-blue-900 dark:text-blue-300">Porcentaje</span></td>
                  <td class="px-6 py-4">15%</td>
                  <td class="px-6 py-4">01 Ene - 31 Dic</td>
                  <td class="px-6 py-4">
                    <div class="flex items-center gap-2">
                      <div class="w-20 h-2 bg-slate-200 dark:bg-slate-700 rounded-full">
                        <div class="h-2 bg-primary rounded-full" style="width: 45%"></div>
                      </div>
                      <span>450/1000</span>
                    </div>
                  </td>
                  <td class="px-6 py-4">
                    <span class="bg-slate-100 text-slate-800 text-xs font-medium mr-2 px-2.5 py-0.5 rounded-full dark:bg-slate-700 dark:text-slate-300">Uso limitado</span>
                  </td>
                  <td class="px-6 py-4 text-right">
                    <div class="flex items-center justify-end gap-1">
                      <button class="p-2 rounded-md hover:bg-slate-100 dark:hover:bg-slate-800 text-slate-500 dark:text-slate-400 hover:text-primary" on:click={openDetailsSidebar}><span class="material-symbols-outlined text-lg">edit</span></button>
                      <button class="p-2 rounded-md hover:bg-slate-100 dark:hover:bg-slate-800 text-slate-500 dark:text-slate-400 hover:text-primary"><span class="material-symbols-outlined text-lg">content_copy</span></button>
                      <button class="p-2 rounded-md hover:bg-slate-100 dark:hover:bg-slate-800 text-slate-500 dark:text-slate-400 hover:text-red-500"><span class="material-symbols-outlined text-lg">delete</span></button>
                    </div>
                  </td>
                </tr>
                <tr class="border-b dark:border-slate-800 hover:bg-slate-50 dark:hover:bg-slate-800/50">
                  <td class="px-6 py-4">
                    <div class="group relative flex items-center">
                      <span class="font-mono bg-slate-100 dark:bg-slate-800 text-slate-700 dark:text-slate-300 rounded px-2 py-1 text-xs font-semibold">FREESHIP</span>
                      <button class="absolute -right-7 opacity-0 group-hover:opacity-100 transition-opacity text-slate-400 hover:text-primary">
                        <span class="material-symbols-outlined text-base">content_copy</span>
                      </button>
                    </div>
                  </td>
                  <td class="px-6 py-4 font-medium text-slate-900 dark:text-white whitespace-nowrap">Envío Gratis Enero</td>
                  <td class="px-6 py-4"><span class="bg-green-100 text-green-800 text-xs font-medium mr-2 px-2.5 py-0.5 rounded-full dark:bg-green-900 dark:text-green-300">Envío Gratis</span></td>
                  <td class="px-6 py-4">-</td>
                  <td class="px-6 py-4">01 Ene - 31 Ene</td>
                  <td class="px-6 py-4">
                    <div class="flex items-center gap-2">
                      <div class="w-20 h-2 bg-slate-200 dark:bg-slate-700 rounded-full">
                        <div class="h-2 bg-primary rounded-full" style="width: 80%"></div>
                      </div>
                      <span>1600/2000</span>
                    </div>
                  </td>
                  <td class="px-6 py-4">
                    <span class="bg-slate-100 text-slate-800 text-xs font-medium mr-2 px-2.5 py-0.5 rounded-full dark:bg-slate-700 dark:text-slate-300">Primera compra</span>
                  </td>
                  <td class="px-6 py-4 text-right">
                    <div class="flex items-center justify-end gap-1">
                      <button class="p-2 rounded-md hover:bg-slate-100 dark:hover:bg-slate-800 text-slate-500 dark:text-slate-400 hover:text-primary" on:click={openDetailsSidebar}><span class="material-symbols-outlined text-lg">edit</span></button>
                      <button class="p-2 rounded-md hover:bg-slate-100 dark:hover:bg-slate-800 text-slate-500 dark:text-slate-400 hover:text-primary"><span class="material-symbols-outlined text-lg">content_copy</span></button>
                      <button class="p-2 rounded-md hover:bg-slate-100 dark:hover:bg-slate-800 text-slate-500 dark:text-slate-400 hover:text-red-500"><span class="material-symbols-outlined text-lg">delete</span></button>
                    </div>
                  </td>
                </tr>
                <tr class="border-b dark:border-slate-800 hover:bg-slate-50 dark:hover:bg-slate-800/50">
                  <td class="px-6 py-4">
                    <div class="group relative flex items-center">
                      <span class="font-mono bg-slate-100 dark:bg-slate-800 text-slate-700 dark:text-slate-300 rounded px-2 py-1 text-xs font-semibold">BIENVENIDO10</span>
                      <button class="absolute -right-7 opacity-0 group-hover:opacity-100 transition-opacity text-slate-400 hover:text-primary">
                        <span class="material-symbols-outlined text-base">content_copy</span>
                      </button>
                    </div>
                  </td>
                  <td class="px-6 py-4 font-medium text-slate-900 dark:text-white whitespace-nowrap">Nuevos Usuarios</td>
                  <td class="px-6 py-4"><span class="bg-purple-100 text-purple-800 text-xs font-medium mr-2 px-2.5 py-0.5 rounded-full dark:bg-purple-900 dark:text-purple-300">Monto Fijo</span></td>
                  <td class="px-6 py-4">$10.00</td>
                  <td class="px-6 py-4">01 Ene - 30 Jun</td>
                  <td class="px-6 py-4">
                    <div class="flex items-center gap-2">
                      <div class="w-20 h-2 bg-slate-200 dark:bg-slate-700 rounded-full">
                        <div class="h-2 bg-primary rounded-full" style="width: 15%"></div>
                      </div>
                      <span>75/500</span>
                    </div>
                  </td>
                  <td class="px-6 py-4">
                    <span class="bg-slate-100 text-slate-800 text-xs font-medium mr-2 px-2.5 py-0.5 rounded-full dark:bg-slate-700 dark:text-slate-300">Nuevos usuarios</span>
                  </td>
                  <td class="px-6 py-4 text-right">
                    <div class="flex items-center justify-end gap-1">
                      <button class="p-2 rounded-md hover:bg-slate-100 dark:hover:bg-slate-800 text-slate-500 dark:text-slate-400 hover:text-primary"><span class="material-symbols-outlined text-lg">edit</span></button>
                      <button class="p-2 rounded-md hover:bg-slate-100 dark:hover:bg-slate-800 text-slate-500 dark:text-slate-400 hover:text-primary"><span class="material-symbols-outlined text-lg">content_copy</span></button>
                      <button class="p-2 rounded-md hover:bg-slate-100 dark:hover:bg-slate-800 text-slate-500 dark:text-slate-400 hover:text-red-500"><span class="material-symbols-outlined text-lg">delete</span></button>
                    </div>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
          <!-- Pagination -->
          <nav aria-label="Table navigation" class="flex items-center justify-between p-4">
            <span class="text-sm font-normal text-slate-500 dark:text-slate-400">Mostrando <span class="font-semibold text-slate-900 dark:text-white">1-10</span> de <span class="font-semibold text-slate-900 dark:text-white">1000</span></span>
            <ul class="inline-flex items-center -space-x-px">
              <li>
                <a class="block px-3 py-2 ml-0 leading-tight text-slate-500 bg-white border border-slate-300 rounded-l-lg hover:bg-slate-100 hover:text-slate-700 dark:bg-slate-800 dark:border-slate-700 dark:text-slate-400 dark:hover:bg-slate-700 dark:hover:text-white" href="#">
                  <span class="sr-only">Previous</span>
                  <span class="material-symbols-outlined text-lg">chevron_left</span>
                </a>
              </li>
              <li>
                <a class="px-3 py-2 leading-tight text-slate-500 bg-white border border-slate-300 hover:bg-slate-100 hover:text-slate-700 dark:bg-slate-800 dark:border-slate-700 dark:text-slate-400 dark:hover:bg-slate-700 dark:hover:text-white" href="#">1</a>
              </li>
              <li>
                <a class="px-3 py-2 leading-tight text-slate-500 bg-white border border-slate-300 hover:bg-slate-100 hover:text-slate-700 dark:bg-slate-800 dark:border-slate-700 dark:text-slate-400 dark:hover:bg-slate-700 dark:hover:text-white" href="#">2</a>
              </li>
              <li>
                <a aria-current="page" class="z-10 px-3 py-2 leading-tight text-primary bg-primary/10 border border-primary hover:bg-primary/20 dark:border-primary dark:bg-primary/20 dark:text-white" href="#">3</a>
              </li>
              <li>
                <a class="px-3 py-2 leading-tight text-slate-500 bg-white border border-slate-300 hover:bg-slate-100 hover:text-slate-700 dark:bg-slate-800 dark:border-slate-700 dark:text-slate-400 dark:hover:bg-slate-700 dark:hover:text-white" href="#">...</a>
              </li>
              <li>
                <a class="px-3 py-2 leading-tight text-slate-500 bg-white border border-slate-300 hover:bg-slate-100 hover:text-slate-700 dark:bg-slate-800 dark:border-slate-700 dark:text-slate-400 dark:hover:bg-slate-700 dark:hover:text-white" href="#">100</a>
              </li>
              <li>
                <a class="block px-3 py-2 leading-tight text-slate-500 bg-white border border-slate-300 rounded-r-lg hover:bg-slate-100 hover:text-slate-700 dark:bg-slate-800 dark:border-slate-700 dark:text-slate-400 dark:hover:bg-slate-700 dark:hover:text-white" href="#">
                  <span class="sr-only">Next</span>
                  <span class="material-symbols-outlined text-lg">chevron_right</span>
                </a>
              </li>
            </ul>
          </nav>
        </div>
      </div>
    </main>
  </div>
</div>

<!-- Create Coupon Sidebar Overlay -->
{#if isCreateSidebarOpen}
  <div class="fixed inset-0 bg-black/50 z-40 transition-opacity" on:click={closeCreateSidebar} on:keydown={(e) => e.key === 'Escape' && closeCreateSidebar()} role="button" tabindex="0"></div>
{/if}

<!-- Create Coupon Sidebar -->
<div class="fixed inset-y-0 right-0 flex max-w-full z-50 transform transition-transform duration-300 ease-in-out {isCreateSidebarOpen ? 'translate-x-0' : 'translate-x-full'}">
  <div class="relative w-screen max-w-2xl">
    <div class="flex h-full flex-col overflow-y-scroll bg-white dark:bg-[#18212a] shadow-xl">
      <div class="flex-1">
        <!-- Header -->
        <div class="bg-white dark:bg-[#18212a] px-4 py-6 sm:px-6 sticky top-0 z-10 border-b border-gray-200 dark:border-gray-700">
          <div class="flex items-start justify-between space-x-3">
            <div class="space-y-1">
              <h2 class="text-lg font-medium text-gray-900 dark:text-white">Crear Nuevo Cupón</h2>
              <p class="text-sm text-gray-500 dark:text-gray-400">Completa los detalles para generar un cupón de descuento.</p>
            </div>
            <button class="p-2 rounded-full hover:bg-gray-100 dark:hover:bg-gray-800 text-gray-500 dark:text-gray-400" on:click={closeCreateSidebar}>
              <span class="material-symbols-outlined">close</span>
            </button>
          </div>
        </div>
        <!-- Content -->
        <div class="space-y-6 sm:space-y-0 sm:divide-y sm:divide-gray-200 dark:sm:divide-gray-700 p-4 sm:p-6">
          <!-- Preview Section -->
          <div>
            <h3 class="text-[#111418] dark:text-white text-lg font-bold leading-tight tracking-[-0.015em] pb-2 pt-4">Vista Previa del Cupón</h3>
            <div class="p-4 @container">
              <div class="flex flex-col items-stretch justify-start rounded-lg @xl:flex-row @xl:items-start shadow-[0_0_4px_rgba(0,0,0,0.1)] bg-white dark:bg-background-dark border border-gray-200 dark:border-gray-700">
                <div class="w-full @xl:w-1/3 bg-center bg-no-repeat aspect-video @xl:aspect-square bg-cover rounded-t-lg @xl:rounded-l-lg @xl:rounded-tr-none" data-alt="Abstract blue and purple geometric pattern for a coupon background" style="background-image: url('https://lh3.googleusercontent.com/aida-public/AB6AXuBpGpAgrEW_nM_6EzEg1yZVMIS1BSjv8raBBj8zFGXUJc1Z1AQY_EhOVCv185OboKqK8d2-oqaPdprh6QdAovpbacsSfzI5ePhCzFIyadK9zoKx4wea2vNRaTspmBX34AC2kTTu8qJVkYeC3OVpDa-VdWxeo3acR0zKlRpwVyU5jmkZTicDthaX7hDWUNF_zW8_GkEvahENL23fBBBzuMdyGiBcqEv42wlgEgRUcZF5TRAGHoBCtpVcN3YY9Qd42igBs66wkiF4E74');"></div>
                <div class="flex w-full min-w-72 grow flex-col items-stretch justify-center gap-1 py-4 @xl:px-4 px-4">
                  <p class="text-[#617589] dark:text-gray-400 text-sm font-normal leading-normal">Promo Lanzamiento RTX 4070</p>
                  <p class="text-[#111418] dark:text-white text-2xl font-bold leading-tight tracking-[-0.015em]">PROMO4070</p>
                  <div class="flex items-end gap-3 justify-between mt-2">
                    <div class="flex flex-col gap-1">
                      <p class="text-[#111418] dark:text-gray-300 text-base font-medium leading-normal">20% de descuento en tu próxima compra.</p>
                      <p class="text-[#617589] dark:text-gray-400 text-sm font-normal leading-normal">Válido hasta 31/12/2024</p>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
          <!-- Basic Info Section -->
          <div class="pt-6">
            <h3 class="text-[#111418] dark:text-white text-lg font-bold leading-tight tracking-[-0.015em] pb-4">Información Básica</h3>
            <div class="grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
              <div class="sm:col-span-4">
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300" for="coupon-code">Código del Cupón</label>
                <div class="mt-1 flex rounded-md shadow-sm">
                  <input class="form-input block w-full min-w-0 flex-1 rounded-none rounded-l-md border-gray-300 dark:border-gray-600 dark:bg-gray-700 dark:text-white focus:border-primary focus:ring-primary sm:text-sm" id="coupon-code" name="coupon-code" placeholder="EJ: PROMO4070" type="text"/>
                  <button class="relative -ml-px inline-flex items-center space-x-2 rounded-r-md border border-gray-300 dark:border-gray-600 bg-gray-50 dark:bg-gray-700 px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-600 focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary" type="button">
                    <span class="material-symbols-outlined text-base">auto_awesome</span>
                    <span>Generar</span>
                  </button>
                </div>
              </div>
              <div class="sm:col-span-6">
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300" for="coupon-name">Nombre del Cupón</label>
                <div class="mt-1">
                  <input class="form-input shadow-sm block w-full sm:text-sm border-gray-300 dark:border-gray-600 dark:bg-gray-700 dark:text-white rounded-md focus:border-primary focus:ring-primary" id="coupon-name" name="coupon-name" placeholder="Promo Lanzamiento RTX 4070" type="text"/>
                </div>
              </div>
              <div class="sm:col-span-6">
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300" for="description">Descripción (opcional)</label>
                <div class="mt-1">
                  <textarea class="form-textarea shadow-sm block w-full sm:text-sm border-gray-300 dark:border-gray-600 dark:bg-gray-700 dark:text-white rounded-md focus:border-primary focus:ring-primary" id="description" name="description" placeholder="20% de descuento en tu próxima compra." rows="3"></textarea>
                </div>
              </div>
            </div>
          </div>
          <!-- Type and Value Section -->
          <div class="pt-6">
            <h3 class="text-[#111418] dark:text-white text-lg font-bold leading-tight tracking-[-0.015em] pb-4">Tipo y Valor</h3>
            <div class="space-y-4">
              <div>
                <label class="text-sm font-medium text-gray-700 dark:text-gray-300">Tipo de Descuento</label>
                <fieldset class="mt-2">
                  <legend class="sr-only">Tipo de Descuento</legend>
                  <div class="grid grid-cols-3 gap-3">
                    <label class="border rounded-md py-3 px-3 flex items-center justify-center text-sm font-medium sm:flex-1 cursor-pointer focus:outline-none ring-2 ring-primary border-transparent bg-primary text-white">
                      <input checked="" class="sr-only" name="discount-type" type="radio" value="percentage"/>
                      <span>Porcentaje</span>
                    </label>
                    <label class="border rounded-md py-3 px-3 flex items-center justify-center text-sm font-medium sm:flex-1 cursor-pointer focus:outline-none border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white hover:bg-gray-50 dark:hover:bg-gray-600">
                      <input class="sr-only" name="discount-type" type="radio" value="fixed"/>
                      <span>Monto Fijo</span>
                    </label>
                    <label class="border rounded-md py-3 px-3 flex items-center justify-center text-sm font-medium sm:flex-1 cursor-pointer focus:outline-none border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white hover:bg-gray-50 dark:hover:bg-gray-600">
                      <input class="sr-only" name="discount-type" type="radio" value="free-shipping"/>
                      <span>Envío Gratis</span>
                    </label>
                  </div>
                </fieldset>
              </div>
              <div class="sm:col-span-3">
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300" for="value">Valor</label>
                <div class="mt-1 relative rounded-md shadow-sm">
                  <input class="form-input block w-full sm:text-sm border-gray-300 dark:border-gray-600 dark:bg-gray-700 dark:text-white rounded-md pr-12 focus:border-primary focus:ring-primary" id="value" name="value" placeholder="20" type="number"/>
                  <div class="absolute inset-y-0 right-0 flex items-center">
                    <span class="text-gray-500 dark:text-gray-400 sm:text-sm pr-4">%</span>
                  </div>
                </div>
              </div>
            </div>
          </div>
          <!-- Applicability Section -->
          <div class="pt-6">
            <h3 class="text-[#111418] dark:text-white text-lg font-bold leading-tight tracking-[-0.015em] pb-4">Aplicabilidad</h3>
            <div class="grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
              <div class="sm:col-span-3">
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300" for="apply-to">Aplica a</label>
                <select class="form-select mt-1 block w-full pl-3 pr-10 py-2 text-base border-gray-300 dark:border-gray-600 dark:bg-gray-700 dark:text-white focus:outline-none focus:ring-primary focus:border-primary sm:text-sm rounded-md" id="apply-to" name="apply-to">
                  <option>Todo</option>
                  <option selected="">Producto específico</option>
                  <option>Categoría</option>
                  <option>Marca</option>
                </select>
              </div>
              <div class="sm:col-span-3">
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300" for="specific-item">Producto</label>
                <select class="form-select mt-1 block w-full pl-3 pr-10 py-2 text-base border-gray-300 dark:border-gray-600 dark:bg-gray-700 dark:text-white focus:outline-none focus:ring-primary focus:border-primary sm:text-sm rounded-md" id="specific-item" name="specific-item">
                  <option>Nvidia GeForce RTX 4070</option>
                  <option>AMD Ryzen 7 7800X3D</option>
                  <option>Samsung 980 Pro SSD 1TB</option>
                </select>
              </div>
              <div class="sm:col-span-3">
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300" for="min-purchase">Compra Mínima</label>
                <div class="mt-1 relative rounded-md shadow-sm">
                  <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                    <span class="text-gray-500 dark:text-gray-400 sm:text-sm">$</span>
                  </div>
                  <input class="form-input block w-full pl-7 sm:text-sm border-gray-300 dark:border-gray-600 dark:bg-gray-700 dark:text-white rounded-md focus:border-primary focus:ring-primary" id="min-purchase" name="min-purchase" placeholder="0.00" type="number"/>
                </div>
              </div>
            </div>
          </div>
          <!-- Usage Restrictions -->
          <div class="pt-6">
            <h3 class="text-[#111418] dark:text-white text-lg font-bold leading-tight tracking-[-0.015em] pb-4">Restricciones de Uso</h3>
            <div class="grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
              <div class="sm:col-span-3">
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300" for="max-uses-total">Usos máximos totales</label>
                <input class="form-input mt-1 block w-full sm:text-sm border-gray-300 dark:border-gray-600 dark:bg-gray-700 dark:text-white rounded-md focus:border-primary focus:ring-primary" id="max-uses-total" name="max-uses-total" placeholder="Ilimitado" type="number"/>
              </div>
              <div class="sm:col-span-3">
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300" for="max-uses-user">Usos máximos por usuario</label>
                <input class="form-input mt-1 block w-full sm:text-sm border-gray-300 dark:border-gray-600 dark:bg-gray-700 dark:text-white rounded-md focus:border-primary focus:ring-primary" id="max-uses-user" name="max-uses-user" type="number" value="1"/>
              </div>
              <div class="sm:col-span-6 space-y-4 pt-2">
                <div class="relative flex items-start">
                  <div class="flex items-center h-5">
                    <input class="form-checkbox h-4 w-4 text-primary border-gray-300 rounded focus:ring-primary" id="new-users" name="new-users" type="checkbox"/>
                  </div>
                  <div class="ml-3 text-sm">
                    <label class="font-medium text-gray-700 dark:text-gray-300" for="new-users">Solo para nuevos usuarios</label>
                  </div>
                </div>
                <div class="relative flex items-start">
                  <div class="flex items-center h-5">
                    <input class="form-checkbox h-4 w-4 text-primary border-gray-300 rounded focus:ring-primary" id="first-purchase" name="first-purchase" type="checkbox"/>
                  </div>
                  <div class="ml-3 text-sm">
                    <label class="font-medium text-gray-700 dark:text-gray-300" for="first-purchase">Solo para la primera compra
                    </label></div>
                </div>
              </div>
            </div>
          </div>
          <!-- Validity Section -->
          <div class="pt-6">
            <h3 class="text-[#111418] dark:text-white text-lg font-bold leading-tight tracking-[-0.015em] pb-4">Vigencia</h3>
            <div class="grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
              <div class="sm:col-span-3">
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300" for="start-date">Fecha y hora de inicio</label>
                <input class="form-input mt-1 block w-full sm:text-sm border-gray-300 dark:border-gray-600 dark:bg-gray-700 dark:text-white rounded-md focus:border-primary focus:ring-primary" id="start-date" name="start-date" type="datetime-local"/>
              </div>
              <div class="sm:col-span-3">
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300" for="end-date">Fecha y hora de fin</label>
                <input class="form-input mt-1 block w-full sm:text-sm border-gray-300 dark:border-gray-600 dark:bg-gray-700 dark:text-white rounded-md focus:border-primary focus:ring-primary" id="end-date" name="end-date" type="datetime-local"/>
              </div>
              <div class="sm:col-span-6">
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">Presets rápidos</label>
                <div class="mt-2 flex items-center space-x-2">
                  <button class="px-2.5 py-1.5 border border-gray-300 dark:border-gray-600 shadow-sm text-xs font-medium rounded text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600 focus:outline-none focus:ring-1 focus:ring-primary" type="button">1 semana</button>
                  <button class="px-2.5 py-1.5 border border-gray-300 dark:border-gray-600 shadow-sm text-xs font-medium rounded text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600 focus:outline-none focus:ring-1 focus:ring-primary" type="button">1 mes</button>
                  <button class="px-2.5 py-1.5 border border-gray-300 dark:border-gray-600 shadow-sm text-xs font-medium rounded text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600 focus:outline-none focus:ring-1 focus:ring-primary" type="button">3 meses</button>
                </div>
              </div>
            </div>
          </div>
          <!-- Status Toggle -->
          <div class="pt-6">
            <div class="flex items-center justify-between">
              <div>
                <h3 class="text-[#111418] dark:text-white text-lg font-bold leading-tight tracking-[-0.015em]">Estado</h3>
                <p class="text-sm text-gray-500 dark:text-gray-400">Si está inactivo, el cupón no podrá ser utilizado.</p>
              </div>
              <button aria-checked="true" class="bg-primary relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2" role="switch" type="button">
                <span class="pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out translate-x-5"></span>
              </button>
            </div>
          </div>
        </div>
      </div>


      <!-- Actions -->
      <div class="flex-shrink-0 border-t border-gray-200 dark:border-gray-700 px-4 py-5 sm:px-6 sticky bottom-0 bg-white/80 dark:bg-[#18212a]/80 backdrop-blur-sm">
        <div class="flex justify-end space-x-3">
          <button class="rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 py-2 px-4 text-sm font-medium text-gray-700 dark:text-gray-300 shadow-sm hover:bg-gray-50 dark:hover:bg-gray-600 focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2" type="button" on:click={closeCreateSidebar}>Cancelar</button>
          <button class="inline-flex justify-center rounded-md border border-transparent bg-primary py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-primary/90 focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2" type="submit">Guardar Cupón</button>
        </div>
      </div>
    </div>
  </div>
</div>

<!-- Coupon Details Sidebar Overlay -->
{#if isDetailsSidebarOpen}
  <div class="fixed inset-0 bg-black/50 z-40 transition-opacity" on:click={closeDetailsSidebar} on:keydown={(e) => e.key === 'Escape' && closeDetailsSidebar()} role="button" tabindex="0"></div>
{/if}

<!-- Coupon Details Sidebar -->
<div class="fixed inset-y-0 right-0 z-50 flex h-full max-h-screen w-full max-w-2xl transform transition-transform duration-300 ease-in-out {isDetailsSidebarOpen ? 'translate-x-0' : 'translate-x-full'}">
  <div class="relative flex h-full w-full flex-col overflow-y-auto border-l border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900 shadow-xl">
    <header class="sticky top-0 z-10 flex items-center justify-between border-b border-slate-200 dark:border-slate-800 bg-white/75 dark:bg-slate-900/75 px-6 py-4 backdrop-blur-sm">
      <div class="flex items-center gap-3">
        <div class="bg-center bg-no-repeat aspect-square bg-cover rounded-full size-10" data-alt="Abstract gradient with blue and purple tones" style='background-image: url("https://lh3.googleusercontent.com/aida-public/AB6AXuAKy7o-j0Gc29GqrXlNRmdb7kPHtlamRGnhORJJDJueeYkNnwd6q16Dz5N0tc1qMvAyLGVs2epr2w8eQWF-xfSGoMiewI86nvtvplDu4z4J-nX3So6omVi0sjLUCaXD9aZdryPo128kiBTVUNr5kx4N8nGewNwtUFgq-MhUEmzWZR23CRTWgbIR6TAAtFKHenzlv3gcygeO1h1HgP_udchn9rHng7O8RmYdGZ7zJ4G3dpHMXZ53s1aFrR_ipoSszAePmSdAzRrjSaE");'></div>
        <div class="flex flex-col">
          <h1 class="text-slate-900 dark:text-white text-base font-medium leading-normal">CYBERPC25</h1>
          <p class="text-green-600 dark:text-green-400 text-sm font-normal leading-normal">Activo</p>
        </div>
      </div>
      <div class="flex items-center gap-4">
        <button class="flex min-w-[84px] max-w-[480px] cursor-pointer items-center justify-center overflow-hidden rounded-lg h-10 px-4 bg-primary text-white text-sm font-bold leading-normal tracking-[0.015em]">
          <span class="truncate">Editar Cupón</span>
        </button>
        <button class="text-slate-500 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-slate-800 rounded-full p-2" on:click={closeDetailsSidebar}>
          <span class="material-symbols-outlined">close</span>
        </button>
      </div>
    </header>
    <div class="flex-1 p-6">
      <div class="flex flex-col gap-8">
        <!-- Navigation -->
        <div class="border-b border-slate-200 dark:border-slate-800">
          <nav aria-label="Tabs" class="flex -mb-px gap-6">
            <button class="flex items-center gap-2 px-1 py-3 text-sm font-medium border-b-2 {activeTab === 'info' ? 'border-primary text-primary' : 'border-transparent text-slate-500 dark:text-slate-400 hover:text-slate-700 dark:hover:text-slate-200 hover:border-slate-300 dark:hover:border-slate-700'}" on:click={() => activeTab = 'info'}>
              <span class="material-symbols-outlined !text-xl" style="font-variation-settings: 'FILL' {activeTab === 'info' ? 1 : 0};">info</span>
              Información General
            </button>
            <button class="flex items-center gap-2 px-1 py-3 text-sm font-medium border-b-2 {activeTab === 'stats' ? 'border-primary text-primary' : 'border-transparent text-slate-500 dark:text-slate-400 hover:text-slate-700 dark:hover:text-slate-200 hover:border-slate-300 dark:hover:border-slate-700'}" on:click={() => activeTab = 'stats'}>
              <span class="material-symbols-outlined !text-xl" style="font-variation-settings: 'FILL' {activeTab === 'stats' ? 1 : 0};">bar_chart</span>
              Estadísticas
            </button>
            <button class="flex items-center gap-2 px-1 py-3 text-sm font-medium border-b-2 {activeTab === 'history' ? 'border-primary text-primary' : 'border-transparent text-slate-500 dark:text-slate-400 hover:text-slate-700 dark:hover:text-slate-200 hover:border-slate-300 dark:hover:border-slate-700'}" on:click={() => activeTab = 'history'}>
              <span class="material-symbols-outlined !text-xl" style="font-variation-settings: 'FILL' {activeTab === 'history' ? 1 : 0};">history</span>
              Actividad
            </button>
            <button class="flex items-center gap-2 px-1 py-3 text-sm font-medium border-b-2 {activeTab === 'settings' ? 'border-primary text-primary' : 'border-transparent text-slate-500 dark:text-slate-400 hover:text-slate-700 dark:hover:text-slate-200 hover:border-slate-300 dark:hover:border-slate-700'}" on:click={() => activeTab = 'settings'}>
              <span class="material-symbols-outlined !text-xl" style="font-variation-settings: 'FILL' {activeTab === 'settings' ? 1 : 0};">settings</span>
              Configuración
            </button>
          </nav>
        </div>
        <!-- Information Section -->
        {#if activeTab === 'info'}
        <section class="grid grid-cols-1 lg:grid-cols-3 gap-8">
          <div class="lg:col-span-2">
            <div class="grid grid-cols-[30%_1fr] gap-x-6 text-sm">
              <div class="col-span-2 grid grid-cols-subgrid border-b border-b-slate-200 dark:border-b-slate-800 py-4">
                <p class="text-slate-500 dark:text-slate-400 font-normal leading-normal">Código</p>
                <p class="text-slate-900 dark:text-slate-50 font-normal leading-normal">CYBERPC25</p>
              </div>
              <div class="col-span-2 grid grid-cols-subgrid border-b border-b-slate-200 dark:border-b-slate-800 py-4">
                <p class="text-slate-500 dark:text-slate-400 font-normal leading-normal">Nombre</p>
                <p class="text-slate-900 dark:text-slate-50 font-normal leading-normal">Cupón Cyber Week</p>
              </div>
              <div class="col-span-2 grid grid-cols-subgrid border-b border-b-slate-200 dark:border-b-slate-800 py-4">
                <p class="text-slate-500 dark:text-slate-400 font-normal leading-normal">Descripción</p>
                <p class="text-slate-900 dark:text-slate-50 font-normal leading-normal">25% de descuento en componentes seleccionados.</p>
              </div>
              <div class="col-span-2 grid grid-cols-subgrid border-b border-b-slate-200 dark:border-b-slate-800 py-4">
                <p class="text-slate-500 dark:text-slate-400 font-normal leading-normal">Tipo</p>
                <p class="text-slate-900 dark:text-slate-50 font-normal leading-normal">Porcentaje</p>
              </div>
              <div class="col-span-2 grid grid-cols-subgrid border-b border-b-slate-200 dark:border-b-slate-800 py-4">
                <p class="text-slate-500 dark:text-slate-400 font-normal leading-normal">Valor</p>
                <p class="text-slate-900 dark:text-slate-50 font-normal leading-normal">25%</p>
              </div>
              <div class="col-span-2 grid grid-cols-subgrid border-b border-b-slate-200 dark:border-b-slate-800 py-4">
                <p class="text-slate-500 dark:text-slate-400 font-normal leading-normal">Aplicabilidad</p>
                <p class="text-slate-900 dark:text-slate-50 font-normal leading-normal">Productos seleccionados</p>
              </div>
              <div class="col-span-2 grid grid-cols-subgrid border-b border-b-slate-200 dark:border-b-slate-800 py-4">
                <p class="text-slate-500 dark:text-slate-400 font-normal leading-normal">Restricciones</p>
                <p class="text-slate-900 dark:text-slate-50 font-normal leading-normal">Uso único por cliente</p>
              </div>
              <div class="col-span-2 grid grid-cols-subgrid py-4">
                <p class="text-slate-500 dark:text-slate-400 font-normal leading-normal">Vigencia</p>
                <p class="text-slate-900 dark:text-slate-50 font-normal leading-normal">01/11/2024 - 07/11/2024</p>
              </div>
            </div>
          </div>
          <div class="lg:col-span-1">
            <div class="flex flex-col gap-4">
              <h3 class="text-slate-900 dark:text-white text-base font-bold leading-tight">Vista Previa del Cupón</h3>
              <div class="flex flex-col items-stretch justify-start rounded-lg shadow-[0_0_4px_rgba(0,0,0,0.1)] bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700">
                <div class="w-full bg-center bg-no-repeat aspect-video bg-cover rounded-t-lg" data-alt="Promotional image for PC components with a blue color scheme" style='background-image: url("https://lh3.googleusercontent.com/aida-public/AB6AXuApVkzc24ObnT-DZYQ1O6n9mRX_uc0bDaymtjmfE7q00HhFDsoBAbeSkWt2zYaGVoZnWFazUivJ5XEhJAY5LMbbWSDCnEblwVGuJ0F8mfY6dMLpE9_1i_hHb6m_6P5EoiCSMEoN3YpnsRtwwYZdIjRYcHdvvuSe00qUtit5cNzlISeDLFU6jh--fDZ_E-Se-Mw5f0nyb-eZ1B80ug7UvQX2gG2vCT2eWTOENA4_G60nEPYQy6PQr1iJAonrQXBNRjq88Lt3c8E6DM8");'></div>
                <div class="flex w-full min-w-0 grow flex-col items-stretch justify-center gap-2 p-4">
                  <p class="text-slate-900 dark:text-white text-lg font-bold leading-tight tracking-[-0.015em]">25% OFF en Componentes</p>
                  <div class="flex flex-col gap-3">
                    <div class="flex flex-col gap-1">
                      <p class="text-slate-500 dark:text-slate-400 text-sm font-normal leading-normal">Usa el código CYBERPC25 al finalizar tu compra.</p>
                      <p class="text-slate-500 dark:text-slate-400 text-sm font-normal leading-normal">Válido hasta el 07/11/2024</p>
                    </div>
                    <button class="flex min-w-[84px] max-w-[480px] cursor-pointer items-center justify-center overflow-hidden rounded-lg h-8 px-4 bg-primary/20 text-primary text-sm font-medium leading-normal">
                      <span class="truncate">Copiar Código</span>
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </section>
        <!-- Stats Section (Original) -->
        <section>
          <h3 class="text-slate-900 dark:text-white text-lg font-bold leading-tight tracking-[-0.015em] pb-4">Estadísticas</h3>
          <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
            <div class="rounded-xl border border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900 p-4">
              <p class="text-sm text-slate-500 dark:text-slate-400">Usos</p>
              <p class="text-2xl font-bold text-slate-900 dark:text-white">850 / 1000</p>
              <p class="text-xs text-slate-400 dark:text-slate-500">Totales vs. Disponibles</p>
            </div>
            <div class="rounded-xl border border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900 p-4">
              <p class="text-sm text-slate-500 dark:text-slate-400">Descuento Total</p>
              <p class="text-2xl font-bold text-slate-900 dark:text-white">$12,750.00</p>
              <p class="text-xs text-slate-400 dark:text-slate-500">Monto otorgado</p>
            </div>
            <div class="rounded-xl border border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900 p-4">
              <p class="text-sm text-slate-500 dark:text-slate-400">Conversión</p>
              <p class="text-2xl font-bold text-slate-900 dark:text-white">92.5%</p>
              <p class="text-xs text-slate-400 dark:text-slate-500">Usos que generaron venta</p>
            </div>
            <div class="rounded-xl border border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900 p-4">
              <p class="text-sm text-slate-500 dark:text-slate-400">Valor Promedio</p>
              <p class="text-2xl font-bold text-slate-900 dark:text-white">$15.00</p>
              <p class="text-xs text-slate-400 dark:text-slate-500">Descuento por uso</p>
            </div>
          </div>
        </section>
        {:else if activeTab === 'stats'}
        <section class="flex flex-col gap-6">
          <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
            <div class="rounded-xl border border-slate-200 dark:border-slate-800 bg-slate-50 dark:bg-slate-800/50 p-4">
              <p class="text-sm text-slate-500 dark:text-slate-400">Descuento total otorgado</p>
              <p class="text-2xl font-bold text-slate-900 dark:text-white">$12,750.00</p>
            </div>
            <div class="rounded-xl border border-slate-200 dark:border-slate-800 bg-slate-50 dark:bg-slate-800/50 p-4">
              <p class="text-sm text-slate-500 dark:text-slate-400">Conversión</p>
              <p class="text-2xl font-bold text-slate-900 dark:text-white">92.5%</p>
              <p class="text-xs text-slate-400 dark:text-slate-500">% de usos que generaron venta</p>
            </div>
          </div>
          <div class="rounded-xl border border-slate-200 dark:border-slate-800 bg-slate-50 dark:bg-slate-800/50 p-4 flex flex-col gap-3">
            <div class="flex justify-between items-baseline">
              <h3 class="text-base font-medium text-slate-900 dark:text-white">Usos totales vs disponibles</h3>
              <p class="text-sm font-bold text-slate-900 dark:text-white">850 <span class="font-normal text-slate-500 dark:text-slate-400">/ 1000</span></p>
            </div>
            <div class="w-full bg-slate-200 dark:bg-slate-700 rounded-full h-2.5">
              <div class="bg-primary h-2.5 rounded-full" style="width: 85%"></div>
            </div>
            <p class="text-sm text-slate-500 dark:text-slate-400">Quedan 150 usos disponibles.</p>
          </div>
          <div class="rounded-xl border border-slate-200 dark:border-slate-800 bg-slate-50 dark:bg-slate-800/50 p-4 flex flex-col gap-4">
            <h3 class="text-base font-medium text-slate-900 dark:text-white">Top 10 Usuarios</h3>
            <div class="flex flex-col gap-3">
              <div class="grid grid-cols-[1fr_auto] items-center gap-4">
                <div class="flex items-center gap-3">
                  <p class="w-6 text-sm text-slate-500 dark:text-slate-400">1.</p>
                  <p class="text-sm font-medium text-slate-700 dark:text-slate-200">Ana García</p>
                </div>
                <div class="flex items-center gap-2">
                  <div class="w-48 bg-slate-200 dark:bg-slate-700 rounded-full h-2">
                    <div class="bg-primary h-2 rounded-full" style="width: 100%"></div>
                  </div>
                  <p class="w-8 text-right text-sm font-semibold text-slate-900 dark:text-white">12</p>
                </div>
              </div>
              <div class="grid grid-cols-[1fr_auto] items-center gap-4">
                <div class="flex items-center gap-3">
                  <p class="w-6 text-sm text-slate-500 dark:text-slate-400">2.</p>
                  <p class="text-sm font-medium text-slate-700 dark:text-slate-200">Carlos Rodriguez</p>
                </div>
                <div class="flex items-center gap-2">
                  <div class="w-48 bg-slate-200 dark:bg-slate-700 rounded-full h-2">
                    <div class="bg-primary h-2 rounded-full" style="width: 91.6%"></div>
                  </div>
                  <p class="w-8 text-right text-sm font-semibold text-slate-900 dark:text-white">11</p>
                </div>
              </div>
              <div class="grid grid-cols-[1fr_auto] items-center gap-4">
                <div class="flex items-center gap-3">
                  <p class="w-6 text-sm text-slate-500 dark:text-slate-400">3.</p>
                  <p class="text-sm font-medium text-slate-700 dark:text-slate-200">Luisa Martínez</p>
                </div>
                <div class="flex items-center gap-2">
                  <div class="w-48 bg-slate-200 dark:bg-slate-700 rounded-full h-2">
                    <div class="bg-primary h-2 rounded-full" style="width: 75%"></div>
                  </div>
                  <p class="w-8 text-right text-sm font-semibold text-slate-900 dark:text-white">9</p>
                </div>
              </div>
              <div class="grid grid-cols-[1fr_auto] items-center gap-4">
                <div class="flex items-center gap-3">
                  <p class="w-6 text-sm text-slate-500 dark:text-slate-400">4.</p>
                  <p class="text-sm font-medium text-slate-700 dark:text-slate-200">Javier López</p>
                </div>
                <div class="flex items-center gap-2">
                  <div class="w-48 bg-slate-200 dark:bg-slate-700 rounded-full h-2">
                    <div class="bg-primary h-2 rounded-full" style="width: 66.6%"></div>
                  </div>
                  <p class="w-8 text-right text-sm font-semibold text-slate-900 dark:text-white">8</p>
                </div>
              </div>
              <div class="grid grid-cols-[1fr_auto] items-center gap-4">
                <div class="flex items-center gap-3">
                  <p class="w-6 text-sm text-slate-500 dark:text-slate-400">5.</p>
                  <p class="text-sm font-medium text-slate-700 dark:text-slate-200">Sofía Hernández</p>
                </div>
                <div class="flex items-center gap-2">
                  <div class="w-48 bg-slate-200 dark:bg-slate-700 rounded-full h-2">
                    <div class="bg-primary h-2 rounded-full" style="width: 58.3%"></div>
                  </div>
                  <p class="w-8 text-right text-sm font-semibold text-slate-900 dark:text-white">7</p>
                </div>
              </div>
              <div class="grid grid-cols-[1fr_auto] items-center gap-4">
                <div class="flex items-center gap-3">
                  <p class="w-6 text-sm text-slate-500 dark:text-slate-400">6.</p>
                  <p class="text-sm font-medium text-slate-700 dark:text-slate-200">user_84321</p>
                </div>
                <div class="flex items-center gap-2">
                  <div class="w-48 bg-slate-200 dark:bg-slate-700 rounded-full h-2">
                    <div class="bg-primary h-2 rounded-full" style="width: 50%"></div>
                  </div>
                  <p class="w-8 text-right text-sm font-semibold text-slate-900 dark:text-white">6</p>
                </div>
              </div>
              <div class="grid grid-cols-[1fr_auto] items-center gap-4">
                <div class="flex items-center gap-3">
                  <p class="w-6 text-sm text-slate-500 dark:text-slate-400">7.</p>
                  <p class="text-sm font-medium text-slate-700 dark:text-slate-200">Pedro Pérez</p>
                </div>
                <div class="flex items-center gap-2">
                  <div class="w-48 bg-slate-200 dark:bg-slate-700 rounded-full h-2">
                    <div class="bg-primary h-2 rounded-full" style="width: 41.6%"></div>
                  </div>
                  <p class="w-8 text-right text-sm font-semibold text-slate-900 dark:text-white">5</p>
                </div>
              </div>
              <div class="grid grid-cols-[1fr_auto] items-center gap-4">
                <div class="flex items-center gap-3">
                  <p class="w-6 text-sm text-slate-500 dark:text-slate-400">8.</p>
                  <p class="text-sm font-medium text-slate-700 dark:text-slate-200">user_55678</p>
                </div>
                <div class="flex items-center gap-2">
                  <div class="w-48 bg-slate-200 dark:bg-slate-700 rounded-full h-2">
                    <div class="bg-primary h-2 rounded-full" style="width: 41.6%"></div>
                  </div>
                  <p class="w-8 text-right text-sm font-semibold text-slate-900 dark:text-white">5</p>
                </div>
              </div>
              <div class="grid grid-cols-[1fr_auto] items-center gap-4">
                <div class="flex items-center gap-3">
                  <p class="w-6 text-sm text-slate-500 dark:text-slate-400">9.</p>
                  <p class="text-sm font-medium text-slate-700 dark:text-slate-200">Laura Gómez</p>
                </div>
                <div class="flex items-center gap-2">
                  <div class="w-48 bg-slate-200 dark:bg-slate-700 rounded-full h-2">
                    <div class="bg-primary h-2 rounded-full" style="width: 33.3%"></div>
                  </div>
                  <p class="w-8 text-right text-sm font-semibold text-slate-900 dark:text-white">4</p>
                </div>
              </div>
              <div class="grid grid-cols-[1fr_auto] items-center gap-4">
                <div class="flex items-center gap-3">
                  <p class="w-6 text-sm text-slate-500 dark:text-slate-400">10.</p>
                  <p class="text-sm font-medium text-slate-700 dark:text-slate-200">user_99012</p>
                </div>
                <div class="flex items-center gap-2">
                  <div class="w-48 bg-slate-200 dark:bg-slate-700 rounded-full h-2">
                    <div class="bg-primary h-2 rounded-full" style="width: 33.3%"></div>
                  </div>
                  <p class="w-8 text-right text-sm font-semibold text-slate-900 dark:text-white">4</p>
                </div>
              </div>
            </div>
          </div>
        </section>
        {:else if activeTab === 'history'}
        <section class="flex flex-col gap-8">
          <div>
            <h3 class="text-slate-900 dark:text-white text-lg font-bold leading-tight tracking-[-0.015em] mb-4">Historial de usos recientes</h3>
            <div class="overflow-x-auto rounded-lg border border-slate-200 dark:border-slate-800">
              <table class="w-full text-sm text-left text-slate-500 dark:text-slate-400">
                <thead class="text-xs text-slate-700 dark:text-slate-300 uppercase bg-slate-50 dark:bg-slate-800">
                  <tr>
                    <th class="px-6 py-3" scope="col">Fecha / Hora</th>
                    <th class="px-6 py-3" scope="col">ID Usuario</th>
                    <th class="px-6 py-3" scope="col">ID Pedido</th>
                    <th class="px-6 py-3" scope="col">Descuento</th>
                    <th class="px-6 py-3" scope="col">Estado Pedido</th>
                  </tr>
                </thead>
                <tbody>
                  <tr class="bg-white dark:bg-slate-900 border-b dark:border-slate-800">
                    <td class="px-6 py-4">07/11/2024, 14:30</td>
                    <td class="px-6 py-4 font-medium text-slate-900 dark:text-white">USR-G5H9</td>
                    <td class="px-6 py-4">ORD-K2L4</td>
                    <td class="px-6 py-4">$25.50</td>
                    <td class="px-6 py-4">
                      <span class="inline-flex items-center px-2 py-1 text-xs font-medium text-green-700 bg-green-100 rounded-full dark:bg-green-900 dark:text-green-300">Completado</span>
                    </td>
                  </tr>
                  <tr class="bg-slate-50 dark:bg-slate-900/50 border-b dark:border-slate-800">
                    <td class="px-6 py-4">07/11/2024, 11:15</td>
                    <td class="px-6 py-4 font-medium text-slate-900 dark:text-white">USR-J8P1</td>
                    <td class="px-6 py-4">ORD-B7N3</td>
                    <td class="px-6 py-4">$18.75</td>
                    <td class="px-6 py-4">
                      <span class="inline-flex items-center px-2 py-1 text-xs font-medium text-green-700 bg-green-100 rounded-full dark:bg-green-900 dark:text-green-300">Completado</span>
                    </td>
                  </tr>
                  <tr class="bg-white dark:bg-slate-900 border-b dark:border-slate-800">
                    <td class="px-6 py-4">06/11/2024, 20:45</td>
                    <td class="px-6 py-4 font-medium text-slate-900 dark:text-white">USR-M4C6</td>
                    <td class="px-6 py-4">ORD-X9V5</td>
                    <td class="px-6 py-4">$32.00</td>
                    <td class="px-6 py-4">
                      <span class="inline-flex items-center px-2 py-1 text-xs font-medium text-yellow-700 bg-yellow-100 rounded-full dark:bg-yellow-900 dark:text-yellow-300">Pendiente</span>
                    </td>
                  </tr>
                  <tr class="bg-slate-50 dark:bg-slate-900/50 border-b dark:border-slate-800">
                    <td class="px-6 py-4">06/11/2024, 18:02</td>
                    <td class="px-6 py-4 font-medium text-slate-900 dark:text-white">USR-F2R9</td>
                    <td class="px-6 py-4">ORD-Z1T8</td>
                    <td class="px-6 py-4">$15.20</td>
                    <td class="px-6 py-4">
                      <span class="inline-flex items-center px-2 py-1 text-xs font-medium text-red-700 bg-red-100 rounded-full dark:bg-red-900 dark:text-red-300">Cancelado</span>
                    </td>
                  </tr>
                  <tr class="bg-white dark:bg-slate-900">
                    <td class="px-6 py-4">05/11/2024, 09:55</td>
                    <td class="px-6 py-4 font-medium text-slate-900 dark:text-white">USR-A3S7</td>
                    <td class="px-6 py-4">ORD-Q6W2</td>
                    <td class="px-6 py-4">$45.10</td>
                    <td class="px-6 py-4">
                      <span class="inline-flex items-center px-2 py-1 text-xs font-medium text-green-700 bg-green-100 rounded-full dark:bg-green-900 dark:text-green-300">Completado</span>
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
          <div>
            <h3 class="text-slate-900 dark:text-white text-lg font-bold leading-tight tracking-[-0.015em] mb-4">Timeline de eventos</h3>
            <div class="relative pl-6 border-l-2 border-slate-200 dark:border-slate-700">
              <div class="mb-8">
                <div class="absolute w-4 h-4 bg-primary rounded-full -left-[9px] mt-1.5 border-2 border-white dark:border-slate-900"></div>
                <time class="mb-1 text-xs font-normal text-slate-400 dark:text-slate-500">07/11/2024, 15:00</time>
                <p class="text-sm font-medium text-slate-900 dark:text-white">Cupón desactivado</p>
                <p class="text-sm text-slate-500 dark:text-slate-400">Desactivado por <span class="font-semibold text-slate-700 dark:text-slate-300">Admin</span>. Razón: Fin de la promoción.</p>
              </div>
              <div class="mb-8">
                <div class="absolute w-4 h-4 bg-primary/50 rounded-full -left-[9px] mt-1.5 border-2 border-white dark:border-slate-900"></div>
                <time class="mb-1 text-xs font-normal text-slate-400 dark:text-slate-500">05/11/2024, 10:00</time>
                <p class="text-sm font-medium text-slate-900 dark:text-white">Uso N° 500 alcanzado</p>
                <p class="text-sm text-slate-500 dark:text-slate-400">El cupón ha sido utilizado 500 veces.</p>
              </div>
              <div class="mb-8">
                <div class="absolute w-4 h-4 bg-primary/50 rounded-full -left-[9px] mt-1.5 border-2 border-white dark:border-slate-900"></div>
                <time class="mb-1 text-xs font-normal text-slate-400 dark:text-slate-500">01/11/2024, 09:00</time>
                <p class="text-sm font-medium text-slate-900 dark:text-white">Cupón activado</p>
                <p class="text-sm text-slate-500 dark:text-slate-400">Activado por <span class="font-semibold text-slate-700 dark:text-slate-300">Admin</span>.</p>
              </div>
              <div class="mb-8">
                <div class="absolute w-4 h-4 bg-primary/50 rounded-full -left-[9px] mt-1.5 border-2 border-white dark:border-slate-900"></div>
                <time class="mb-1 text-xs font-normal text-slate-400 dark:text-slate-500">31/10/2024, 18:30</time>
                <p class="text-sm font-medium text-slate-900 dark:text-white">Cupón editado</p>
                <p class="text-sm text-slate-500 dark:text-slate-400">Límite de uso cambiado a 1000 por <span class="font-semibold text-slate-700 dark:text-slate-300">Admin</span>.</p>
              </div>
              <div>
                <div class="absolute w-4 h-4 bg-primary/50 rounded-full -left-[9px] mt-1.5 border-2 border-white dark:border-slate-900"></div>
                <time class="mb-1 text-xs font-normal text-slate-400 dark:text-slate-500">30/10/2024, 12:00</time>
                <p class="text-sm font-medium text-slate-900 dark:text-white">Cupón creado</p>
                <p class="text-sm text-slate-500 dark:text-slate-400">Creado por <span class="font-semibold text-slate-700 dark:text-slate-300">Admin</span>.</p>
              </div>
            </div>
          </div>
        </section>
        {:else if activeTab === 'settings'}
        <form class="space-y-6">
          <div class="space-y-2">
            <label class="text-sm font-medium text-slate-700 dark:text-slate-300" for="coupon-name">Nombre del Cupón</label>
            <input class="w-full rounded-md border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-900 text-slate-900 dark:text-slate-50 placeholder:text-slate-400 dark:placeholder:text-slate-500 focus:border-primary focus:ring-primary" id="coupon-name" placeholder="Ej: Descuento de Verano" type="text" value="Cupón Cyber Week"/>
          </div>
          <div class="space-y-2">
            <label class="text-sm font-medium text-slate-700 dark:text-slate-300" for="coupon-description">Descripción</label>
            <textarea class="w-full rounded-md border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-900 text-slate-900 dark:text-slate-50 placeholder:text-slate-400 dark:placeholder:text-slate-500 focus:border-primary focus:ring-primary" id="coupon-description" placeholder="Breve descripción del propósito del cupón" rows="3">25% de descuento en componentes seleccionados.</textarea>
          </div>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div class="space-y-2">
              <label class="text-sm font-medium text-slate-700 dark:text-slate-300" for="coupon-type">Tipo de Cupón</label>
              <select class="w-full rounded-md border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-900 text-slate-900 dark:text-slate-50 focus:border-primary focus:ring-primary" id="coupon-type">
                <option>Monto Fijo</option>
                <option selected="">Porcentaje</option>
                <option>Envío Gratis</option>
              </select>
            </div>
            <div class="space-y-2">
              <label class="text-sm font-medium text-slate-700 dark:text-slate-300" for="coupon-value">Valor</label>
              <div class="relative">
                <input class="w-full rounded-md border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-900 text-slate-900 dark:text-slate-50 placeholder:text-slate-400 dark:placeholder:text-slate-500 focus:border-primary focus:ring-primary pl-7" id="coupon-value" placeholder="25" type="number" value="25"/>
                <span class="absolute left-2.5 top-1/2 -translate-y-1/2 text-slate-400 dark:text-slate-500">%</span>
              </div>
            </div>
          </div>
          <div class="space-y-2">
            <label class="text-sm font-medium text-slate-700 dark:text-slate-300" for="coupon-applicability">Aplicabilidad</label>
            <select class="w-full rounded-md border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-900 text-slate-900 dark:text-slate-50 focus:border-primary focus:ring-primary" id="coupon-applicability">
              <option>Todo el pedido</option>
              <option selected="">Productos seleccionados</option>
              <option>Categorías seleccionadas</option>
            </select>
          </div>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div class="space-y-2">
              <label class="text-sm font-medium text-slate-700 dark:text-slate-300" for="coupon-restrictions">Restricciones de Uso</label>
              <select class="w-full rounded-md border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-900 text-slate-900 dark:text-slate-50 focus:border-primary focus:ring-primary" id="coupon-restrictions">
                <option>Sin límite</option>
                <option selected="">Uso único por cliente</option>
                <option>Límite de usos totales</option>
              </select>
            </div>
            <div class="space-y-2">
              <label class="text-sm font-medium text-slate-700 dark:text-slate-300" for="usage-limit">Límite de usos totales</label>
              <input class="w-full rounded-md border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-900 text-slate-900 dark:text-slate-50 placeholder:text-slate-400 dark:placeholder:text-slate-500 focus:border-primary focus:ring-primary" id="usage-limit" placeholder="1000" type="number" value="1000"/>
            </div>
          </div>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div class="space-y-2">
              <label class="text-sm font-medium text-slate-700 dark:text-slate-300" for="start-date">Fecha de Inicio</label>
              <input class="w-full rounded-md border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-900 text-slate-900 dark:text-slate-50 focus:border-primary focus:ring-primary" id="start-date" type="date" value="2024-11-01"/>
            </div>
            <div class="space-y-2">
              <label class="text-sm font-medium text-slate-700 dark:text-slate-300" for="end-date">Fecha de Fin</label>
              <input class="w-full rounded-md border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-900 text-slate-900 dark:text-slate-50 focus:border-primary focus:ring-primary" id="end-date" type="date" value="2024-11-07"/>
            </div>
          </div>
          <div class="flex items-center space-x-3 pt-2">
            <label class="relative inline-flex items-center cursor-pointer"><input checked="" class="sr-only peer" type="checkbox" value=""/>
              <div class="w-11 h-6 bg-slate-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-primary/30 dark:peer-focus:ring-primary/80 rounded-full peer dark:bg-slate-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-slate-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-slate-600 peer-checked:bg-primary"></div>
            </label>
            <span class="text-sm font-medium text-slate-700 dark:text-slate-300">Cupón Activo</span>
          </div>
          <div class="flex justify-end items-center gap-4 pt-4 border-t border-slate-200 dark:border-slate-800">
            <button class="flex min-w-[84px] max-w-[480px] cursor-pointer items-center justify-center overflow-hidden rounded-lg h-10 px-4 bg-transparent text-slate-700 dark:text-slate-200 text-sm font-bold leading-normal tracking-[0.015em] border border-slate-300 dark:border-slate-700 hover:bg-slate-100 dark:hover:bg-slate-800" type="button">
              <span class="truncate">Cancelar</span>
            </button>
            <button class="flex min-w-[84px] max-w-[480px] cursor-pointer items-center justify-center overflow-hidden rounded-lg h-10 px-4 bg-primary text-white text-sm font-bold leading-normal tracking-[0.015em] hover:bg-primary/90" type="submit">
              <span class="truncate">Guardar Cambios</span>
            </button>
          </div>
        </form>
        {/if}
      </div>
    </div>
  </div>
</div>

<!-- Mass Assignment Modal -->
{#if isMassAssignModalOpen}
  <div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm transition-opacity" on:click={closeMassAssignModal}>
    <div class="relative w-full max-w-4xl rounded-xl bg-white dark:bg-[#1C2A36] shadow-2xl" on:click|stopPropagation>
      <div class="flex h-full grow flex-col">
        <div class="px-10 py-8">
          <div class="flex flex-col flex-1">
            <div class="flex flex-wrap justify-between gap-3 pb-6">
              <p class="text-[#111418] dark:text-white text-4xl font-black leading-tight tracking-[-0.033em] min-w-72">Asignación Masiva de Cupones</p>
            </div>
            <div class="pb-3">
              <div class="flex border-b border-[#dbe0e6] dark:border-[#334155] gap-8">
                <button class="flex flex-col items-center justify-center border-b-[3px] {massAssignStep === 1 ? 'border-b-primary' : 'border-b-transparent'} pb-[13px] pt-4" on:click={() => massAssignStep = 1}>
                  <p class="{massAssignStep === 1 ? 'text-primary' : 'text-[#617589] dark:text-[#94a3b8]'} text-sm font-bold leading-normal tracking-[0.015em]">1. Selección de Usuarios</p>
                </button>
                <button class="flex flex-col items-center justify-center border-b-[3px] {massAssignStep === 2 ? 'border-b-primary' : 'border-b-transparent'} pb-[13px] pt-4" disabled={massAssignStep < 2}>
                  <p class="{massAssignStep === 2 ? 'text-primary' : 'text-[#617589] dark:text-[#94a3b8]'} text-sm font-bold leading-normal tracking-[0.015em]">2. Confirmación</p>
                </button>
                <button class="flex flex-col items-center justify-center border-b-[3px] {massAssignStep === 3 ? 'border-b-primary' : 'border-b-transparent'} pb-[13px] pt-4" disabled={massAssignStep < 3}>
                  <p class="{massAssignStep === 3 ? 'text-primary' : 'text-[#617589] dark:text-[#94a3b8]'} text-sm font-bold leading-normal tracking-[0.015em]">3. Resultado</p>
                </button>
              </div>
            </div>
            
            {#if massAssignStep === 1}
            <div class="flex flex-col py-6 gap-3">
              <details class="flex flex-col rounded-lg border border-[#dbe0e6] dark:border-[#334155] bg-white dark:bg-[#101922] px-[15px] py-[7px] group" open="">
                <summary class="flex cursor-pointer items-center justify-between gap-6 py-2">
                  <p class="text-[#111418] dark:text-white text-sm font-medium leading-normal">Buscar y seleccionar usuarios específicos</p>
                  <div class="text-[#111418] dark:text-white group-open:rotate-180 transition-transform">
                    <span class="material-symbols-outlined">expand_more</span>
                  </div>
                </summary>
                <div class="pb-2">
                  <div class="py-3">
                    <label class="flex flex-col min-w-40 h-12 w-full">
                      <div class="flex w-full flex-1 items-stretch rounded-lg h-full">
                        <div class="text-[#617589] dark:text-[#94a3b8] flex border-none bg-[#f0f2f4] dark:bg-[#1C2A36] items-center justify-center pl-4 rounded-l-lg border-r-0">
                          <span class="material-symbols-outlined">search</span>
                        </div>
                        <input class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-[#111418] dark:text-white focus:outline-0 focus:ring-2 focus:ring-primary/50 border-none bg-[#f0f2f4] dark:bg-[#1C2A36] h-full placeholder:text-[#617589] dark:placeholder:text-[#94a3b8] px-4 rounded-l-none border-l-0 pl-2 text-base font-normal leading-normal" placeholder="Buscar por nombre o email..." value=""/>
                      </div>
                    </label>
                  </div>
                  <div class="px-4 border-t border-[#dbe0e6] dark:border-[#334155] pt-2">
                    <label class="flex gap-x-3 py-3 flex-row items-center">
                      <input checked="" class="h-5 w-5 rounded border-[#dbe0e6] dark:border-[#475569] border-2 bg-transparent text-primary checked:bg-primary checked:border-primary checked:bg-[image:--checkbox-tick-svg] focus:ring-2 focus:ring-offset-0 focus:ring-primary/50 focus:ring-offset-white dark:focus:ring-offset-[#101922] focus:outline-none" type="checkbox"/>
                      <p class="text-[#111418] dark:text-white text-base font-normal leading-normal">Alejandro Vargas (a.vargas@email.com)</p>
                    </label>
                    <label class="flex gap-x-3 py-3 flex-row items-center">
                      <input class="h-5 w-5 rounded border-[#dbe0e6] dark:border-[#475569] border-2 bg-transparent text-primary checked:bg-primary checked:border-primary checked:bg-[image:--checkbox-tick-svg] focus:ring-2 focus:ring-offset-0 focus:ring-primary/50 focus:ring-offset-white dark:focus:ring-offset-[#101922] focus:outline-none" type="checkbox"/>
                      <p class="text-[#111418] dark:text-white text-base font-normal leading-normal">Beatriz Navarro (beatriz.n@email.com)</p>
                    </label>
                    <label class="flex gap-x-3 py-3 flex-row items-center">
                      <input checked="" class="h-5 w-5 rounded border-[#dbe0e6] dark:border-[#475569] border-2 bg-transparent text-primary checked:bg-primary checked:border-primary checked:bg-[image:--checkbox-tick-svg] focus:ring-2 focus:ring-offset-0 focus:ring-primary/50 focus:ring-offset-white dark:focus:ring-offset-[#101922] focus:outline-none" type="checkbox"/>
                      <p class="text-[#111418] dark:text-white text-base font-normal leading-normal">Carlos Jimenez (cjimenez@email.com)</p>
                    </label>
                  </div>
                </div>
              </details>
              <details class="flex flex-col rounded-lg border border-[#dbe0e6] dark:border-[#334155] bg-white dark:bg-[#101922] px-[15px] py-[7px] group">
                <summary class="flex cursor-pointer items-center justify-between gap-6 py-2">
                  <p class="text-[#111418] dark:text-white text-sm font-medium leading-normal">Importar lista de emails</p>
                  <div class="text-[#111418] dark:text-white group-open:rotate-180 transition-transform">
                    <span class="material-symbols-outlined">expand_more</span>
                  </div>
                </summary>
                <div class="p-4">
                  <textarea class="form-textarea w-full rounded-lg border-[#dbe0e6] dark:border-[#334155] bg-[#f0f2f4] dark:bg-[#1C2A36] text-[#111418] dark:text-white placeholder:text-[#617589] dark:placeholder:text-[#94a3b8] focus:border-primary focus:ring-primary/50" placeholder="Pega una lista de emails, uno por línea..." rows="6"></textarea>
                </div>
              </details>
              <details class="flex flex-col rounded-lg border border-[#dbe0e6] dark:border-[#334155] bg-white dark:bg-[#101922] px-[15px] py-[7px] group">
                <summary class="flex cursor-pointer items-center justify-between gap-6 py-2">
                  <p class="text-[#111418] dark:text-white text-sm font-medium leading-normal">Asignar a un segmento</p>
                  <div class="text-[#111418] dark:text-white group-open:rotate-180 transition-transform">
                    <span class="material-symbols-outlined">expand_more</span>
                  </div>
                </summary>
                <div class="p-4 space-y-3">
                  <label class="flex items-center gap-x-3">
                    <input class="form-radio h-5 w-5 text-primary border-[#dbe0e6] dark:border-[#475569] dark:bg-[#1C2A36] focus:ring-primary/50" name="segment" type="radio"/>
                    <span class="text-[#111418] dark:text-white">Todos los usuarios</span>
                  </label>
                  <label class="flex items-center gap-x-3">
                    <input class="form-radio h-5 w-5 text-primary border-[#dbe0e6] dark:border-[#475569] dark:bg-[#1C2A36] focus:ring-primary/50" name="segment" type="radio"/>
                    <span class="text-[#111418] dark:text-white">Usuarios sin compras</span>
                  </label>
                  <label class="flex items-center gap-x-3">
                    <input class="form-radio h-5 w-5 text-primary border-[#dbe0e6] dark:border-[#475569] dark:bg-[#1C2A36] focus:ring-primary/50" name="segment" type="radio"/>
                    <span class="text-[#111418] dark:text-white">Usuarios activos</span>
                  </label>
                </div>
              </details>
            </div>
            <div class="flex justify-end gap-4 pt-6 border-t border-[#dbe0e6] dark:border-[#334155]">
              <button class="px-6 py-3 rounded-lg text-sm font-bold bg-gray-100 dark:bg-[#334155] text-[#111418] dark:text-white hover:bg-gray-200 dark:hover:bg-[#475569] transition-colors" on:click={closeMassAssignModal}>Cancelar</button>
              <button class="px-6 py-3 rounded-lg text-sm font-bold bg-primary text-white hover:bg-primary/90 transition-colors flex items-center gap-2" on:click={() => massAssignStep = 2}>
                <span>Siguiente</span>
                <span class="material-symbols-outlined text-base">arrow_forward</span>
              </button>
            </div>
            {:else if massAssignStep === 2}
            <div class="py-6">
              <div class="space-y-6">
                <div class="rounded-lg border border-[#dbe0e6] dark:border-[#334155] p-6 bg-white dark:bg-[#101922]">
                  <h2 class="text-lg font-bold text-[#111418] dark:text-white mb-4">Resumen de la Asignación</h2>
                  <div class="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-4 text-sm">
                    <div>
                      <p class="text-[#617589] dark:text-[#94a3b8] font-medium">Cupón a asignar</p>
                      <p class="text-[#111418] dark:text-white font-semibold">GAMERDAY25 (25% Descuento)</p>
                    </div>
                    <div>
                      <p class="text-[#617589] dark:text-[#94a3b8] font-medium">Usuarios seleccionados</p>
                      <p class="text-[#111418] dark:text-white font-semibold">2 usuarios</p>
                    </div>
                    <div>
                      <p class="text-[#617589] dark:text-[#94a3b8] font-medium">Método de selección</p>
                      <p class="text-[#111418] dark:text-white font-semibold">Selección manual</p>
                    </div>
                  </div>
                </div>
                <div class="rounded-lg border border-[#dbe0e6] dark:border-[#334155] bg-white dark:bg-[#101922]">
                  <div class="p-6">
                    <h3 class="text-lg font-bold text-[#111418] dark:text-white">Usuarios que recibirán el cupón</h3>
                  </div>
                  <div class="border-t border-[#dbe0e6] dark:border-[#334155]">
                    <ul class="divide-y divide-[#dbe0e6] dark:divide-[#334155]">
                      <li class="px-6 py-3 flex items-center gap-3">
                        <span class="material-symbols-outlined text-xl text-[#617589] dark:text-[#94a3b8]">person</span>
                        <span class="text-[#111418] dark:text-white">Alejandro Vargas (a.vargas@email.com)</span>
                      </li>
                      <li class="px-6 py-3 flex items-center gap-3">
                        <span class="material-symbols-outlined text-xl text-[#617589] dark:text-[#94a3b8]">person</span>
                        <span class="text-[#111418] dark:text-white">Carlos Jimenez (cjimenez@email.com)</span>
                      </li>
                    </ul>
                  </div>
                </div>
                <div class="flex items-start gap-4 p-4 rounded-lg bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-500/30">
                  <span class="material-symbols-outlined text-blue-600 dark:text-blue-400 mt-0.5">warning</span>
                  <div class="flex-1">
                    <p class="font-bold text-blue-800 dark:text-blue-300">Acción irreversible</p>
                    <p class="text-sm text-blue-700 dark:text-blue-400">Una vez asignados los cupones, la acción no se podrá deshacer. Por favor, revisa cuidadosamente la lista de usuarios antes de confirmar.</p>
                  </div>
                </div>
              </div>
            </div>
            <div class="flex justify-between items-center gap-4 pt-6 border-t border-[#dbe0e6] dark:border-[#334155]">
              <button class="px-6 py-3 rounded-lg text-sm font-bold bg-gray-100 dark:bg-[#334155] text-[#111418] dark:text-white hover:bg-gray-200 dark:hover:bg-[#475569] transition-colors flex items-center gap-2" on:click={() => massAssignStep = 1}>
                <span class="material-symbols-outlined text-base">arrow_back</span>
                <span>Volver</span>
              </button>
              <button class="px-6 py-3 rounded-lg text-sm font-bold bg-primary text-white hover:bg-primary/90 transition-colors flex items-center gap-2" on:click={() => massAssignStep = 3}>
                <span>Confirmar y Asignar</span>
                <span class="material-symbols-outlined text-base">task_alt</span>
              </button>
            </div>
            {:else if massAssignStep === 3}
            <div class="flex flex-col py-6 items-center text-center">
              <div class="flex items-center justify-center w-20 h-20 bg-[#e0f2fe] rounded-full mb-6">
                <span class="material-symbols-outlined text-primary text-5xl">task_alt</span>
              </div>
              <h2 class="text-2xl font-bold text-[#111418] dark:text-white mb-3">¡Asignación completada con éxito!</h2>
              <p class="text-base text-[#617589] dark:text-[#94a3b8] mb-8 max-w-lg">El proceso de asignación masiva para el cupón <strong class="text-[#111418] dark:text-white">PROMO2024</strong> ha finalizado. A continuación se muestra un resumen del resultado.</p>
              <div class="w-full max-w-md bg-[#f6f7f8] dark:bg-[#101922] rounded-lg border border-[#dbe0e6] dark:border-[#334155] p-6 text-left space-y-4">
                <div class="flex justify-between items-center">
                  <div class="flex items-center gap-3">
                    <span class="material-symbols-outlined text-green-500">check_circle</span>
                    <p class="text-[#111418] dark:text-white">Nuevos usuarios con cupón:</p>
                  </div>
                  <p class="font-bold text-[#111418] dark:text-white">147</p>
                </div>
                <div class="flex justify-between items-center">
                  <div class="flex items-center gap-3">
                    <span class="material-symbols-outlined text-blue-400">info</span>
                    <p class="text-[#111418] dark:text-white">Usuarios que ya tenían el cupón:</p>
                  </div>
                  <p class="font-bold text-[#111418] dark:text-white">3</p>
                </div>
                <div class="flex justify-between items-center">
                  <div class="flex items-center gap-3">
                    <span class="material-symbols-outlined text-yellow-500">warning</span>
                    <p class="text-[#111418] dark:text-white">Emails no válidos o no encontrados:</p>
                  </div>
                  <p class="font-bold text-[#111418] dark:text-white">5</p>
                </div>
              </div>
            </div>
            <div class="flex justify-end gap-4 pt-6 border-t border-[#dbe0e6] dark:border-[#334155]">
              <button class="px-6 py-3 rounded-lg text-sm font-bold bg-gray-100 dark:bg-[#334155] text-[#111418] dark:text-white hover:bg-gray-200 dark:hover:bg-[#475569] transition-colors" on:click={closeMassAssignModal}>Cerrar</button>
              <button class="px-6 py-3 rounded-lg text-sm font-bold bg-primary text-white hover:bg-primary/90 transition-colors flex items-center gap-2" on:click={closeMassAssignModal}>
                <span>Ver cupones asignados</span>
                <span class="material-symbols-outlined text-base">arrow_forward</span>
              </button>
            </div>
            {/if}
          </div>
        </div>
      </div>
    </div>
  </div>
{/if}
