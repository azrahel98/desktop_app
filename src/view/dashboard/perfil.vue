<template>
  <div class="container pb-0 mb-0">
    <Avatar :perfil="perfil" />
    <div class="pagina">
      <div class="lista">
        <Card v-for="job in vinculos" :job="job" />
        <div class="card legajo">
          <h2 class="title fs-5 mb-0 text-center py-3">Historial</h2>
          <button
            v-if="historial.filter((x) => x.devuelto == null).length == 0"
            class="btn btn-sm btn-outline-info mb-2 text-center"
            style="width: min-content"
            data-bs-toggle="modal"
            data-bs-target="#legajoAdd"
          >
            <IconPlus size="18" />
          </button>
          <Legajo_modal
            :perfil="perfil"
            @change="
              async () =>
                (historial = await invoke('buscar_prestamos', {
                  dni: router.currentRoute.value.params.dni.toString()
                }))
            "
          />

          <div class="detalle border-bottom pt-3" v-for="x in historial" :key="x.id">
            <div>
              {{ x.usuario }}
            </div>
            <div class="d-flex flex-column border-start border-2 gap-2">
              <span class="border-top px-1">{{ x.prestamo }}</span>
              <span class="border-top py-1 text-center"
                ><IconOutbound class="text-info" size="15" />{{ x.fechaprestamo }}</span
              >
              <span class="border-top py-1 text-center" v-if="x.devuelto != null"
                ><IconArrowSharpTurnLeft class="text-warning" size="15" />{{ x.devuelto }}</span
              >
              <button
                v-else
                class="btn btn-sm btn-outline-warning p-0 m-0 my-2 mx-2"
                data-bs-toggle="modal"
                :data-bs-target="`#devuelve-${x.id}`"
              >
                Devuelto
              </button>
              <Devuelto
                @change="
                  async () =>
                    (historial = await invoke('buscar_prestamos', {
                      dni: router.currentRoute.value.params.dni.toString()
                    }))
                "
                :id="x.id"
                :prestamo="x.prestamo"
                :fechaprestamo="x.fechaprestamo"
              />
            </div>
          </div>
        </div>
      </div>
      <div></div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { onMounted, ref, watch } from 'vue'
import { router } from '../../router'
import { IconPlus, IconOutbound, IconArrowSharpTurnLeft } from '@tabler/icons-vue'
import Card from '@com/perfil/card.vue'
import { RouteLocationNormalizedLoadedGeneric } from 'vue-router'
import Avatar from '@com/perfil/avatar.vue'
import { invoke } from '@tauri-apps/api/core'
import Legajo_modal from '@com/perfil/legajo_modal.vue'
import Devuelto from '@com/perfil/devuelto.vue'

const perfil = ref<any>({})
const vinculos = ref<Array<any>>([])
const historial = ref<Array<any>>([])

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
  } catch (error) {
    console.log(error)
  }
}
</script>
<style lang="scss" scoped>
.container {
  height: 100vh;
  display: grid;
  grid-template-rows: min-content auto;
  justify-items: center;
  row-gap: 2vh;
  width: 100%;
  padding: 0;
  padding-top: 7rcap;

  .pagina {
    display: grid;
    grid-template-columns: 1fr;
    grid-template-rows: min-content;
    padding: 0;
    margin: 0;
    width: 100%;
    overflow: hidden;
    overflow-y: auto;
    height: 100%;
    box-sizing: border-box;

    .lista {
      width: 100%;
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(250px, max-content));
      justify-content: center;
      justify-items: center;
      row-gap: 5px;
      column-gap: 10px;
      overflow-y: auto;
      .legajo {
        height: 100%;
        overflow-y: auto;
        max-height: 35vh;
        column-span: 1/2;
        width: 100%;
        grid-column: span 2;
        display: flex;
        align-content: center;
        justify-items: center;
        align-items: center;
        .card-title {
          font-size: 1rem;
          font-weight: 500;
        }
        .detalle {
          display: flex;
          gap: 3px;
          justify-content: center;
          align-items: center;
          font-size: 0.87rem;
          font-weight: bold;
          .d-flex {
            font-size: 1.1rcap;
          }
        }
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
