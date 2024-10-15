<template>
  <div class="profile-header text-dark">
    <div class="row justify-content-center">
      <div class="col-auto">
        <div class="position-relative">
          <div
            class="profile-picture bg-secondary rounded-circle d-flex align-items-center justify-content-center"
            style="width: 100px; height: min-content"
          >
            <img
              v-if="perfil.imagen == undefined"
              alt="Profile"
              src="../../assets/logo.png"
              class="rounded-circle"
              style="width: 100px; height: 100px; object-fit: cover"
            />
            <img
              v-else
              alt="Profile"
              :src="`${perfil.imagen}`"
              class="rounded-circle"
              style="width: 100px; height: 100px; object-fit: cover"
            />
          </div>
        </div>
      </div>
      <div class="col-8">
        <div class="d-flex justify-content-between gap-4 align-items-center mb-2">
          <h4 class="mb-0 fw-semibold">{{ perfil.nombres }}</h4>
          <div>
            <button
              class="btn btn-outline-primary p-0 m-0 px-3"
              data-bs-toggle="modal"
              data-bs-target="#editProfileModal"
            >
              <IconEdit size="18" />
            </button>
            <ModalPerfil :perfil="perfil" />
          </div>
        </div>
        <div
          class="d-flex mb-2 flex-wrap row-gap-0 column-gap-2 p-0 m-0 justify-content-center justify-content-sm-start justify-content-md-start"
        >
          <div class="me-4 info p-0 m-0" v-if="perfil.direccion != null">
            <IconHome2 size="24" />
            <span>{{ perfil.direccion }}</span>
          </div>
          <div class="me-4 info" v-if="perfil.telefono != null">
            <IconPhone size="24" />
            <span>{{ perfil.telefono }}</span>
          </div>
          <div class="me-4 info" v-if="perfil.correo != null">
            <IconMailAi size="24" />
            <span>{{ perfil.correo }}</span>
          </div>
          <div class="me-4 info" v-if="perfil.nacimiento != null">
            <IconCake size="24" />
            <span>{{ perfil.nacimiento }}</span>
          </div>
          <div class="me-4 info" v-if="perfil.ruc != null">
            <IconBuildingBank size="24" />
            <span>{{ perfil.ruc }}</span>
          </div>
        </div>
        <p class="mb-0 text-muted">@{{ perfil.dni }}</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import ModalPerfil from './modal.vue'
import {
  IconHome2,
  IconPhone,
  IconMailAi,
  IconCake,
  IconBuildingBank,
  IconEdit
} from '@tabler/icons-vue'

defineProps({
  perfil: { type: Object, required: true }
})
</script>

<style lang="scss" scoped>
.profile-header {
  max-width: 100%;
  padding: 0 !important;
  margin: 0 !important;
  height: min-content;

  .row {
    padding: 0 !important;
    margin: 0 !important;
    align-items: start;
    justify-content: start;
    justify-items: start;
  }
}

.info {
  display: flex;
  justify-content: center;
  align-items: center;
  font-size: 0.874rem;
}
</style>
