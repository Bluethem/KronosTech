<script lang="ts">
  // No script logic needed yet, just UI
</script>

<div class="relative flex h-auto min-h-screen w-full flex-col group/design-root bg-background-light dark:bg-background-dark font-display text-[#111418] dark:text-gray-200">
  <div class="layout-container flex h-full grow flex-col">
    <main class="flex-1 p-4 sm:p-6 lg:p-8">
      <div class="mx-auto max-w-7xl">
        <!-- Header matching gestion-descuentos -->
        <div class="flex flex-col sm:flex-row flex-wrap justify-between items-start gap-4 mb-6">
          <div class="flex items-center gap-4">
            <h1 class="text-3xl lg:text-4xl font-black tracking-tight text-[#111418] dark:text-white">Gestión de Descuentos</h1>
            <div class="flex h-8 shrink-0 items-center justify-center gap-x-2 rounded-full bg-primary/10 dark:bg-primary/20 px-4">
              <span class="material-symbols-outlined text-primary dark:text-primary-400 !text-base">add_circle</span>
              <p class="text-primary dark:text-primary-400 text-sm font-medium">Nuevo Descuento</p>
            </div>
          </div>
        </div>

        <!-- Form Content -->
        <div class="px-4 sm:px-10 md:px-20 lg:px-40 flex flex-1 justify-center py-5">
          <div class="layout-content-container flex flex-col w-full max-w-[960px] flex-1 bg-white dark:bg-background-dark rounded-xl shadow-sm border border-gray-200 dark:border-slate-800">
            <!-- Page Heading inside form card -->
            <div class="flex flex-wrap justify-between gap-3 p-4 border-b border-gray-200 dark:border-slate-800">
              <div class="flex min-w-72 flex-col gap-3">
                <p class="text-[#111418] dark:text-white tracking-light text-[32px] font-bold leading-tight">Crear Nuevo Descuento</p>
                <p class="text-[#617589] dark:text-slate-400 text-sm font-normal leading-normal">Completa la información para registrar un nuevo descuento en el sistema.</p>
              </div>
            </div>
            
            <div class="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-2 p-4">
              <!-- Nombre del descuento -->
              <label class="flex flex-col min-w-40 flex-1 py-3 md:col-span-2">
                <p class="text-[#111418] dark:text-slate-200 text-base font-medium leading-normal pb-2">Nombre *</p>
                <input class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-[#111418] dark:text-white focus:outline-0 focus:ring-0 bg-background-light dark:bg-slate-800 border-gray-200 dark:border-slate-700 h-14 placeholder:text-[#617589] dark:placeholder:text-slate-500 p-[15px] text-base font-normal leading-normal" placeholder="Ej: Descuento de Verano" value=""/>
              </label>
              <!-- Descripción -->
              <label class="flex flex-col min-w-40 flex-1 py-3 md:col-span-2">
                <p class="text-[#111418] dark:text-slate-200 text-base font-medium leading-normal pb-2">Descripción</p>
                <textarea class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-[#111418] dark:text-white focus:outline-0 focus:ring-0 bg-background-light dark:bg-slate-800 border-gray-200 dark:border-slate-700 min-h-36 placeholder:text-[#617589] dark:placeholder:text-slate-500 p-[15px] text-base font-normal leading-normal" placeholder="Describe brevemente en qué consiste el descuento"></textarea>
              </label>
            </div>

            <!-- Section Header -->
            <h3 class="text-[#111418] dark:text-white text-lg font-bold leading-tight tracking-[-0.015em] px-4 pb-2 pt-4">Detalles del Descuento</h3>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-x-8 p-4">
              <!-- Tipo -->
              <div class="flex flex-col py-3">
                <p class="text-[#111418] dark:text-slate-200 text-base font-medium leading-normal pb-2">Tipo</p>
                <div class="flex h-14 items-center justify-center rounded-lg bg-[#f0f2f4] dark:bg-slate-800 p-1">
                  <label class="flex cursor-pointer h-full grow items-center justify-center overflow-hidden rounded-lg px-2 has-[:checked]:bg-white has-[:checked]:dark:bg-slate-700 has-[:checked]:shadow-[0_0_4px_rgba(0,0,0,0.1)] has-[:checked]:text-[#111418] dark:has-[:checked]:text-white text-[#617589] dark:text-slate-400 text-sm font-medium leading-normal transition-all">
                    <span class="truncate">Porcentaje</span>
                    <input checked="" class="invisible w-0" name="discount-type" type="radio" value="Porcentaje"/>
                  </label>
                  <label class="flex cursor-pointer h-full grow items-center justify-center overflow-hidden rounded-lg px-2 has-[:checked]:bg-white has-[:checked]:dark:bg-slate-700 has-[:checked]:shadow-[0_0_4px_rgba(0,0,0,0.1)] has-[:checked]:text-[#111418] dark:has-[:checked]:text-white text-[#617589] dark:text-slate-400 text-sm font-medium leading-normal transition-all">
                    <span class="truncate">Monto Fijo</span>
                    <input class="invisible w-0" name="discount-type" type="radio" value="Monto Fijo"/>
                  </label>
                </div>
              </div>
              <!-- Valor -->
              <label class="flex flex-col min-w-40 flex-1 py-3">
                <p class="text-[#111418] dark:text-slate-200 text-base font-medium leading-normal pb-2">Valor *</p>
                <div class="relative flex items-center">
                  <input class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-[#111418] dark:text-white focus:outline-0 focus:ring-0 bg-background-light dark:bg-slate-800 border-gray-200 dark:border-slate-700 h-14 placeholder:text-[#617589] dark:placeholder:text-slate-500 px-4 pl-10 text-base font-normal leading-normal" placeholder="0" type="number"/>
                  <span class="material-symbols-outlined absolute left-3 text-slate-400" data-icon="percent">percent</span>
                </div>
              </label>
            </div>

            <!-- Section Header -->
            <h3 class="text-[#111418] dark:text-white text-lg font-bold leading-tight tracking-[-0.015em] px-4 pb-2 pt-4">Condiciones de Aplicación</h3>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-x-8 p-4">
              <!-- Aplica a -->
              <label class="flex flex-col min-w-40 flex-1 py-3">
                <p class="text-[#111418] dark:text-slate-200 text-base font-medium leading-normal pb-2">Aplica a</p>
                <div class="relative">
                  <select class="form-input appearance-none flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-[#111418] dark:text-white focus:outline-0 focus:ring-0 bg-background-light dark:bg-slate-800 border-gray-200 dark:border-slate-700 h-14 placeholder:text-[#617589] dark:placeholder:text-slate-500 p-[15px] text-base font-normal leading-normal pr-10">
                    <option>Global</option>
                    <option>Producto</option>
                    <option>Categoría</option>
                    <option>Marca</option>
                    <option>Familia</option>
                  </select>
                  <span class="material-symbols-outlined absolute right-3 top-1/2 -translate-y-1/2 text-slate-400 pointer-events-none" data-icon="expand_more">expand_more</span>
                </div>
              </label>
              <!-- Producto Específico -->
              <label class="flex flex-col min-w-40 flex-1 py-3">
                <p class="text-[#111418] dark:text-slate-200 text-base font-medium leading-normal pb-2">Producto Específico</p>
                <div class="relative">
                  <select class="form-input appearance-none flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-[#111418] dark:text-white focus:outline-0 focus:ring-0 bg-background-light dark:bg-slate-800 border-gray-200 dark:border-slate-700 h-14 placeholder:text-[#617589] dark:placeholder:text-slate-500 p-[15px] text-base font-normal leading-normal pr-10">
                    <option>Seleccionar producto...</option>
                    <option>Procesador Intel Core i9</option>
                    <option>Tarjeta Gráfica NVIDIA RTX 4090</option>
                    <option>Memoria RAM Corsair Vengeance 32GB</option>
                  </select>
                  <span class="material-symbols-outlined absolute right-3 top-1/2 -translate-y-1/2 text-slate-400 pointer-events-none" data-icon="expand_more">expand_more</span>
                </div>
              </label>
              <!-- Compra mínima -->
              <label class="flex flex-col min-w-40 flex-1 py-3">
                <p class="text-[#111418] dark:text-slate-200 text-base font-medium leading-normal pb-2">Compra mínima (opcional)</p>
                <div class="relative flex items-center">
                  <input class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-[#111418] dark:text-white focus:outline-0 focus:ring-0 bg-background-light dark:bg-slate-800 border-gray-200 dark:border-slate-700 h-14 placeholder:text-[#617589] dark:placeholder:text-slate-500 px-4 pl-10 text-base font-normal leading-normal" placeholder="0.00" type="number"/>
                  <span class="material-symbols-outlined absolute left-3 text-slate-400" data-icon="attach_money">attach_money</span>
                </div>
              </label>
              <!-- Cantidad mínima -->
              <label class="flex flex-col min-w-40 flex-1 py-3">
                <p class="text-[#111418] dark:text-slate-200 text-base font-medium leading-normal pb-2">Cantidad mínima (opcional)</p>
                <input class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-[#111418] dark:text-white focus:outline-0 focus:ring-0 bg-background-light dark:bg-slate-800 border-gray-200 dark:border-slate-700 h-14 placeholder:text-[#617589] dark:placeholder:text-slate-500 p-[15px] text-base font-normal leading-normal" placeholder="1" type="number"/>
              </label>
              <!-- Usos máximos -->
              <label class="flex flex-col min-w-40 flex-1 py-3 md:col-span-2">
                <p class="text-[#111418] dark:text-slate-200 text-base font-medium leading-normal pb-2">Usos máximos (opcional)</p>
                <input class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-[#111418] dark:text-white focus:outline-0 focus:ring-0 bg-background-light dark:bg-slate-800 border-gray-200 dark:border-slate-700 h-14 placeholder:text-[#617589] dark:placeholder:text-slate-500 p-[15px] text-base font-normal leading-normal" placeholder="Dejar vacío para ilimitado" type="number"/>
              </label>
            </div>

            <!-- Section Header -->
            <h3 class="text-[#111418] dark:text-white text-lg font-bold leading-tight tracking-[-0.015em] px-4 pb-2 pt-4">Vigencia y Estado</h3>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-x-8 p-4">
              <!-- Fecha inicio -->
              <label class="flex flex-col min-w-40 flex-1 py-3">
                <p class="text-[#111418] dark:text-slate-200 text-base font-medium leading-normal pb-2">Fecha de inicio</p>
                <div class="relative">
                  <input class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-[#111418] dark:text-white focus:outline-0 focus:ring-0 bg-background-light dark:bg-slate-800 border-gray-200 dark:border-slate-700 h-14 placeholder:text-[#617589] dark:placeholder:text-slate-500 pl-12 pr-4 text-base font-normal leading-normal" type="datetime-local"/>
                  <span class="material-symbols-outlined absolute left-3 top-1/2 -translate-y-1/2 text-slate-400 pointer-events-none" data-icon="calendar_today">calendar_today</span>
                </div>
              </label>
              <!-- Fecha fin -->
              <label class="flex flex-col min-w-40 flex-1 py-3">
                <p class="text-[#111418] dark:text-slate-200 text-base font-medium leading-normal pb-2">Fecha de fin</p>
                <div class="relative">
                  <input class="form-input flex w-full min-w-0 flex-1 resize-none overflow-hidden rounded-lg text-[#111418] dark:text-white focus:outline-0 focus:ring-0 bg-background-light dark:bg-slate-800 border-gray-200 dark:border-slate-700 h-14 placeholder:text-[#617589] dark:placeholder:text-slate-500 pl-12 pr-4 text-base font-normal leading-normal" type="datetime-local"/>
                  <span class="material-symbols-outlined absolute left-3 top-1/2 -translate-y-1/2 text-slate-400 pointer-events-none" data-icon="event_busy">event_busy</span>
                </div>
              </label>
              <!-- Estado activo -->
              <div class="flex items-center justify-between min-w-40 flex-1 py-3 md:col-span-2">
                <p class="text-[#111418] dark:text-slate-200 text-base font-medium leading-normal">Estado activo</p>
                <label class="relative inline-flex items-center cursor-pointer">
                  <input checked="" class="sr-only peer" type="checkbox" value=""/>
                  <div class="w-11 h-6 bg-slate-200 dark:bg-slate-700 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-slate-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-primary"></div>
                </label>
              </div>
            </div>

            <!-- Action Buttons -->
            <div class="flex justify-end gap-4 p-4 mt-6 border-t border-gray-200 dark:border-slate-800">
              <a href="/gestion-descuentos" class="flex items-center justify-center h-12 px-6 rounded-lg bg-slate-100 dark:bg-slate-700 text-[#111418] dark:text-white text-base font-medium hover:bg-slate-200 dark:hover:bg-slate-600 transition-colors">Cancelar</a>
              <button class="flex items-center justify-center h-12 px-6 rounded-lg bg-primary text-white text-base font-medium hover:bg-primary/90 transition-colors">Guardar Descuento</button>
            </div>
          </div>
        </div>
      </div>
    </main>
  </div>
</div>
