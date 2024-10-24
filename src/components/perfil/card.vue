<template>
  <div class="card pb-0 mb-0">
    <div class="card-body">
      <div>
        <h5 class="card-title">{{ job.area }}</h5>
        <h6 class="card-subtitle text-muted">{{ job.cargo }}</h6>
      </div>
      <ul class="list-unstyled">
        <li class="mb-2">
          <IconCalendar size="20" class="text-primary fw-bold" />
          <span class="text-muted"> Ingreso:</span> {{ job.ingreso }}
        </li>
        <li class="mb-2">
          <IconCashBanknote size="20" />
          <span class="text-muted"> Sueldo:</span> {{ job.sueldo }}
        </li>
        <li class="mb-2">
          <IconCapStraight size="20" />
          <span class="text-muted"> Conv:</span>
          {{ job.convocatoria == null ? job.cas : job.convocatoria }}
        </li>
        <li class="mb-2">
          <IconBriefcase size="20" />
          <span class="text-muted"> Régimen:</span>
          {{ job.regimen }}
        </li>
        <li v-if="job.renuncia" class="mb-2">
          <IconArrowRight size="20" class="text-danger" />
          <span class="text-danger"> Renuncia:</span>
          {{ job.renuncia }}
        </li>
      </ul>
    </div>
    <div
      class="card-footer p-2"
      :class="[job.activo === 'N' ? 'bg-danger-subtle' : 'bg-facebook-lt']"
    >
      <small
        class=""
        :class="[job.activo === 'N' ? 'text-danger fw-bold' : 'text-facebook fw-bold']"
      >
        {{ job.activo === 'N' ? 'Finalizado' : 'Activo' }}
      </small>
    </div>
  </div>
</template>

<script setup lang="ts">
import {
  IconCalendar,
  IconCashBanknote,
  IconBriefcase,
  IconArrowRight,
  IconCapStraight
} from '@tabler/icons-vue'

defineProps({
  job: { type: Object, required: true }
})
</script>

<style lang="scss" scoped>
.card {
  width: 100%;
  max-width: 230px;
  height: 100%;
  min-height: 30vh;
  display: grid;
  height: min-content;
  grid-template-rows: auto min-content;
  .card-body {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    row-gap: 2vh;
  }
  .card-title {
    font-size: 0.9rem;
    font-weight: bold;
  }
  .card-subtitle {
    font-size: 0.8rem;
  }
  .mb-2 {
    font-size: 1.1rcap;
    font-weight: 500;
    span {
      font-weight: 600;
    }
  }
}
</style>
