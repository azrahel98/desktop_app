<template>
  <div class="main">
    <div class="row">
      <div class="col-sm-1 col-md-2 col-g-4">
        <div class="avatar rounded-3 w-100">
          <img
            v-if="!perfil.imagen"
            src="../../assets/mann.svg"
            class="rounded-circle img-fluid"
            style="object-fit: fill"
          />
          <img
            v-else
            :src="perfil.imagen"
            :alt="perfil.nombres"
            class="rounded-circle img-fluid"
            style="width: 100px; height: 100px; object-fit: cover"
          />
        </div>
      </div>
      <div class="col-sm-1 col-md-1 col-lg-2 text-center">
        <button
          class="btn btn-outline-primary btn-icon btn-md"
          data-bs-toggle="modal"
          data-bs-target="#editProfileModal"
        >
          <IconEdit size="18" />
        </button>
        <ModalPerfil class="text-start" :perfil="perfil" />
      </div>
      <div class="col-sm-12 col-md-9 h-100 col-lg-6 text-start">
        <div class="card">
          <div class="card-body pb-1 pt-3 d-flex flex-column">
            <div class="" v-if="perfil.ruc">
              <IconClipboardList class="icon me-2 text-secondary" />
              Ruc : <strong>{{ perfil.ruc }}</strong>
            </div>
            <div class="" v-if="perfil.correo">
              <IconMail class="icon me-2 text-secondary" />
              Correo : <strong>{{ perfil.correo }}</strong>
            </div>
            <div class="" v-if="perfil.direccion">
              <IconHome class="icon me-2 text-secondary" />
              Vive en: <strong>{{ perfil.direccion }}</strong>
            </div>
            <div class="" v-if="perfil.telefono">
              <IconPhoneCall class="icon me-2 text-secondary" />
              Telefono: <strong>{{ perfil.telefono }}</strong>
            </div>
            <div class="" v-if="perfil.nacimiento">
              <IconCalendar class="icon text-secondary me-2" />
              Cumpleaños: <strong>{{ formatDate(perfil.nacimiento, 'dd MMM yyyy') }}</strong>
            </div>
            <div v-if="perfil.dni">
              <IconCreditCard class="icon text-secondary me-2" />
              Dni: <strong>{{ perfil.dni }}</strong>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { formatDate } from 'date-fns'
import ModalPerfil from './modal.vue'
import {
  IconHome,
  IconEdit,
  IconCalendar,
  IconClipboardList,
  IconMail,
  IconPhoneCall,
  IconCreditCard
} from '@tabler/icons-vue'

defineProps({
  perfil: { type: Object, required: true }
})
</script>

<style lang="scss" scoped>
.main {
  display: flex;
  justify-content: center;
  flex-direction: column;
  justify-items: center;
  max-width: 1000px;
  .row {
    justify-content: space-around;

    .avatar {
      height: 100%;
      img {
        height: 20vh;
        width: 100%;
      }
    }
    .card {
      overflow-y: auto;
      height: max-content;
      padding-bottom: 1.4vh;
      .card-body {
        justify-content: start;
        align-content: start;
      }
    }
  }
}
</style>
