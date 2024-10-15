<template>
  <div
    class="modal modal-md fade"
    id="editProfileModal"
    tabindex="-1"
    aria-labelledby="editProfileModal"
    aria-hidden="true"
    ref="editProfileModal"
  >
    <div class="modal-dialog modal-dialog-scrollable">
      <div class="modal-content px-4 py-1 shadow-sm rounded-lg">
        <div class="modal-header border-0 mb-2">
          <h5 class="modal-title fw-bold" id="editProfileModalLabel">Editar Perfil</h5>
          <button
            type="button"
            class="btn-close"
            data-bs-dismiss="modal"
            aria-label="Close"
          ></button>
        </div>
        <div class="modal-body">
          <form @submit.prevent="datos(perfil)" class="needs-validation" novalidate>
            <!-- Foto de perfil -->
            <div class="text-center mb-4">
              <div class="position-relative d-inline-block">
                <img
                  v-if="imagePreview !== undefined"
                  alt="Profile"
                  :src="imagePreview"
                  class="rounded-circle shadow-sm"
                  style="width: 100px; height: 100px; object-fit: cover"
                />
                <img
                  v-else-if="perfil.imagen"
                  alt="Profile"
                  :src="`${perfil.imagen}`"
                  class="rounded-circle shadow-sm"
                  style="width: 100px; height: 100px; object-fit: cover"
                />
                <img
                  v-else
                  alt="Profile"
                  src="../../assets/logo.png"
                  class="rounded-circle shadow-sm"
                  style="width: 100px; height: 100px; object-fit: cover"
                />
                <label
                  for="profilePicture"
                  class="position-absolute bottom-0 end-0 bg-light rounded-circle p-1 cursor-pointer shadow-sm"
                >
                  <IconCamera size="24" />
                  <input
                    id="profilePicture"
                    type="file"
                    @change="(e) => handleFileChange(e)"
                    accept="image/*"
                    class="d-none"
                  />
                </label>
              </div>
            </div>

            <!-- Campos del formulario -->
            <div class="mb-3">
              <label for="name" class="form-label fw-semibold">Nombres</label>
              <input id="name" type="text" class="form-control" :value="perfil.nombres" disabled />
            </div>

            <div class="mb-3">
              <label for="direccion" class="form-label fw-semibold">Dirección</label>
              <input
                id="direccion"
                type="text"
                class="form-control"
                v-model="perfil.direccion"
                required
              />
            </div>

            <div class="mb-3">
              <label for="telefono" class="form-label fw-semibold">Teléfono</label>
              <input
                id="telefono"
                type="tel"
                min="8"
                class="form-control"
                v-model="perfil.telefono"
                required
              />
            </div>

            <div class="mb-3">
              <label for="email" class="form-label fw-semibold">Email</label>
              <input id="email" type="email" class="form-control" v-model="perfil.correo" />
            </div>

            <div class="mb-3">
              <label for="ruc" class="form-label fw-semibold">RUC</label>
              <input id="ruc" type="number" class="form-control" v-model="perfil.ruc" />
            </div>

            <div class="mb-3">
              <label for="birthDate" class="form-label fw-semibold">Cumpleaños</label>
              <input
                id="birthDate"
                type="date"
                class="form-control"
                v-model="perfil.nacimiento"
                required
              />
            </div>

            <div class="d-flex justify-content-end mt-4">
              <button type="button" class="btn btn-secondary me-2" data-bs-dismiss="modal">
                Cancelar
              </button>
              <button type="submit" class="btn btn-primary text-white">Guardar cambios</button>
            </div>
          </form>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { IconCamera } from '@tabler/icons-vue'
import { invoke } from '@tauri-apps/api/core'

defineProps({
  perfil: { type: Object, required: true }
})
const editProfileModal = ref<HTMLElement | null>(null)
const datos = async (x: any) => {
  try {
    await invoke('actualizar_trabajador', {
      body: {
        direccion: x.direccion,
        telefono: x.telefono,
        correo: x.correo,
        nacimiento: x.nacimiento,
        ruc: x.ruc,
        imagen: imagePreview.value,
        dni: x.dni
      }
    })
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

const file = ref<File | null>(null)
const imagePreview = ref<string | undefined>(undefined)

const handleFileChange = (e: Event) => {
  const target = e.target as HTMLInputElement
  if (target.files && target.files.length > 0) {
    file.value = target.files[0]
    if (file.value) {
      const reader = new FileReader()
      reader.onload = function (e) {
        const base64 = e.target!.result as string

        imagePreview.value = base64
      }
      reader.readAsDataURL(file.value)
    }
  }
  console.log(imagePreview.value)
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
