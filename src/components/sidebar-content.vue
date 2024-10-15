<template>
  <div class="col-xl-3 col-xxl-2">
    <div class="sidebar">
      <div class="s-1 flex-column">
        <div class="title text-center">
          <img src="../assets/logo.png" class="img-fluid pt-4" width="60" />
        </div>
        <hr class="solid pb-4" />

        <router-link :to="{ name: 'dashboard' }" class="d-flex">
          <IconHomeFilled size="22" class="text-secondary" />
          <p class="text-secondary">Dashboard</p>
        </router-link>

        <router-link :to="{ name: 'buscar' }" class="d-flex">
          <IconSearch size="22" class="text-secondary" />
          <p class="text-secondary">Buscar</p>
        </router-link>
        <router-link
          v-if="router.currentRoute.value.name == 'perfil'"
          :to="{ name: 'perfil', params: { dni: router.currentRoute.value.params.dni } }"
          class="d-flex active"
        >
          <IconListDetails size="22" class="text-secondary" />
          <p class="text-secondary">{{ router.currentRoute.value.name }}</p>
        </router-link>
      </div>

      <div class="user">
        <div class="avatar avatar-md">
          <img src="../assets/logo.png" class="img-fluid rounded-circle" />
        </div>
        <div class="d-flex flex-column">
          <p style="font-size: 0.84rem">{{ user.nombre }}</p>
          <p>Administrador</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { IconHomeFilled, IconListDetails, IconSearch } from '@tabler/icons-vue'
import { router } from '../router'
import { userStore } from '@store/user'

const user = userStore()

defineProps({
  open: { type: Boolean, required: true }
})
</script>
<style lang="scss" scoped>
.side {
  width: 100%;
}
.sidebar {
  display: grid;
  grid-template-rows: auto 10vh;
  height: 100vh;
  position: sticky;
  top: 0;
  overflow-y: auto;
  width: 220px;
  transition: max-width 2.5 linear !important;
  @media only screen and (max-width: 1200px) {
    .side {
      display: none !important;
    }
  }

  .s-1 {
    display: flex;
    height: min-content;

    h1 {
      color: #2b3674;
      text-align: center;
      font-size: 2.3rem;
      padding-top: 4vh;
      padding-bottom: 3vh;
      font-weight: bolder;
      letter-spacing: 2px;
    }
    .d-flex {
      //top,rigth,bottom,left
      padding: 1vh 0 1vh 1.8vw;
      margin: 0;
      align-items: center;
      height: min-content;
      gap: 0.5vw;
      border-radius: 5px;
      p {
        padding: 0;
        margin: 0;
        font-weight: 400;
        font-size: 0.85rem;
        text-align: center;
        vertical-align: middle;
      }
    }
  }

  .user {
    display: flex;
    justify-content: center;
    height: min-content;
    gap: 0.5vw;

    p:first-of-type {
      color: rgb(23, 46, 138);
      font-weight: 700;
      font-size: 16px;
    }
    p:last-of-type {
      font-weight: 400;
      font-size: 0.74rem;
      color: rgb(163, 174, 208);
    }
  }
}
</style>
