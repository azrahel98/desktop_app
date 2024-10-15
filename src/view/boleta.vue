<template>
  <div class="min-h-screen bg-gradient-to-br from-gray-100 to-gray-200 p-8">
    <div class="w-full max-w-4xl mx-auto bg-white shadow-xl rounded-2xl overflow-hidden">
      <!-- Header -->
      <div class="bg-gradient-to-r from-blue-500 to-purple-600 text-white p-6">
        <div class="flex justify-between items-center">
          <div class="flex items-center space-x-4">
            <div class="h-16 w-16 rounded-full bg-white border-2 border-white overflow-hidden">
              <img
                :src="trabajador.avatarUrl"
                :alt="`${trabajador.nombre} ${trabajador.apellido}`"
                class="h-full w-full object-cover"
              />
            </div>
            <div>
              <h1 class="text-2xl font-bold">{{ trabajador.nombre }} {{ trabajador.apellido }}</h1>
              <p class="text-blue-100">{{ trabajador.cargo }}</p>
            </div>
          </div>
          <button
            @click="cerrarSesion"
            class="bg-white text-blue-600 px-4 py-2 rounded-lg flex items-center"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              class="h-4 w-4 mr-2"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"
              />
            </svg>
            Cerrar Sesión
          </button>
        </div>
      </div>

      <!-- Content -->
      <div class="p-6">
        <!-- Tabs -->
        <div class="mb-4">
          <button
            @click="activeTab = 'profile'"
            :class="[
              'px-4 py-2 rounded-lg mr-2',
              activeTab === 'profile' ? 'bg-blue-500 text-white' : 'bg-gray-200 text-gray-700'
            ]"
          >
            Perfil
          </button>
          <button
            @click="activeTab = 'payslips'"
            :class="[
              'px-4 py-2 rounded-lg',
              activeTab === 'payslips' ? 'bg-blue-500 text-white' : 'bg-gray-200 text-gray-700'
            ]"
          >
            Boletas de Pago
          </button>
        </div>

        <!-- Profile Tab -->
        <div v-if="activeTab === 'profile'" class="bg-white rounded-lg shadow p-6">
          <h2 class="text-xl font-bold mb-4">Información Personal</h2>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div v-for="(value, key) in profileInfo" :key="key" class="space-y-2">
              <p class="text-sm font-medium text-gray-500">{{ key }}</p>
              <p>{{ value }}</p>
            </div>
          </div>
        </div>

        <!-- Payslips Tab -->
        <div v-if="activeTab === 'payslips'" class="bg-white rounded-lg shadow p-6">
          <h2 class="text-xl font-bold mb-4">Boletas de Pago</h2>
          <div class="flex space-x-4 mb-6">
            <select
              v-model="mesSeleccionado"
              class="w-[180px] border border-gray-300 rounded-md p-2"
            >
              <option value="">Selecciona un mes</option>
              <option v-for="mes in meses" :key="mes.value" :value="mes.value">
                {{ mes.label }}
              </option>
            </select>
            <select
              v-model="anioSeleccionado"
              class="w-[180px] border border-gray-300 rounded-md p-2"
            >
              <option value="">Selecciona un año</option>
              <option v-for="anio in anios" :key="anio" :value="anio">
                {{ anio }}
              </option>
            </select>
          </div>
          <div class="h-[300px] overflow-y-auto pr-4">
            <div v-if="boletasFiltradas.length > 0">
              <div
                v-for="boleta in boletasFiltradas"
                :key="boleta.id"
                class="flex items-center justify-between p-4 border-b last:border-b-0 hover:bg-gray-50 transition-colors"
              >
                <div class="flex items-center space-x-4">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="h-6 w-6 text-blue-500"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                  >
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                    />
                  </svg>
                  <div>
                    <p class="font-medium">Boleta {{ formatDate(boleta.fecha) }}</p>
                    <p class="text-sm text-gray-500">Monto: ${{ boleta.monto.toFixed(2) }}</p>
                  </div>
                </div>
                <button
                  @click="descargarBoleta(boleta.urlDescarga)"
                  class="flex items-center space-x-2 px-3 py-1 border border-gray-300 rounded-md hover:bg-blue-50 hover:text-blue-600 transition-colors"
                >
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="h-4 w-4"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                  >
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"
                    />
                  </svg>
                  <span>Descargar</span>
                </button>
              </div>
            </div>
            <p v-else class="text-center text-gray-500 mt-4">
              No hay boletas disponibles para el período seleccionado.
            </p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import { makepdf } from '../tools/pdf'

const activeTab = ref('profile')
const mesSeleccionado = ref('')
const anioSeleccionado = ref('')

const trabajador = ref({
  id: '12345',
  nombre: 'Juan',
  apellido: 'Pérez',
  email: 'juan.perez@empresa.com',
  cargo: 'Desarrollador Senior',
  departamento: 'Tecnología',
  fechaIngreso: '2020-03-15',
  sueldo: 1700.0,
  afp: 'AFP Modelo',
  tipoContrato: 'Indefinido',
  avatarUrl: '/placeholder.svg?height=100&width=100'
})

const profileInfo = computed(() => ({
  Email: trabajador.value.email,
  Departamento: trabajador.value.departamento,
  'Fecha de Ingreso': new Date(trabajador.value.fechaIngreso).toLocaleDateString('es-ES'),
  'Tipo de Contrato': trabajador.value.tipoContrato,
  'Sueldo Base': `$${trabajador.value.sueldo.toFixed(2)}`,
  AFP: trabajador.value.afp
}))

const boletas = ref([
  { id: '1', fecha: '2023-05-01', monto: 1500.0, urlDescarga: '/boletas/mayo2023.pdf' },
  { id: '2', fecha: '2023-06-01', monto: 1500.0, urlDescarga: '/boletas/junio2023.pdf' },
  { id: '3', fecha: '2023-07-01', monto: 1600.0, urlDescarga: '/boletas/julio2023.pdf' },
  { id: '4', fecha: '2023-08-01', monto: 1600.0, urlDescarga: '/boletas/agosto2023.pdf' },
  { id: '5', fecha: '2023-09-01', monto: 1600.0, urlDescarga: '/boletas/septiembre2023.pdf' },
  { id: '6', fecha: '2023-10-01', monto: 1700.0, urlDescarga: '/boletas/octubre2023.pdf' },
  { id: '7', fecha: '2023-11-01', monto: 1700.0, urlDescarga: '/boletas/noviembre2023.pdf' },
  { id: '8', fecha: '2023-12-01', monto: 1700.0, urlDescarga: '/boletas/diciembre2023.pdf' }
])

const meses = [
  { value: '1', label: 'Enero' },
  { value: '2', label: 'Febrero' },
  { value: '3', label: 'Marzo' },
  { value: '4', label: 'Abril' },
  { value: '5', label: 'Mayo' },
  { value: '6', label: 'Junio' },
  { value: '7', label: 'Julio' },
  { value: '8', label: 'Agosto' },
  { value: '9', label: 'Septiembre' },
  { value: '10', label: 'Octubre' },
  { value: '11', label: 'Noviembre' },
  { value: '12', label: 'Diciembre' }
]

const anios = ['2023', '2022', '2021', '2020']

const boletasFiltradas = computed(() => {
  return boletas.value.filter((boleta) => {
    if (!mesSeleccionado.value || !anioSeleccionado.value) return true
    const fecha = new Date(boleta.fecha)
    return (
      fecha.getMonth() === parseInt(mesSeleccionado.value) - 1 &&
      fecha.getFullYear() === parseInt(anioSeleccionado.value)
    )
  })
})

const descargarBoleta = (urlDescarga) => {
  makepdf()
}

const cerrarSesion = () => {}

const formatDate = (dateString) => {
  const date = new Date(dateString)
  return date.toLocaleDateString('es-ES', { month: 'long', year: 'numeric' })
}
</script>
