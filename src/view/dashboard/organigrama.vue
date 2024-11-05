<template>
  <div class="chart-container">
    <!-- Controls -->
    <div class="controls mb-4">
      <div class="d-flex gap-3 align-items-center">
        <button class="btn btn-light d-flex align-items-center gap-2">
          <i class="bi bi-diagram-2"></i>
          Legend
          <i class="bi bi-info-circle text-muted"></i>
        </button>
        <div class="badge bg-light text-dark border d-flex align-items-center gap-2">
          <i class="bi bi-people-fill text-primary"></i>
          Departamentos ({{ departments.length }})
        </div>
        <div class="ms-auto d-flex align-items-center gap-2">
          <button class="btn btn-light"><i class="bi bi-dash"></i></button>
          <span class="px-3 border rounded">250%</span>
          <button class="btn btn-light"><i class="bi bi-plus"></i></button>
        </div>
      </div>
    </div>

    <!-- Organizational Chart -->
    <div class="org-chart">
      <!-- Level 1: Alcaldía -->
      <div class="level">
        <div class="org-card main-card">
          <div class="card-content">
            <div class="profile">
              <div class="avatar">
                <i class="bi bi-building text-primary"></i>
              </div>
              <div class="info">
                <h6>Alcaldía</h6>
                <small class="text-muted">Gobierno Municipal</small>
              </div>
            </div>
            <div class="tags">
              <span class="tag manager">Principal</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Level 2: Main Departments -->
      <div class="level">
        <div v-for="dept in mainDepartments" :key="dept.id" class="org-card">
          <div class="card-content">
            <div class="profile">
              <div class="avatar">
                <i class="bi bi-briefcase"></i>
              </div>
              <div class="info">
                <h6>{{ dept.name }}</h6>
                <small class="text-muted">{{ dept.id }}</small>
              </div>
            </div>
            <div class="tags">
              <span class="tag department">Departamento</span>
              <span v-if="dept.children" class="tag count">+{{ dept.children.length }}</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Level 3: Sub Departments -->
      <div class="level">
        <div v-for="dept in subDepartments" :key="dept.id" class="org-card">
          <div class="card-content">
            <div class="profile">
              <div class="avatar">
                <i class="bi bi-folder"></i>
              </div>
              <div class="info">
                <h6>{{ dept.name }}</h6>
                <small class="text-muted">{{ dept.id }}</small>
              </div>
            </div>
            <div class="tags">
              <span class="tag sub-department">Subdepartamento</span>
              <span v-if="dept.children" class="tag count">+{{ dept.children.length }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
interface Department {
  id: string
  name: string
  children?: Department[]
}

const departments = [
  {
    id: '1.1',
    name: 'Organo de Control Institucional'
  },
  {
    id: '1.2',
    name: 'Procuraduria Municipal'
  },
  {
    id: '1.3',
    name: 'Oficina de Secretaria General',
    children: [
      { id: '1.3.1', name: 'Unidad de Gestion Documentaria' },
      { id: '1.3.2', name: 'Area de Registro Civil' }
    ]
  },
  {
    id: '1.4',
    name: 'Unidad de Imagen Institucional'
  },
  {
    id: '1.5',
    name: 'Gerencia Municipal',
    children: [
      {
        id: '1.5.1',
        name: 'Oficina General de Administracion',
        children: [
          { id: '1.5.1.1', name: 'Unidad de Gestion de Recursos Humanos' },
          { id: '1.5.1.2', name: 'Unidad de Contabilidad' },
          { id: '1.5.1.3', name: 'Unidad de Tesoreria' },
          { id: '1.5.1.4', name: 'Unidad de Abastecimiento' },
          { id: '1.5.1.5', name: 'Unidad de Desarrollo Tecnologico' }
        ]
      },
      { id: '1.5.2', name: 'Oficina de Asesoria Juridica' },
      {
        id: '1.5.3',
        name: 'Oficina de Planeamiento y Presupuesto',
        children: [{ id: '1.5.3.1', name: 'Unidad de Planeamiento Estrategico' }]
      }
    ]
  }
]

const mainDepartments = departments.filter((dept) => !dept.id.includes('.', 3))
const subDepartments = departments
  .filter((dept) => dept.id.includes('.', 3))
  .concat(departments.flatMap((dept) => dept.children || []))
</script>

<style scoped>
.chart-container {
  padding: 2rem;
  background-color: #f8f9fa;
  min-height: 100vh;
}

.org-chart {
  display: flex;
  flex-direction: column;
  gap: 3rem;
  align-items: center;
}

.level {
  display: flex;
  gap: 2rem;
  flex-wrap: wrap;
  justify-content: center;
}

.org-card {
  background: white;
  border-radius: 12px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
  padding: 1rem;
  min-width: 280px;
  border: 1px solid #e9ecef;
  transition: all 0.2s ease;
}

.org-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
}

.main-card {
  border: 2px solid #6610f2;
}

.card-content {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.profile {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.avatar {
  width: 48px;
  height: 48px;
  border-radius: 50%;
  background-color: #f8f9fa;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1.25rem;
}

.info h6 {
  margin: 0;
  font-size: 0.9rem;
  line-height: 1.2;
}

.tags {
  display: flex;
  gap: 0.5rem;
  flex-wrap: wrap;
}

.tag {
  padding: 0.25rem 0.75rem;
  border-radius: 1rem;
  font-size: 0.75rem;
  font-weight: 500;
}

.tag.manager {
  background-color: #e3f2fd;
  color: #1976d2;
}

.tag.department {
  background-color: #f3e5f5;
  color: #9c27b0;
}

.tag.sub-department {
  background-color: #e8f5e9;
  color: #2e7d32;
}

.tag.count {
  background-color: #eeeeee;
  color: #616161;
}

/* Responsive adjustments */
@media (max-width: 768px) {
  .chart-container {
    padding: 1rem;
  }

  .org-card {
    min-width: 100%;
  }

  .controls {
    flex-direction: column;
    gap: 1rem;
  }
}
</style>
