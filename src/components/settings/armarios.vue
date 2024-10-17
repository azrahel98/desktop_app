<template>
  <button
    class="btn btn-outline-primary p-0 m-0 px-3"
    data-bs-toggle="modal"
    data-bs-target="#agregarestante"
  >
    <IconEdit size="18" />
  </button>
  <div
    class="modal modal-md fade"
    id="agregarestante"
    tabindex="-1"
    aria-labelledby="agregarestante"
    aria-hidden="true"
    ref="agregarestante"
  >
    <div class="modal-dialog modal-dialog-scrollable">
      <div class="modal-content px-4 py-1 shadow-sm rounded-lg">
        <div class="modal-header border-0 mb-2">
          <h5 class="modal-title fw-bold" id="agregarestante">Agregar Estante</h5>
          <button
            type="button"
            class="btn-close"
            data-bs-dismiss="modal"
            aria-label="Close"
          ></button>
        </div>
        <div class="modal-body">
          <form @submit.prevent="registrar()" class="needs-validation" novalidate>
            <div class="mb-1">
              <label for="name" class="form-label fw-semibold">Nombre</label>
              <input id="name" type="text" class="form-control" v-model="nombre" />
            </div>
            <label for="name" class="form-label fw-semibold">Columnas y Filas</label>
            <div class="input-group mb-3">
              <input
                type="number"
                id="columnas"
                v-model.number="columnas"
                class="form-control"
                min="1"
                max="10"
              />
              <span class="input-group-text">@</span>
              <input
                type="number"
                id="filas"
                v-model.number="filas"
                class="form-control"
                min="1"
                max="10"
              />
            </div>
            <div class="armario-container">
              <div class="armario">
                <div
                  v-for="fila in filas"
                  :key="'fila-' + fila"
                  class="fila"
                  :style="{ height: `calc(100% / ${filas})` }"
                >
                  <div
                    v-for="columna in columnas"
                    :key="'columna-' + columna"
                    class="columna"
                    :style="{ width: `calc(100% / ${columnas})` }"
                  >
                    <div class="estante"></div>
                    <div class="archivador-anillas"></div>
                  </div>
                </div>
              </div>
            </div>

            <div class="d-flex justify-content-end mt-4">
              <button type="button" class="btn btn-secondary me-2" data-bs-dismiss="modal">
                Cancelar
              </button>
              <button type="submit" class="btn btn-primary text-white">Agregar Estante</button>
            </div>
          </form>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { IconEdit } from '@tabler/icons-vue'
import { invoke } from '@tauri-apps/api/core'
import { ref } from 'vue'

const nombre = ref('')
const filas = ref(3)
const columnas = ref(1)

const registrar = async () => {
  try {
    await invoke('ingresar_estante', {
      table: { id: 0, nombre: nombre.value, filas: filas.value, columnas: columnas.value }
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
  border: 10px solid #a0a0a0;
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
