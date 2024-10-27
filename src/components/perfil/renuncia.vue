<template>
  <div>
    <!-- Botón para abrir el modal -->
    <button
      class="btn btn-sm btn-warning d-flex align-items-center"
      data-bs-toggle="modal"
      data-bs-target="#modalRenuncia"
    >
      <IconCalendar size="15" />
    </button>

    <!-- Modal -->
    <div
      class="modal fade"
      id="modalRenuncia"
      tabindex="-1"
      aria-labelledby="modalRenunciaLabel"
      aria-hidden="true"
      ref="modal"
    >
      <div class="modal-dialog modal-sm modal-dialog-centered">
        <div class="modal-content">
          <div class="modal-header py-2">
            <h5 class="modal-title fw-bold" id="modalRenunciaLabel">Registrar Renuncia</h5>
            <button
              type="button"
              class="btn-close"
              data-bs-dismiss="modal"
              aria-label="Close"
            ></button>
          </div>

          <form @submit.prevent="handleSubmit(id)">
            <div class="modal-body py-2">
              <div class="mb-2">
                <label for="fechaRenuncia" class="form-label card-title fw-semibold mb-1">
                  Fecha de Renuncia
                </label>
                <input
                  id="fechaRenuncia"
                  v-model="fecha"
                  type="date"
                  class="form-control"
                  required
                />
              </div>
            </div>

            <div class="modal-footer py-2">
              <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">
                Cancelar
              </button>
              <button type="submit" class="btn btn-primary">Confirmar</button>
            </div>
          </form>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { IconCalendar } from '@tabler/icons-vue'
import { closemodal } from '../../tools/modal'
import { invoke } from '@tauri-apps/api/core'

const emit = defineEmits(['submit'])
defineProps({
  id: { type: Number, required: true }
})
const fecha = ref('')
const modal = ref<HTMLElement | null>(null)

const handleSubmit = async (id: number) => {
  try {
    if (!fecha.value) return
    console.log(fecha.value)
    await invoke('renuncia_trabajador', { id, fecha: fecha.value })
    emit('submit', fecha.value)
    closemodal(modal.value)
    fecha.value = ''
  } catch (error) {}
}
</script>
