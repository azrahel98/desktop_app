<template>
  <div class="mains">
    <div class="info fw-medium">Configuraciones</div>
    <div class="container">
      <div class="row">
        <div class="col-md-4">
          <div class="card" style="width: 18rem">
            <div class="card-body">
              <h5 class="card-title">Estantes</h5>
              <Armarios />
              <h6 class="card-subtitle mb-2 text-body-secondary">agregar estantes</h6>
              <div class="d-flex flex-column">
                <span v-for="x in estantes">{{ x.nombre }}</span>
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

  .info {
    color: #707eae;
    padding-top: 2vh;
    font-family: 'DM Sans';
    font-size: 1rem;
  }
  .search {
    justify-self: center;
    display: flex;
    width: 20vw;
    justify-content: center;
    align-items: center;
    .form-control {
      font-size: 0.84rem !important;
    }
  }
  .resultados {
    overflow-y: scroll;
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
    grid-auto-flow: row;
    column-gap: 2vh;
    row-gap: 3vh;
    justify-content: center;
    height: 100%;
    .card {
      display: grid;
      padding-top: 2vh;
      justify-content: center;
      align-items: center;
      padding-bottom: 2vh;
      grid-template-rows: min-content auto min-content;
      justify-self: center;
      justify-items: center;
      row-gap: 1vh;
      width: 100%;
      height: max-content;
      max-height: 40vh;
      max-width: 180px;
      img {
        width: 8rcap;
        justify-self: center;
        text-align: center;
        border-radius: 15px;
      }
      .avatar {
        justify-self: center;
        text-align: center;
        object-fit: cover;
        object-position: center;
      }
      .user {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-items: center;
        text-wrap: wrap;
        text-align: center;
        font-size: 1.2rcap;
        font-weight: 500;

        .nombre {
          font-size: 1.3rcap;
          font-weight: 600;
        }
      }
    }
  }
}
</style>
