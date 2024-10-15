<template>
  <div
    class="modal modal-md fade"
    :id="`devuelve-${id}`"
    tabindex="-1"
    aria-labelledby="devuelve"
    aria-hidden="true"
    ref="editProfileModal"
  >
    <div class="modal-dialog modal-dialog-centered">
      <div class="modal-content px-4 py-1 shadow-sm rounded-lg">
        <div class="modal-header border-0 mb-2">
          <h5 class="modal-title fw-bold" id="editProfileModalLabel">Devolucion</h5>
          <button
            type="button"
            class="btn-close"
            data-bs-dismiss="modal"
            aria-label="Close"
          ></button>
        </div>
        <div class="modal-body">
          <form
            @submit.prevent="datos(id, prestamo, fechaprestamo)"
            class="needs-validation"
            novalidate
          >
            <div class="mb-3">
              <label for="name" class="form-label fw-semibold">Motivo</label>
              <input id="name" type="text" class="form-control" :value="prestamo" disabled />
            </div>

            <div class="mb-3">
              <label for="fecha" class="form-label fw-semibold">Fecha</label>
              <input
                id="fecha"
                type="datetime-local"
                min="8"
                class="form-control"
                :class="[error_fecha ? 'is-invalid' : '']"
                v-model="fecha"
                required
              />
              <span class="text-danger">{{ error_fecha }}</span>
            </div>

            <div class="d-flex justify-content-end mt-4">
              <button type="button" class="btn btn-secondary me-2" data-bs-dismiss="modal">
                Cancelar
              </button>
              <button type="submit" class="btn btn-primary text-white">Agregar Prestamo</button>
            </div>
          </form>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { userStore } from '../../store/user'
import { format, isAfter, parse } from 'date-fns'

const usTore = userStore()

defineProps({
  prestamo: { type: String, required: true },
  id: { type: Number, required: true },
  fechaprestamo: { type: String, required: true }
})

const error_fecha = ref('')
const fecha = ref()
const emit = defineEmits(['change'])

const editProfileModal = ref<HTMLElement | null>(null)
const datos = async (x: Number, prestamo: string, fechaprestamo: string) => {
  try {
    const fecha_prestamo = parse(
      format(fechaprestamo, 'dd/MM/yyyy HH:mm'),
      'dd/MM/yyyy HH:mm',
      new Date()
    )
    const ahora = parse(format(fecha.value, 'dd/MM/yyyy HH:mm'), 'dd/MM/yyyy HH:mm', new Date())

    if (isAfter(ahora, fecha_prestamo)) {
      await invoke('editar_prestamo', {
        body: {
          id: x,
          prestamo,
          usuario: usTore.id,
          devuelto: fecha.value.replace('T', ' ') + ':00',
          fechaprestamo: fechaprestamo
        }
      })
      fecha.value = ''
      emit('change')
      if (editProfileModal.value) {
        editProfileModal.value.classList.remove('show')
        editProfileModal.value.setAttribute('aria-hidden', 'true')
        editProfileModal.value.style.display = 'none'
        editProfileModal.value.removeAttribute('aria-modal')
        editProfileModal.value.removeAttribute('role')

        const backdrops = document.querySelectorAll('.modal-backdrop')
        backdrops.forEach((backdrop) => backdrop.remove())

        document.body.classList.remove('modal-open')
        document.body.removeAttribute('data-bs-overflow')
        document.body.removeAttribute('data-bs-padding-right')

        document.body.style.overflow = ''
        document.body.style.paddingRight = ''
      }
    } else {
      error_fecha.value = 'La fecha no puede ser anterior del prestamo'
    }
  } catch (error) {
    console.log(error)
  }
}
</script>

<style lang="scss" scoped>
input {
  font-size: 14px;
  font-weight: 500;
}
.modal-content {
  border-radius: 1rem;
}

.form-label {
  color: #4a4a4a;
}

.shadow-sm {
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
}

.cursor-pointer {
  cursor: pointer;
}
</style>
