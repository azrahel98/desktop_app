<template>
  <div class="search-input-wrappers">
    <input
      type="text"
      data-bs-toggle="dropdown"
      v-model="search"
      @keyup.prevent="buscar"
      class="form-control h-25 rounded-5"
      aria-expanded="false"
    />
    <ul class="dropdown-menu">
      <li v-for="x in resultados" @click="() => ruta(x.dni)">
        <div class="d-flex justify-content-start gap-2 px-3">
          <div class="avatar avatar-sm">
            <img src="../assets/logo.png" class="img-fluid rounded" />
          </div>
          <div class="d-flex flex-column">
            <span class="fw-semibold" :class="{ 'text-danger fw-normal ': x.activo == 'Y' }">{{
              x.nombres
            }}</span>
            <span class="fw-medium text-secondary">{{ x.dni }}</span>
          </div>
        </div>
        <hr />
      </li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { ref } from 'vue'
import { router } from '../router'

const search = ref('')

const resultados = ref<Array<any>>([])

const buscar = async () => {
  try {
    if (search.value.length > 3) {
      resultados.value = []

      const res: any = await invoke('buscar_trabajadores', { nombre: search.value })
      resultados.value = res
      console.log(res)
    }
  } catch (error) {
    console.log(error)
  }
}

const ruta = (dni: string) => {
  resultados.value = []
  search.value = ''

  router.replace({ name: 'perfil', params: { dni } })
}
</script>

<style scoped lang="scss">
.form-control {
  font-size: 0.874rem;
  font-weight: 500;
}
.dropdown-menu {
  min-width: 190px;
  max-height: 50vh;
  overflow-y: scroll !important;
  li,
  span {
    font-size: 12px;
    padding: 0 !important;
    margin: 0 !important;
  }
  hr {
    margin: 1.5vh 0 2vh 0;
  }
}
</style>
