<template>
  <button
    class="btn btn-md btn-outline-info mb-2 text-center"
    style="width: min-content"
    data-bs-toggle="modal"
    data-bs-target="#estanteadd"
  >
    <IconPlus size="18" />
  </button>
  <div
    class="modal modal-md fade"
    id="estanteadd"
    tabindex="-1"
    aria-labelledby="estanteadd"
    aria-hidden="true"
    ref="estanteadd"
  >
    <div class="modal-dialog modal-dialog-scrollable">
      <div class="modal-content px-4 py-1 shadow-sm rounded-lg">
        <div class="modal-header border-0 mb-2">
          <h5 class="modal-title fw-bold" id="agregarestante">Seleccionar Estante</h5>
          <button
            type="button"
            class="btn-close"
            data-bs-dismiss="modal"
            aria-label="Close"
          ></button>
        </div>
        <div class="modal-body">
          <form @submit.prevent="guardar" class="needs-validation" novalidate>
            <div class="mb-1">
              <label for="name" class="form-label fw-semibold">Nombre</label>
              <select
                class="form-select"
                aria-label="Default select example"
                v-model="armario"
                @click="buscar_estantes"
              >
                <option v-for="x in estantes" :value="x">{{ x.nombre }}</option>
              </select>
            </div>

            <label for="name" class="form-label fw-semibold">Columnas y Filas</label>
            <div class="input-group mb-3" v-if="armario != null">
              <input
                type="number"
                id="columnas"
                v-model="ubicacion.fila"
                :max="armario.filas"
                class="form-control"
                min="1"
                @change="agregarArchivador"
              />
              <span class="input-group-text">@</span>
              <input
                type="number"
                id="filas"
                v-model="ubicacion.columna"
                class="form-control"
                :max="armario.columnas"
                @change="agregarArchivador"
                min="1"
              />
            </div>
            <div class="armario-container" v-if="armario != null">
              <div class="armario">
                <div
                  v-for="fila in armario.filas"
                  :key="'fila-' + fila"
                  class="fila"
                  :style="{ height: `calc(100% / ${armario.filas})` }"
                >
                  <div
                    v-for="columna in armario.columnas"
                    :key="'columna-' + columna"
                    class="columna"
                    :style="{ width: `calc(100% / ${armario.columnas})` }"
                  >
                    <div class="estante"></div>
                    <div v-if="tieneArchivador(fila, columna)" class="archivador" />
                    <div class="archivador-anillas"></div>
                  </div>
                </div>
              </div>
            </div>

            <div class="d-flex justify-content-end mt-4">
              <button type="button" class="btn btn-secondary me-2" data-bs-dismiss="modal">
                Cancelar
              </button>
              <button type="submit" class="btn btn-primary text-white">Guardar</button>
            </div>
          </form>
        </div>
      </div>
    </div>
  </div>
</template>
<script lang="ts" setup>
import { IconPlus } from '@tabler/icons-vue'
import { invoke } from '@tauri-apps/api/core'
import { reactive, ref } from 'vue'
import { router } from '../../../router'

const estantes = ref<Array<any>>([])

const ubicacion = reactive({
  fila: 1,
  columna: 1
})

const armario = ref<any>(null)
const archivadores = reactive(new Set())
const buscar_estantes = async () => {
  if (estantes.value.length == 0) {
    const res: any = await invoke('listar_estantes')
    estantes.value = res
  }
}

const agregarArchivador = () => {
  archivadores.clear()
  const posicion = `${ubicacion.fila},${ubicacion.columna}`
  archivadores.add(posicion)
}
const tieneArchivador = (fila: any, columna: any) => {
  return archivadores.has(`${fila},${columna}`)
}

const guardar = async () => {
  try {
    await invoke('agregar_ubicacion', {
      dni: router.currentRoute.value.params.dni.toString(),
      fila: ubicacion.fila,
      columna: ubicacion.columna,
      estante: 1
    })
  } catch (error) {
    console.log(error)
  }
}
</script>
<style scoped>
.armario-container {
  position: relative;
  padding-bottom: 40px;
}

.armario {
  background: linear-gradient(145deg, #b8b8b8, #d9d9d9);
  height: 30vh;
  display: flex;
  flex-direction: column;
  padding: 5px;
  box-shadow:
    10px 10px 20px #8a8a8a,
    -10px -10px 20px #ffffff;
}

.fila {
  display: flex;
  flex: 1;
}

.columna {
  flex: 1;
  border-right: 2px solid #909090;
  border-bottom: 2px solid #909090;
  position: relative;
  box-sizing: border-box;
}

.columna:last-child {
  border-right: none;
}

.fila:last-child .columna {
  border-bottom: none;
}

.estante {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  height: 5px;
  background: linear-gradient(to bottom, #d9d9d9, #b8b8b8);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.archivador {
  position: absolute;
  top: 5px;
  left: 5px;
  right: 5px;
  bottom: 5px;
  background-color: #000000;
  border-radius: 2px;
  box-shadow: 0 0 5px rgba(0, 0, 0, 0.3);
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  padding: 5px;
}

.archivador::before {
  content: '';
  position: absolute;
  left: 10%;
  top: 5px;
  bottom: 5px;
  width: 20px;
  background-color: #ffffff;
  border-radius: 2px;
}

.archivador::after {
  content: 'ARTESCO';
  position: absolute;
  left: 15%;
  top: 50%;
  transform: translateY(-50%) rotate(-90deg);
  color: #000000;
  font-size: 10px;
  font-weight: bold;
  white-space: nowrap;
}

.archivador-anillas {
  position: absolute;
  left: 5px;
  top: 15%;
  bottom: 15%;
  width: 5px;
  background-color: #555555;
  border-radius: 2px;
  display: flex;
  flex-direction: column;
  justify-content: space-around;
}

.archivador-anillas::before,
.archivador-anillas::after {
  content: '';
  height: 10px;
  width: 10px;
  background-color: #888888;
  border-radius: 50%;
  position: absolute;
  left: -2.5px;
}

.archivador-anillas::before {
  top: -5px;
}

.archivador-anillas::after {
  bottom: -5px;
}

@media (max-width: 768px) {
  .armario {
    height: 300px;
  }
}
</style>
