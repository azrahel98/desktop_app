<template>
  <div
    class="modal modal-md fade"
    id="legajoAdd"
    tabindex="-1"
    aria-labelledby="legajoAdd"
    aria-hidden="true"
    ref="editProfileModal"
  >
    <div class="modal-dialog modal-dialog-centered">
      <div class="modal-content px-4 py-1 shadow-sm rounded-lg">
        <div class="modal-header border-0 mb-2">
          <h5 class="modal-title fw-bold" id="editProfileModalLabel">Agregar Prestamo</h5>
          <button
            type="button"
            class="btn-close"
            data-bs-dismiss="modal"
            aria-label="Close"
          ></button>
        </div>
        <div class="modal-body">
          <form @submit.prevent="datos(perfil)" class="needs-validation" novalidate>
            <div class="mb-3">
              <label for="name" class="form-label fw-semibold">Nombres</label>
              <input id="name" type="text" class="form-control" :value="perfil.nombres" disabled />
            </div>

            <div class="mb-3">
              <label for="direccion" class="form-label fw-semibold">Ubicacion de Origen</label>
              <select
                class="form-select"
                aria-label="Default select example"
                v-model="legajo.ubicacion_origen"
              >
                <option value="Archivo">Armario</option>
                <option value="Piso 1">Oficina</option>
                <option value="Piso 2" selected>Cueva</option>
              </select>
            </div>

            <div class="mb-3">
              <label for="telefono" class="form-label fw-semibold">Motivo de Prestamo</label>
              <div class="form-floating">
                <textarea
                  class="form-control"
                  placeholder="Leave a comment here"
                  id="floatingTextarea"
                  v-model="legajo.prestamo"
                />
                <label for="floatingTextarea">..</label>
              </div>
            </div>
            <div class="mb-3">
              <label for="fecha" class="form-label fw-semibold">Fecha</label>
              <input
                id="fecha"
                type="datetime-local"
                min="8"
                class="form-control"
                v-model="legajo.fechaprestamo"
                required
              />
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
import { reactive, ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { userStore } from '../../store/user'

const usTore = userStore()
const emit = defineEmits(['change'])
defineProps({
  perfil: { type: Object, required: true }
})

const legajo = reactive({
  ubicacion_origen: '',
  prestamo: '',
  usuario: usTore.id,
  trabajador: '',
  fechaprestamo: ''
})

const editProfileModal = ref<HTMLElement | null>(null)
const datos = async (x: any) => {
  try {
    await invoke('añadir_prestamo', {
      body: {
        ubicacion_origen: legajo.ubicacion_origen,
        prestamo: legajo.prestamo,
        usuario: usTore.id,
        trabajador: x.dni,
        fechaprestamo: legajo.fechaprestamo.replace('T', ' ') + ':00'
      }
    })
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
