<template>
  <div class="mains pt-3">
    <div class="col">
      <div class="page-pretitle fw-medium">Overview</div>
      <h2 class="page-title">Configuracion</h2>
    </div>
    <div class="container">
      <div class="row row-cards">
        <div class="col-md-6">
          <div class="card">
            <div class="card-header p-2 d-flex justify-content-between">
              <h4 class="card-title">Estantes</h4>
              <Armarios />
            </div>
            <div class="card-body row">
              <div class="card card-sm col-md-12" v-for="x in estantes">
                <div class="card-body">
                  <div class="row align-items-center gap-2">
                    <div class="col-auto">
                      <span class="bg-primary text-white avatar avatar-sm">
                        <IconLayoutList class="icon me-0 icon-sm" />
                      </span>
                    </div>
                    <div class="col">
                      <div class="font-weight-medium">{{ x.nombre }}</div>
                      <div class="text-secondary text-break">
                        {{ x.legajos }} legajos archivados
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import Armarios from '@com/settings/armarios.vue'
import { IconLayoutList } from '@tabler/icons-vue'
import { invoke } from '@tauri-apps/api/core'
import { onMounted, ref } from 'vue'

const estantes = ref<Array<any>>([])

const busquedas = async () => {
  try {
    const res: any = await invoke('listar_estantes')
    estantes.value = res
    console.log(res)
  } catch (error) {
    console.log(error)
  }
}

onMounted(async () => {
  await busquedas()
})
</script>

<style lang="scss" scoped>
.mains {
  display: grid;
  grid-template-rows: min-content min-content 1fr;
  row-gap: 4vh;
  grid-template-columns: 1fr;
  width: 100%;
  height: 100%;
  overflow-y: auto;
  padding-bottom: 1vh;
}
</style>
