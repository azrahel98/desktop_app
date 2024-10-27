<template>
  <div class="card rounded-2 pb-0 mb-0">
    <div class="card-body">
      <div class="d-flex">
        <div>
          <h5 class="card-title mb-3">{{ job.area }}</h5>
          <h6 class="card-subtitle text-muted">{{ job.cargo }}</h6>
        </div>
        <div v-if="!job.renuncia">
          <Renuncia @submit="e" :id="job.id" />
        </div>
      </div>
      <ul class="list-unstyled">
        <li class="mb-1">
          <IconCalendar size="20" class="text-primary fw-bold" />
          <span class="text-muted"> Ingreso:</span>
          {{ formatDate(addDays(job.ingreso, 1), 'dd MMM yyyy') }}
        </li>
        <li class="mb-1">
          <IconCashBanknote size="20" />
          <span class="text-muted"> Sueldo:</span> S/.{{ job.sueldo }}
        </li>
        <li class="mb-1">
          <IconCapStraight size="20" />
          <span class="text-muted"> Conv:</span>
          {{ job.convocatoria == null ? (job.cas as string).toLowerCase() : job.convocatoria }}
        </li>
        <li class="mb-1">
          <IconBriefcase size="20" />
          <span class="text-muted"> Régimen:</span>
          {{ job.regimen }}
        </li>
        <li v-if="job.renuncia" class="mb-1">
          <IconArrowRight size="20" class="text-danger" />
          <span class="badge bg-youtube text-white"> Renuncia:</span>
          {{ formatDate(addDays(job.hvb, 1), 'dd MMM yyyy') }}
        </li>
      </ul>
    </div>
    <div class="card-status-top" :class="[job.activo === 'N' ? 'bg-youtube' : 'bg-facebook']"></div>
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
import { addDays, formatDate } from 'date-fns'
import Renuncia from './renuncia.vue'

defineProps({
  job: { type: Object, required: true }
})

const emit = defineEmits(['submit'])

const e = () => {
  emit('submit')
}
</script>

<style lang="scss" scoped>
.card {
  width: 100%;
  max-width: 230px;
  min-height: 300px;
  height: min-content;
  display: grid;
  height: min-content;
  background-color: white;
  grid-template-rows: auto min-content;
  .card-body {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
  }
  .card-title {
    font-size: 1.32rcap;
    font-weight: bold;
    white-space: break-spaces;
    vertical-align: middle;
    padding: 0;
    margin: 0;
    line-height: 1.2;
  }
  .card-subtitle {
    font-size: 1.2rcap;
  }
  .mb-1 {
    font-size: 1.1rcap;
    font-weight: 500;
    span {
      font-size: 1.2rcap;
      text-transform: capitalize;
      font-weight: 600;
    }
  }
}
</style>
