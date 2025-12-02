<script lang="ts">
  let isReviewModalOpen = false;
  let isDetailsModalOpen = false;

  function openReviewModal() {
    isReviewModalOpen = true;
  }

  function closeReviewModal() {
    isReviewModalOpen = false;
  }

  function openDetailsModal() {
    isDetailsModalOpen = true;
  }

  function closeDetailsModal() {
    isDetailsModalOpen = false;
  }
</script>

<div class="relative flex h-auto min-h-screen w-full flex-col group/design-root overflow-x-hidden bg-background-light dark:bg-background-dark font-display text-gray-800 dark:text-gray-200">
  <div class="layout-container flex h-full grow flex-col">
    <div class="px-4 sm:px-8 md:px-12 lg:px-20 xl:px-40 flex flex-1 justify-center py-5">
      <div class="layout-content-container flex flex-col max-w-[1280px] flex-1">
        <!-- Page Header -->
        <div class="flex flex-wrap justify-between items-center gap-4 p-4">
          <div class="flex flex-col gap-1">
            <p class="text-[#111418] dark:text-white text-4xl font-black leading-tight tracking-[-0.033em]">Gestión de Reembolsos</p>
            <p class="text-[#617589] dark:text-gray-400 text-base font-normal leading-normal">Administra y procesa las solicitudes de reembolso de los clientes.</p>
          </div>
        </div>

        <!-- Stats Cards -->
        <div class="flex flex-wrap gap-4 p-4">
          <div class="flex min-w-[200px] flex-1 flex-col gap-2 rounded-lg p-6 border border-gray-200 dark:border-gray-700 bg-white dark:bg-background-dark">
            <div class="flex items-center justify-between">
              <p class="text-[#111418] dark:text-gray-300 text-base font-medium leading-normal">Reembolsos pendientes</p>
              <span class="material-symbols-outlined text-gray-400">hourglass_top</span>
            </div>
            <div class="flex items-center gap-2">
              <p class="text-[#111418] dark:text-white tracking-light text-3xl font-bold leading-tight">8</p>
              <div class="relative flex items-center justify-center w-6 h-6 bg-red-500 rounded-full text-white text-xs font-bold">!</div>
            </div>
            <p class="text-red-500 text-sm font-medium leading-normal">+2 esta semana</p>
          </div>
          <div class="flex min-w-[200px] flex-1 flex-col gap-2 rounded-lg p-6 border border-gray-200 dark:border-gray-700 bg-white dark:bg-background-dark">
            <div class="flex items-center justify-between">
              <p class="text-[#111418] dark:text-gray-300 text-base font-medium leading-normal">Reembolsos completados (mes)</p>
              <span class="material-symbols-outlined text-gray-400">task_alt</span>
            </div>
            <p class="text-[#111418] dark:text-white tracking-light text-3xl font-bold leading-tight">42</p>
            <p class="text-green-600 text-sm font-medium leading-normal">+15.2%</p>
          </div>
          <div class="flex min-w-[200px] flex-1 flex-col gap-2 rounded-lg p-6 border border-gray-200 dark:border-gray-700 bg-white dark:bg-background-dark">
            <div class="flex items-center justify-between">
              <p class="text-[#111418] dark:text-gray-300 text-base font-medium leading-normal">Monto total reembolsado (mes)</p>
              <span class="material-symbols-outlined text-gray-400">payments</span>
            </div>
            <p class="text-[#111418] dark:text-white tracking-light text-3xl font-bold leading-tight">$5,830.50</p>
            <p class="text-green-600 text-sm font-medium leading-normal">+8.5%</p>
          </div>
        </div>

        <!-- Filters -->
        <div class="flex flex-wrap justify-between items-center gap-4 p-4">
          <div class="flex items-center gap-2">
            <div class="relative">
              <span class="material-symbols-outlined absolute left-3 top-1/2 -translate-y-1/2 text-gray-400">search</span>
              <input class="pl-10 pr-4 py-2 w-64 border border-gray-200 dark:border-gray-700 rounded-lg bg-white dark:bg-gray-800 focus:ring-2 focus:ring-primary focus:outline-none dark:text-white" placeholder="Buscar por pedido o email..." type="text"/>
            </div>
            <select class="border border-gray-200 dark:border-gray-700 rounded-lg bg-white dark:bg-gray-800 focus:ring-2 focus:ring-primary focus:outline-none dark:text-white px-4 py-2">
              <option>Estado: Todos</option>
              <option>Solicitado</option>
              <option>Procesando</option>
              <option>Completado</option>
              <option>Rechazado</option>
            </select>
            <select class="border border-gray-200 dark:border-gray-700 rounded-lg bg-white dark:bg-gray-800 focus:ring-2 focus:ring-primary focus:outline-none dark:text-white px-4 py-2">
              <option>Tipo: Todos</option>
              <option>Total</option>
              <option>Parcial</option>
            </select>
          </div>
        </div>

        <!-- Table -->
        <div class="px-4 py-3 @container">
          <div class="overflow-x-auto rounded-lg border border-gray-200 dark:border-gray-700 bg-white dark:bg-background-dark">
            <table class="w-full text-left">
              <thead class="bg-gray-50 dark:bg-gray-800">
                <tr>
                  <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">ID Reembolso</th>
                  <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Nº de pedido</th>
                  <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Cliente</th>
                  <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Tipo</th>
                  <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Monto</th>
                  <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Estado</th>
                  <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Fecha solicitado</th>
                  <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider" style="width: 210px;">Acciones</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-gray-200 dark:divide-gray-700">
                <tr class="hover:bg-gray-50 dark:hover:bg-gray-800/50 bg-yellow-50 dark:bg-yellow-900/20">
                  <td class="px-4 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">RF84301</td>
                  <td class="px-4 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">ORD-99124</td>
                  <td class="px-4 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-white">Ana Torres <br/><span class="font-normal text-gray-500 dark:text-gray-400">ana.t@email.com</span></td>
                  <td class="px-4 py-4 whitespace-nowrap"><span class="px-2.5 py-1 text-xs font-semibold rounded-full bg-gray-200 text-gray-800 dark:bg-gray-700 dark:text-gray-300">Parcial</span></td>
                  <td class="px-4 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">$150.00</td>
                  <td class="px-4 py-4 whitespace-nowrap"><span class="px-2.5 py-1 text-xs font-semibold rounded-full bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-300">Solicitado</span></td>
                  <td class="px-4 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">2023-10-26</td>
                  <td class="px-4 py-4 text-sm font-medium">
                    <div class="flex items-center gap-2">
                      <button class="px-3 py-1.5 text-xs font-semibold text-white bg-primary rounded-md hover:bg-blue-600 whitespace-nowrap" on:click={openReviewModal}>Procesar</button>
                      <button class="px-3 py-1.5 text-xs font-semibold text-gray-700 bg-gray-200 rounded-md hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-300 dark:hover:bg-gray-600 whitespace-nowrap" on:click={openDetailsModal}>Ver detalle</button>
                    </div>
                  </td>
                </tr>
                <tr class="hover:bg-gray-50 dark:hover:bg-gray-800/50">
                  <td class="px-4 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">RF84300</td>
                  <td class="px-4 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">ORD-99121</td>
                  <td class="px-4 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-white">Carlos Gomez <br/><span class="font-normal text-gray-500 dark:text-gray-400">c.gomez@email.com</span></td>
                  <td class="px-4 py-4 whitespace-nowrap"><span class="px-2.5 py-1 text-xs font-semibold rounded-full bg-gray-200 text-gray-800 dark:bg-gray-700 dark:text-gray-300">Total</span></td>
                  <td class="px-4 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">$899.99</td>
                  <td class="px-4 py-4 whitespace-nowrap"><span class="px-2.5 py-1 text-xs font-semibold rounded-full bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-300">Rechazado</span></td>
                  <td class="px-4 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">2023-10-25</td>
                  <td class="px-4 py-4 text-sm font-medium">
                    <div class="flex items-center gap-2">
                      <button class="px-3 py-1.5 text-xs font-semibold text-white bg-primary rounded-md hover:bg-blue-600 whitespace-nowrap" on:click={openReviewModal}>Procesar</button>
                      <button class="px-3 py-1.5 text-xs font-semibold text-gray-700 bg-gray-200 rounded-md hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-300 dark:hover:bg-gray-600 whitespace-nowrap" on:click={openDetailsModal}>Ver detalle</button>
                    </div>
                  </td>
                </tr>
                <tr class="hover:bg-gray-50 dark:hover:bg-gray-800/50">
                  <td class="px-4 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">RF84299</td>
                  <td class="px-4 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">ORD-99115</td>
                  <td class="px-4 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-white">Lucía Fernández <br/><span class="font-normal text-gray-500 dark:text-gray-400">lfernandez@email.com</span></td>
                  <td class="px-4 py-4 whitespace-nowrap"><span class="px-2.5 py-1 text-xs font-semibold rounded-full bg-gray-200 text-gray-800 dark:bg-gray-700 dark:text-gray-300">Total</span></td>
                  <td class="px-4 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">$1,250.00</td>
                  <td class="px-4 py-4 whitespace-nowrap"><span class="px-2.5 py-1 text-xs font-semibold rounded-full bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-300">Rechazado</span></td>
                  <td class="px-4 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">2023-10-24</td>
                  <td class="px-4 py-4 text-sm font-medium">
                    <div class="flex items-center gap-2">
                      <button class="px-3 py-1.5 text-xs font-semibold text-white bg-primary rounded-md hover:bg-blue-600 whitespace-nowrap" on:click={openReviewModal}>Procesar</button>
                      <button class="px-3 py-1.5 text-xs font-semibold text-gray-700 bg-gray-200 rounded-md hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-300 dark:hover:bg-gray-600 whitespace-nowrap" on:click={openDetailsModal}>Ver detalle</button>
                    </div>
                  </td>
                </tr>
                <tr class="hover:bg-gray-50 dark:hover:bg-gray-800/50">
                  <td class="px-4 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">RF84298</td>
                  <td class="px-4 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">ORD-99112</td>
                  <td class="px-4 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-white">Javier Muñoz <br/><span class="font-normal text-gray-500 dark:text-gray-400">javierm@email.com</span></td>
                  <td class="px-4 py-4 whitespace-nowrap"><span class="px-2.5 py-1 text-xs font-semibold rounded-full bg-gray-200 text-gray-800 dark:bg-gray-700 dark:text-gray-300">Parcial</span></td>
                  <td class="px-4 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">$45.50</td>
                  <td class="px-4 py-4 whitespace-nowrap"><span class="px-2.5 py-1 text-xs font-semibold rounded-full bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-300">Aceptado</span></td>
                  <td class="px-4 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">2023-10-23</td>
                  <td class="px-4 py-4 text-sm font-medium">
                    <div class="flex items-center gap-2">
                      <button class="px-3 py-1.5 text-xs font-semibold text-white bg-primary rounded-md hover:bg-blue-600 whitespace-nowrap" on:click={openReviewModal}>Procesar</button>
                      <button class="px-3 py-1.5 text-xs font-semibold text-gray-700 bg-gray-200 rounded-md hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-300 dark:hover:bg-gray-600 whitespace-nowrap" on:click={openDetailsModal}>Ver detalle</button>
                    </div>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>

        <!-- Pagination -->
        <div class="flex items-center justify-between p-4 border-t border-gray-200 dark:border-gray-700">
          <p class="text-sm text-gray-600 dark:text-gray-400">Mostrando <span class="font-semibold">1</span>-<span class="font-semibold">4</span> de <span class="font-semibold">25</span> resultados</p>
          <div class="flex items-center justify-center">
            <button class="flex w-10 h-10 items-center justify-center rounded-md border border-gray-200 dark:border-gray-700 hover:bg-gray-100 dark:hover:bg-gray-800">
              <span class="material-symbols-outlined text-lg">chevron_left</span>
            </button>
            <button class="text-sm font-bold flex w-10 h-10 items-center justify-center text-white rounded-md bg-primary mx-1">1</button>
            <button class="text-sm font-normal flex w-10 h-10 items-center justify-center text-[#111418] dark:text-gray-300 rounded-md hover:bg-gray-100 dark:hover:bg-gray-800">2</button>
            <button class="text-sm font-normal flex w-10 h-10 items-center justify-center text-[#111418] dark:text-gray-300 rounded-md hover:bg-gray-100 dark:hover:bg-gray-800">3</button>
            <span class="text-sm font-normal flex w-10 h-10 items-center justify-center text-[#111418] dark:text-gray-300 rounded-md">...</span>
            <button class="text-sm font-normal flex w-10 h-10 items-center justify-center text-[#111418] dark:text-gray-300 rounded-md hover:bg-gray-100 dark:hover:bg-gray-800">7</button>
            <button class="flex w-10 h-10 items-center justify-center rounded-md border border-gray-200 dark:border-gray-700 hover:bg-gray-100 dark:hover:bg-gray-800 ml-1">
              <span class="material-symbols-outlined text-lg">chevron_right</span>
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>

<!-- Refund Review Modal -->
{#if isReviewModalOpen}
  <div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-gray-900/50 dark:bg-gray-900/70 backdrop-blur-sm" on:click={closeReviewModal}>
    <div class="w-full max-w-4xl rounded-xl bg-white dark:bg-background-dark shadow-2xl" on:click|stopPropagation>
      <div class="flex flex-col">
        <!-- Modal Header -->
        <div class="flex items-center justify-between gap-3 p-6 border-b border-gray-200 dark:border-gray-800">
          <p class="text-[#111418] dark:text-gray-100 tracking-light text-[24px] font-bold leading-tight">Revisar Solicitud de Reembolso #8675309</p>
          <button class="flex h-8 w-8 items-center justify-center rounded-full text-gray-500 hover:bg-gray-100 dark:hover:bg-gray-800 dark:text-gray-400" on:click={closeReviewModal}>
            <span aria-hidden="true" class="material-symbols-outlined">close</span>
          </button>
        </div>
        <!-- Modal Body -->
        <div class="flex-1 p-6 space-y-8">
          <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
            <!-- Left Column: Info -->
            <div class="space-y-6">
              <!-- Order Info -->
              <div>
                <h3 class="text-[#111418] dark:text-gray-100 text-lg font-bold leading-tight tracking-[-0.015em] pb-2">Información del pedido</h3>
                <div class="grid grid-cols-2 gap-x-4 gap-y-1 border-t border-gray-200 dark:border-gray-800 pt-4">
                  <div class="flex flex-col gap-1 py-2 pr-2 col-span-2 sm:col-span-1">
                    <p class="text-[#617589] dark:text-gray-400 text-sm font-normal leading-normal">Número de pedido:</p>
                    <p class="text-[#111418] dark:text-gray-200 text-sm font-medium leading-normal">ORD-98765</p>
                  </div>
                  <div class="flex flex-col gap-1 py-2 pl-0 sm:pl-2 col-span-2 sm:col-span-1">
                    <p class="text-[#617589] dark:text-gray-400 text-sm font-normal leading-normal">Cliente:</p>
                    <p class="text-[#111418] dark:text-gray-200 text-sm font-medium leading-normal">Juan Pérez</p>
                  </div>
                  <div class="flex flex-col gap-1 py-2 pr-2 col-span-2 sm:col-span-1">
                    <p class="text-[#617589] dark:text-gray-400 text-sm font-normal leading-normal">Email:</p>
                    <p class="text-[#111418] dark:text-gray-200 text-sm font-medium leading-normal">juan.perez@example.com</p>
                  </div>
                  <div class="flex flex-col gap-1 py-2 pl-0 sm:pl-2 col-span-2 sm:col-span-1">
                    <p class="text-[#617589] dark:text-gray-400 text-sm font-normal leading-normal">Fecha del pedido:</p>
                    <p class="text-[#111418] dark:text-gray-200 text-sm font-medium leading-normal">15 de Mayo, 2024</p>
                  </div>
                  <div class="flex flex-col gap-1 py-2 pr-2 col-span-2 sm:col-span-1">
                    <p class="text-[#617589] dark:text-gray-400 text-sm font-normal leading-normal">Monto original pagado:</p>
                    <p class="text-[#111418] dark:text-gray-200 text-sm font-medium leading-normal">S/ 1,250.50</p>
                  </div>
                  <div class="flex flex-col gap-1 py-2 pl-0 sm:pl-2 col-span-2 sm:col-span-1">
                    <p class="text-[#617589] dark:text-gray-400 text-sm font-normal leading-normal">Método de pago:</p>
                    <p class="text-[#111418] dark:text-gray-200 text-sm font-medium leading-normal">Visa **** 4242</p>
                  </div>
                </div>
              </div>
              <!-- Refund Request Info -->
              <div>
                <h3 class="text-[#111418] dark:text-gray-100 text-lg font-bold leading-tight tracking-[-0.015em] pb-2">Solicitud de reembolso</h3>
                <div class="space-y-4 border-t border-gray-200 dark:border-gray-800 pt-4">
                  <div class="flex items-center gap-4">
                    <p class="text-[#617589] dark:text-gray-400 text-sm font-normal leading-normal w-32 shrink-0">Tipo:</p>
                    <span class="inline-flex items-center rounded-full bg-primary/20 dark:bg-primary/30 px-3 py-1 text-sm font-medium text-primary dark:text-blue-200">Parcial</span>
                  </div>
                  <div class="flex items-center gap-4">
                    <p class="text-[#617589] dark:text-gray-400 text-sm font-normal leading-normal w-32 shrink-0">Monto solicitado:</p>
                    <p class="text-primary dark:text-blue-300 text-base font-bold leading-normal">S/ 250.00</p>
                  </div>
                  <div class="flex items-center gap-4">
                    <p class="text-[#617589] dark:text-gray-400 text-sm font-normal leading-normal w-32 shrink-0">Fecha de solicitud:</p>
                    <p class="text-[#111418] dark:text-gray-200 text-sm font-medium leading-normal">28 de Mayo, 2024</p>
                  </div>
                  <div class="flex flex-col gap-2">
                    <p class="text-[#617589] dark:text-gray-400 text-sm font-normal leading-normal">Motivo del cliente:</p>
                    <div class="w-full rounded-lg border border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800 p-3">
                      <p class="text-[#111418] dark:text-gray-200 text-sm font-normal leading-relaxed">El ventilador de la tarjeta gráfica que compré hace un ruido extraño cuando está bajo carga. He intentado solucionarlo, pero el problema persiste. Solicito un reembolso parcial por el componente defectuoso.</p>
                    </div>
                  </div>
                </div>
              </div>
            </div>
            <!-- Right Column: Decision Form -->
            <div class="bg-background-light dark:bg-gray-900/50 rounded-lg p-6 space-y-6">
              <h3 class="text-[#111418] dark:text-gray-100 text-lg font-bold leading-tight tracking-[-0.015em]">Decisión</h3>
              <div class="space-y-4">
                <!-- Radio Buttons -->
                <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                  <label class="flex cursor-pointer items-center gap-3 rounded-lg border-2 border-green-500 bg-green-50 dark:bg-green-900/30 p-4 ring-2 ring-transparent has-[:checked]:ring-green-500">
                    <input class="form-radio h-5 w-5 border-gray-300 text-green-600 focus:ring-green-500" name="decision" type="radio" value="approve"/>
                    <span class="font-semibold text-green-800 dark:text-green-300">Aprobar reembolso</span>
                  </label>
                  <label class="flex cursor-pointer items-center gap-3 rounded-lg border-2 border-red-500 bg-red-50 dark:bg-red-900/30 p-4 ring-2 ring-transparent has-[:checked]:ring-red-500">
                    <input class="form-radio h-5 w-5 border-gray-300 text-red-600 focus:ring-red-500" name="decision" type="radio" value="reject"/>
                    <span class="font-semibold text-red-800 dark:text-red-300">Rechazar reembolso</span>
                  </label>
                </div>
                <!-- Conditional Fields -->
                <div>
                  <!-- Approval Fields -->
                  <div class="space-y-4">
                    <div>
                      <label class="text-sm font-medium text-gray-700 dark:text-gray-300" for="refund_amount">Monto final a reembolsar</label>
                      <input class="mt-1 block w-full rounded border-gray-300 shadow-sm focus:border-primary focus:ring-primary dark:bg-gray-700 dark:border-gray-600 dark:text-white sm:text-sm" id="refund_amount" type="text" value="S/ 250.00"/>
                    </div>
                    <div>
                      <label class="text-sm font-medium text-gray-700 dark:text-gray-300" for="admin_notes">Notas administrativas (opcional)</label>
                      <textarea class="mt-1 block w-full rounded border-gray-300 shadow-sm focus:border-primary focus:ring-primary dark:bg-gray-700 dark:border-gray-600 dark:text-white sm:text-sm" id="admin_notes" placeholder="Notas internas sobre este reembolso..." rows="3"></textarea>
                    </div>
                    <p class="text-xs text-gray-500 dark:text-gray-400">Al aprobar, el reembolso se marcará como completado y se iniciará el proceso de devolución.</p>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
        <!-- Modal Footer -->
        <div class="flex items-center justify-end gap-4 p-6 bg-background-light dark:bg-gray-900/50 rounded-b-xl border-t border-gray-200 dark:border-gray-800">
          <button class="rounded px-4 py-2 text-sm font-semibold text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-700 hover:bg-gray-100 dark:hover:bg-gray-600 border border-gray-300 dark:border-gray-600 shadow-sm" on:click={closeReviewModal}>Cancelar</button>
          <button class="flex items-center gap-2 rounded bg-green-600 px-4 py-2 text-sm font-semibold text-green-50 shadow-sm hover:bg-green-700">
            <span aria-hidden="true" class="material-symbols-outlined text-green-50" style="font-size: 18px;">check_circle</span>
            Confirmar Aprobación
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}

<!-- Refund Details Modal -->
{#if isDetailsModalOpen}
  <div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-gray-900/50 dark:bg-black/60 backdrop-blur-sm" on:click={closeDetailsModal}>
    <div class="relative w-full max-w-2xl bg-white dark:bg-gray-900 rounded-xl shadow-lg flex flex-col" on:click|stopPropagation>
      <!-- Modal Header -->
      <div class="flex items-start justify-between p-6 border-b border-gray-200 dark:border-gray-700">
        <div class="flex flex-col gap-2">
          <p class="text-[#111418] dark:text-gray-100 tracking-light text-2xl font-bold leading-tight">Detalle del Reembolso #789123</p>
          <div class="flex items-center gap-x-2">
            <div class="flex h-7 shrink-0 items-center justify-center gap-x-2 rounded-full bg-green-100 dark:bg-green-900/40 px-3 py-1">
              <p class="text-green-700 dark:text-green-300 text-sm font-medium leading-normal">Completado</p>
            </div>
          </div>
        </div>
        <button class="text-gray-400 hover:text-gray-600 dark:text-gray-500 dark:hover:text-gray-300" on:click={closeDetailsModal}>
          <span class="material-symbols-outlined">close</span>
        </button>
      </div>
      <!-- Modal Body -->
      <div class="p-6 space-y-8 overflow-y-auto max-h-[70vh] scrollbar-thin scrollbar-thumb-gray-400 dark:scrollbar-thumb-gray-600 scrollbar-track-gray-200 dark:scrollbar-track-gray-800">
        <!-- Sección Información del pedido -->
        <div class="space-y-4">
          <h3 class="text-[#111418] dark:text-gray-200 text-lg font-bold leading-tight tracking-[-0.015em]">Información del pedido</h3>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-x-6 gap-y-4 text-sm">
            <div>
              <p class="text-[#617589] dark:text-gray-400 font-normal leading-normal">Número de pedido</p>
              <p class="text-[#111418] dark:text-gray-100 font-medium leading-normal">ORD-55432-A</p>
            </div>
            <div>
              <p class="text-[#617589] dark:text-gray-400 font-normal leading-normal">Cliente</p>
              <p class="text-[#111418] dark:text-gray-100 font-medium leading-normal">Alejandro Vargas</p>
            </div>
            <div>
              <p class="text-[#617589] dark:text-gray-400 font-normal leading-normal">Email</p>
              <p class="text-[#111418] dark:text-gray-100 font-medium leading-normal">a.vargas@email.com</p>
            </div>
            <div>
              <p class="text-[#617589] dark:text-gray-400 font-normal leading-normal">Monto original</p>
              <p class="text-[#111418] dark:text-gray-100 font-medium leading-normal">S/ 850.50</p>
            </div>
            <div>
              <p class="text-[#617589] dark:text-gray-400 font-normal leading-normal">Fecha del pedido</p>
              <p class="text-[#111418] dark:text-gray-100 font-medium leading-normal">15 de Mayo, 2024</p>
            </div>
          </div>
        </div>
        <hr class="border-t border-gray-200 dark:border-gray-700"/>
        <!-- Sección Detalles del reembolso -->
        <div class="space-y-4">
          <h3 class="text-[#111418] dark:text-gray-200 text-lg font-bold leading-tight tracking-[-0.015em]">Detalles del reembolso</h3>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-x-6 gap-y-4 text-sm">
            <div>
              <p class="text-[#617589] dark:text-gray-400 font-normal leading-normal">Tipo</p>
              <p class="text-[#111418] dark:text-gray-100 font-medium leading-normal">Parcial</p>
            </div>
            <div>
              <p class="text-[#617589] dark:text-gray-400 font-normal leading-normal">Fecha solicitado</p>
              <p class="text-[#111418] dark:text-gray-100 font-medium leading-normal">18 de Mayo, 2024</p>
            </div>
            <div class="col-span-1 md:col-span-2">
              <p class="text-[#617589] dark:text-gray-400 font-normal leading-normal">Monto reembolsado</p>
              <p class="text-primary dark:text-primary-300 text-2xl font-bold leading-normal">S/ 250.00</p>
            </div>
            <div class="col-span-1 md:col-span-2">
              <p class="text-[#617589] dark:text-gray-400 font-normal leading-normal">Motivo del cliente</p>
              <p class="text-[#111418] dark:text-gray-100 font-medium leading-normal">El producto llegó dañado y solo necesito el reembolso por ese ítem específico.</p>
            </div>
            <div>
              <p class="text-[#617589] dark:text-gray-400 font-normal leading-normal">Fecha completado</p>
              <p class="text-[#111418] dark:text-gray-100 font-medium leading-normal">20 de Mayo, 2024</p>
            </div>
          </div>
        </div>
        <hr class="border-t border-gray-200 dark:border-gray-700"/>
        <!-- Sección Decisión administrativa -->
        <div class="space-y-4">
          <h3 class="text-[#111418] dark:text-gray-200 text-lg font-bold leading-tight tracking-[-0.015em]">Decisión administrativa</h3>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-x-6 gap-y-4 text-sm">
            <div>
              <p class="text-[#617589] dark:text-gray-400 font-normal leading-normal">Aprobado por</p>
              <p class="text-[#111418] dark:text-gray-100 font-medium leading-normal">Carlos Mendoza</p>
            </div>
            <div class="col-span-1 md:col-span-2">
              <p class="text-[#617589] dark:text-gray-400 font-normal leading-normal">Notas del administrador</p>
              <p class="text-[#111418] dark:text-gray-100 font-medium leading-normal">Cliente envió fotos del producto dañado. Reembolso parcial aprobado.</p>
            </div>
          </div>
        </div>
        <!-- Timeline -->
        <div class="pt-2">
          <div class="relative pl-8">
            <div class="absolute left-3 top-1 bottom-1 w-0.5 bg-gray-200 dark:bg-gray-700"></div>
            <div class="relative mb-6">
              <div class="absolute -left-[30px] top-1/2 -translate-y-1/2 h-5 w-5 rounded-full bg-primary flex items-center justify-center">
                <div class="h-2 w-2 rounded-full bg-white"></div>
              </div>
              <p class="text-[#111418] dark:text-gray-100 text-sm font-medium">Solicitado</p>
              <p class="text-[#617589] dark:text-gray-400 text-xs">18 de Mayo, 2024 - 09:30 AM</p>
            </div>
            <div class="relative">
              <div class="absolute -left-[30px] top-1/2 -translate-y-1/2 h-5 w-5 rounded-full bg-green-500 flex items-center justify-center">
                <div class="h-2 w-2 rounded-full bg-white"></div>
              </div>
              <p class="text-[#111418] dark:text-gray-100 text-sm font-medium">Completado</p>
              <p class="text-[#617589] dark:text-gray-400 text-xs">20 de Mayo, 2024 - 02:15 PM</p>
            </div>
          </div>
        </div>
      </div>
      <!-- Modal Footer -->
      <div class="flex items-center justify-end p-6 border-t border-gray-200 dark:border-gray-700">
        <button class="bg-gray-100 dark:bg-gray-700 text-[#111418] dark:text-gray-100 hover:bg-gray-200 dark:hover:bg-gray-600 font-medium py-2 px-4 rounded-lg transition-colors" on:click={closeDetailsModal}>Cerrar</button>
      </div>
    </div>
  </div>
{/if}
