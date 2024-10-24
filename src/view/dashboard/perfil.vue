<template>
  <div class="container pb-0 mb-0">
    <div class="text-start pb-5">
      <div class="page-pretitle fw-medium">Overview</div>
      <h2 class="page-title">Perfil</h2>
    </div>
    <Avatar :perfil="perfil" />
    <div class="pagina">
      <div class="lista">
        <Card v-for="job in vinculos" :job="job" />
        <div class="card w-100" style="max-height: 30vh; height: 100%">
          <h2 class="title fs-6 mb-0 text-center py-3" v-if="ubicacion != null">
            {{ ubicacion.nombre }}
          </h2>
          <h2 class="title fs-6 mb-0 text-center py-3" v-else>Agregar a Armario</h2>
          <div class="d-flex justify-content-center" v-if="ubicacion == null">
            <AddEStante @change="consulta(router.currentRoute.value.params.dni.toString())" />
          </div>
          <estante
            :filas="ubicacion.filas"
            :columnas="ubicacion.columnas"
            :archivadores="archivadores"
            v-else
          />
        </div>
      </div>
      <div />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { onMounted, reactive, ref, watch } from 'vue'
import { router } from '../../router'
import Card from '@com/perfil/card.vue'
import { RouteLocationNormalizedLoadedGeneric } from 'vue-router'
import Avatar from '@com/perfil/avatar.vue'
import { invoke } from '@tauri-apps/api/core'
import AddEStante from '../../components/perfil/legajo/estanteadd.vue'
import estante from '@com/perfil/legajo/estante.vue'

const perfil = ref<any>({})
const vinculos = ref<Array<any>>([])
const historial = ref<Array<any>>([])

const ubicacion = ref<any>(null)
const archivadores = reactive(new Set())

onMounted(async () => await consulta(router.currentRoute.value.params.dni.toString()))

watch(router.currentRoute, async (x: RouteLocationNormalizedLoadedGeneric, _y) => {
  await consulta(x.params.dni.toString())
})

const consulta = async (x: string) => {
  try {
    vinculos.value = []
    const res = await invoke('buscar_x_dni', { dni: x })
    perfil.value = res

    const vin: any = await invoke('vinculos', { dni: x })
    vinculos.value = vin

    historial.value = await invoke('buscar_prestamos', { dni: x })
    ubicacion.value = await invoke('get_ubicacion', { dni: x })
    agregarArchivador()
    console.log(vinculos.value)
  } catch (error) {
    console.log(error)
  }
}

const agregarArchivador = () => {
  archivadores.clear()
  const posicion = `${ubicacion.value.ufila},${ubicacion.value.ucolumna}`
  archivadores.add(posicion)
}
</script>
<style lang="scss" scoped>
.container {
  height: 100vh;
  margin: 0;
  display: grid;
  grid-template-rows: min-content min-content auto;
  grid-template-columns: 1fr;
  row-gap: 2vh;
  max-width: 100%;
  padding: 0;
  padding-top: 2rcap;

  .main {
    padding-left: 5vh;
    justify-self: center;
  }

  .pagina {
    justify-self: center;
    display: grid;
    grid-template-columns: 1fr;
    grid-template-rows: 1fr;
    padding: 0;
    margin: 0;
    width: 100%;
    justify-content: start;
    overflow: hidden;
    overflow-y: auto;
    height: 100%;
    box-sizing: border-box;

    .lista {
      width: 100%;
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(230px, max-content));
      justify-content: center;
      justify-items: center;
      row-gap: 5px;
      height: min-content;
      column-gap: 10px;
      overflow-y: auto;
      height: 100%;
      .card {
        width: 100%;
        max-width: 230px;

        height: min-content;
      }
    }
    .legajos {
      display: grid;
      grid-template-rows: min-content min-content auto;
      justify-items: center;
    }
  }
}

.rows,
.row {
  padding: 0;
  margin: 0;
}
</style>
