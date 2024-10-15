<template>
  <div class="main-page">
    <sidebarBs :is-open="isOpen" :toggle="toggleSidebar" :close="closeSidebar" />
    <div class="pagina">
      <navBar :open="isOpen" :toggle="toggleSidebar" />
      <RouterView style="padding-right: 2vw" />
    </div>
  </div>
</template>

<script setup lang="ts">
import sidebarBs from '@com/sidebar-bs.vue'
import navBar from '@com/nav.vue'
import { ref } from 'vue'
import { RouterView } from 'vue-router'

const isOpen = ref(false)

const toggleSidebar = () => {
  isOpen.value = !isOpen.value
}

const closeSidebar = () => {
  isOpen.value = false
}
</script>

<style lang="scss" scoped>
.main-page {
  display: grid !important;
  width: 100%;
  grid-template-columns:
    min-content
    auto;
  column-gap: 2vw;
  padding: 0;
  margin: 0;

  .pagina {
    display: grid;
    height: 100vh;
  }
}
</style>
