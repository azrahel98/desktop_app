<template>
  <div
    class="modal modal-md fade"
    id="editProfileModal"
    tabindex="-1"
    aria-labelledby="editProfileModalLabel"
    aria-hidden="true"
    ref="editProfileModal"
  >
    <div class="modal-dialog modal-dialog-scrollable">
      <div class="modal-content px-4 py-3 shadow-sm rounded-lg">
        <div class="modal-header border-0">
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
            <div class="text-center mb-4">
              <div class="position-relative d-inline-block">
                <img
                  v-if="imagePreview"
                  :src="imagePreview"
                  alt="Profile"
                  class="rounded-circle shadow-sm"
                  style="width: 100px; height: 100px; object-fit: cover"
                />
                <img
                  v-else-if="perfil.imagen"
                  :src="perfil.imagen"
                  alt="Profile"
                  class="rounded-circle shadow-sm"
                  style="width: 100px; height: 100px; object-fit: cover"
                />
                <img
                  v-else
                  src="../../assets/logo.png"
                  alt="Profile"
                  class="rounded-circle shadow-sm"
                  style="width: 100px; height: 100px; object-fit: cover"
                />
                <label
                  for="profilePicture"
                  class="position-absolute bottom-0 end-0 bg-light rounded-circle p-1 cursor-pointer shadow-sm border border-2"
                >
                  <IconCamera size="24" />
                  <input
                    id="profilePicture"
                    type="file"
                    @change="handleFileChange"
                    accept="image/*"
                    class="d-none"
                  />
                </label>
              </div>
            </div>

            <div class="row">
              <div class="col-md-6 mb-3">
                <label for="name" class="form-label fw-semibold">Nombres</label>
                <input
                  id="name"
                  type="text"
                  class="form-control"
                  :value="perfil.nombres"
                  disabled
                />
              </div>

              <div class="col-md-6 mb-3">
                <label for="direccion" class="form-label fw-semibold">Dirección</label>
                <input
                  id="direccion"
                  type="text"
                  class="form-control"
                  v-model="perfil.direccion"
                  required
                />
              </div>

              <div class="col-md-6 mb-3">
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

              <div class="col-md-6 mb-3">
                <label for="email" class="form-label fw-semibold">Email</label>
                <input id="email" type="email" class="form-control" v-model="perfil.correo" />
              </div>

              <div class="col-md-6 mb-3">
                <label for="ruc" class="form-label fw-semibold">RUC</label>
                <input id="ruc" type="number" class="form-control" v-model="perfil.ruc" />
              </div>

              <div class="col-md-6 mb-3">
                <label for="birthDate" class="form-label fw-semibold">Cumpleaños</label>
                <input
                  id="birthDate"
                  type="date"
                  class="form-control"
                  v-model="perfil.nacimiento"
                  required
                />
              </div>
            </div>

            <div class="d-flex justify-content-end mt-4">
              <button type="button" class="btn btn-secondary me-2" data-bs-dismiss="modal">
                Cancelar
              </button>
              <button type="submit" class="btn btn-primary">Guardar cambios</button>
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
const imagePreview = ref<string | undefined>(undefined)

const handleFileChange = (e: Event) => {
  const target = e.target as HTMLInputElement
  if (target.files && target.files.length > 0) {
    const file = target.files[0]
    const reader = new FileReader()
    reader.onload = (event) => {
      imagePreview.value = event.target!.result as string
    }
    reader.readAsDataURL(file)
  }
}

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
    // Código para cerrar el modal...
  } catch (error) {
    console.error(error)
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

@media (max-width: 576px) {
  .form-control {
    font-size: 12px;
  }
}
</style>
