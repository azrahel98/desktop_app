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
        <div class="modal-header border-0 m-0">
          <h5 class="modal-title fw-bold" id="agregarestante">Seleccionar Estante</h5>
          <button
            type="button"
            class="btn-close"
            data-bs-dismiss="modal"
            aria-label="Close"
          ></button>
        </div>
        <div class="modal-body p-0 m-0">
          <form @submit.prevent="guardar(armario.id)" class="needs-validation d-grid" novalidate>
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

            <Estante
              :archivadores="archivadores"
              :filas="armario.filas"
              :seleccionbale="true"
              @seleccionado="recibir"
              :columnas="armario.columnas"
              v-if="armario != null"
            />
            <div v-else />
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

<style lang="scss" scoped>
.d-grid {
  grid-template-rows: min-content auto min-content;
  height: 50vh !important;
}
</style>

<script lang="ts" setup>
import { IconPlus } from '@tabler/icons-vue'
import { invoke } from '@tauri-apps/api/core'
import { reactive, ref } from 'vue'
import { router } from '../../../router'
import Estante from './estante.vue'
import { closemodal } from '../../../tools/modal'

const estantes = ref<Array<any>>([])
const emit = defineEmits(['change'])
const estanteadd = ref<HTMLElement | null>(null)

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

const recibir = (e: any) => {
  ubicacion.fila = e.fila
  ubicacion.columna = e.col
  agregarArchivador()
}

const agregarArchivador = () => {
  archivadores.clear()
  const posicion = `${ubicacion.fila},${ubicacion.columna}`
  archivadores.add(posicion)
}

const guardar = async (x: number) => {
  try {
    await invoke('agregar_ubicacion', {
      dni: router.currentRoute.value.params.dni.toString(),
      fila: ubicacion.fila,
      columna: ubicacion.columna,
      estante: x
    })
    closemodal(estanteadd.value)
    emit('change')
  } catch (error) {
    console.log(error)
  }
}
</script>
