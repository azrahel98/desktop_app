<template>
  <div class="mains">
    <div class="info fw-medium">Buscar Empleados</div>
    <div class="search">
      <input
        type="text"
        v-model="busqueda"
        @keyup.enter="realizarBusqueda"
        class="form-control rounded-5 text-center fs-6 w-100"
      />
    </div>
    <div class="resultados">
      <div v-for="x in trabajadores" class="card rounded-4">
        <div class="rounded-0 p-0 m-0">
          <img :src="`${x.imagen}`" class="border-1 border-secondary" v-if="x.imagen != null" />
          <img src="../../assets/mujer.svg" class="border-1 border-secondary" v-else-if="!x.sexo" />
          <img src="../../assets/mann.svg" class="border-1 border-secondary" v-else />
        </div>

        <div class="user">
          <span class="nombre p-0 m-0">{{ x.nombre }}</span>
          <span class="text-secondary">@{{ x.dni }}</span>
        </div>
        <div class="rounded-bottom-4 text-center">
          <button
            class="btn btn-sm border-0"
            :class="[x.activo ? 'btn-outline-primary' : 'btn-outline-danger']"
          >
            <RouterLink :to="{ name: 'perfil', params: { dni: x.dni.toString() } }">
              <IconDirectionSign class="w-100" size="30" />
            </RouterLink>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { IconDirectionSign } from '@tabler/icons-vue'
import { ref } from 'vue'

interface Itrabajador {
  nombre: String
  puesto: String
  cargos: Number
  dni: String
  activo: Boolean
  sexo: Boolean
  imagen: String | null
}

const trabajadores = ref<Array<Itrabajador>>([])

const busqueda = ref('')

const realizarBusqueda = async () => {
  try {
    trabajadores.value = []
    if (busqueda.value.length > 3) {
      const res: any = await invoke('buscar_trabajadores', { nombre: busqueda.value })
      console.log(res)
      res.forEach((e: any) => {
        trabajadores.value.push({
          activo: e.activo == 'Y' ? true : false,
          nombre: e.nombres,
          puesto: e.cargo,
          cargos: e.cargos,
          imagen: e.imagen,
          dni: e.dni,
          sexo: e.sexo == 'M' ? true : false
        })
      })
    }
  } catch (error) {
    trabajadores.value = []
    console.log(error)
  }
}
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
    font-size: 0.74;
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
