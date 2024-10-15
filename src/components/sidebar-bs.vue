<template>
  <div class="bg-white">
    <sidebarContent :open="isOpen" class="d-none d-xl-block" />

    <Teleport to="body">
      <transition name="slide">
        <div
          v-if="isOpen"
          id="mobile-sidebar"
          class="position-fixed top-0 start-0 h-100 bg-white d-xl-none"
          style="width: 220px; z-index: 1040"
        >
          <sidebarContent :open="isOpen" class="d-block d-xl-none" />
        </div>
      </transition>

      <transition name="fade">
        <div
          v-if="isOpen"
          class="position-fixed top-0 start-0 w-100 h-100 bg-dark d-xl-none"
          style="opacity: 0.8; z-index: 1030"
          @click="close()"
        ></div>
      </transition>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import sidebarContent from './sidebar-content.vue'

defineProps({
  isOpen: { type: Boolean, required: true },
  toggle: { type: Function, required: true },
  close: { type: Function, required: true }
})
</script>

<style lang="scss" scoped>
.slide-enter-active,
.slide-leave-active {
  transition: transform 0.3s ease;
}

.slide-enter-from,
.slide-leave-to {
  transform: translateX(-100%);
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
