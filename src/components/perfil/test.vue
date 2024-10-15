<template>
  <div class="card">
    <div class="card-header bg-primary text-white">
      <h5 class="mb-0">Historial</h5>
    </div>
    <div class="card-body">
      <ul class="list-group list-group-flush">
        <li v-for="(item, index) in historialItems" :key="index" class="list-group-item">
          <div class="d-flex justify-content-between align-items-center">
            <div>
              <h6 class="mb-1">{{ item.title }}</h6>
              <small class="text-muted">
                <i class="bi bi-clock me-1"></i>{{ item.startDate }}
              </small>
            </div>
            <div class="text-end">
              <span :class="getStatusClass(item.status)">
                <i :class="getStatusIcon(item.status)" class="me-1"></i>
                {{ item.status }}
              </span>
              <br />
              <small class="text-muted">
                <i class="bi bi-calendar me-1"></i>{{ item.endDate }}
              </small>
            </div>
          </div>
        </li>
      </ul>
    </div>
  </div>
</template>

<script>
export default {
  name: 'HistorialCard',
  data() {
    return {
      historialItems: [
        {
          title: 'Prestamo a Nino',
          startDate: '2024-10-08 20:16:00',
          endDate: '2024-10-11 08:52:00',
          status: 'Completado'
        },
        {
          title: 'ADSFADSF',
          startDate: '2024-10-10 20:00:00',
          endDate: '2024-10-10 20:49:00',
          status: 'En Progreso'
        },
        {
          title: 'adfadf',
          startDate: '2024-10-11 07:46:00',
          endDate: '2024-10-12 06:58:00',
          status: 'Pendiente'
        },
        {
          title: 'adfadfadsf',
          startDate: '2024-10-11 06:01:00',
          endDate: '2024-10-11 07:02:00',
          status: 'Cancelado'
        }
      ]
    }
  },
  methods: {
    getStatusClass(status) {
      switch (status) {
        case 'Completado':
          return 'text-success'
        case 'En Progreso':
          return 'text-primary'
        case 'Pendiente':
          return 'text-warning'
        case 'Cancelado':
          return 'text-danger'
        default:
          return 'text-muted'
      }
    },
    getStatusIcon(status) {
      switch (status) {
        case 'Completado':
          return 'bi bi-check-circle'
        case 'En Progreso':
          return 'bi bi-play-circle'
        case 'Pendiente':
          return 'bi bi-clock'
        case 'Cancelado':
          return 'bi bi-x-circle'
        default:
          return 'bi bi-question-circle'
      }
    }
  }
}
</script>

<style scoped>
.card {
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}

.list-group-item {
  transition: background-color 0.3s ease;
}

.list-group-item:hover {
  background-color: #f8f9fa;
}
</style>
