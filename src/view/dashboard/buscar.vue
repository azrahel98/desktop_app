<template>
  <div class="mains">
    <div class="col">
      <div class="page-pretitle fw-medium">Overview</div>
      <h2 class="page-title">Dashboard</h2>
    </div>
    <div class="search">
      <input
        type="text"
        v-model="busqueda"
        @keyup.enter="realizarBusqueda"
        class="form-control rounded-2 text-center fs-6 w-100"
      />
    </div>
    <div class="resultados">
      <div class="card" v-for="x in trabajadores">
        <div class="card-status-top" :class="[x.activo ? 'bg-success' : 'bg-danger']"></div>
        <div class="card-body p-4 text-center">
          <span class="avatar avatar-xl mb-3 rounded">
            <img :src="`${x.imagen}`" class="border-1 border-secondary" v-if="x.imagen != null" />
            <img
              src="../../assets/mujer.svg"
              class="border-1 border-secondary"
              v-else-if="!x.sexo" />
            <img src="../../assets/mann.svg" class="border-1 border-secondary" v-else
          /></span>

          <RouterLink
            class="text-black"
            :to="{ name: 'perfil', params: { dni: x.dni.toString() } }"
          >
            <h4 class="m-0 mb-1">{{ x.nombre }}</h4>
          </RouterLink>
          <div class="text-secondary">{{ x.dni }}</div>
          <div class="mt-1">
            <span
              class="badge"
              :class="[x.activo ? 'bg-success text-white' : 'bg-danger text-white']"
              >{{ x.activo ? 'Activo' : 'Inactivo' }}</span
            >
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
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
  padding-top: 3vh;
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
    width: 30vw;
    align-items: center;

    justify-content: center;
    .form-control {
      justify-self: center;
      font-size: 0.84rem !important;
      max-width: 25vw;
    }
  }
  .resultados {
    overflow-y: scroll;
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    grid-auto-flow: row;
    column-gap: 2vh;
    row-gap: 3vh;
    justify-content: center;
    justify-items: center;
    height: 100%;
    width: 100%;
    .card {
      height: min-content;
      width: 200px;
      max-height: 40vh;
    }
  }
}
</style>
<!-- .card {
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
} -->
