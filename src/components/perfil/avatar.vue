<template>
  <div class="profile-header text-dark">
    <div class="header">
      <span class="avatar rounded">
        <img src="../../assets/mujer.svg" height="2px" class="border-1 border-secondary" />
      </span>
      <div class="card">
        <div class="card-body">
          <div class="mb-2">
            <IconBook size="24" class="icon text-secondary" />
            Estudio en: <strong>University of Ljubljana</strong>
          </div>
          <div class="mb-2">
            <IconBriefcase class="icon text-secondary" />
            Worked at: <strong>Devpulse</strong>
          </div>
          <div class="mb-2">
            <IconHome class="icon text-secondary" />

            Lives in: <strong>Šentilj v Slov. Goricah, Slovenia</strong>
          </div>
          <div class="mb-2">
            <IconMapPin class="icon text-secondary" />
            From: <strong><span class="flag flag-country-si"></span> Slovenia</strong>
          </div>
          <div class="mb-2">
            <IconCalendar class="icon text-secondary" />
            Birth date: <strong>13/01/1985</strong>
          </div>
          <div>
            <IconClock class="icon text-secondary" />
            Time zone: <strong>Europe/Ljubljana</strong>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import ModalPerfil from './modal.vue'
import {
  IconBook,
  IconBriefcase,
  IconHome,
  IconMapPin,
  IconCalendar,
  IconClock
} from '@tabler/icons-vue'

defineProps({
  perfil: { type: Object, required: true }
})
</script>

<style lang="scss" scoped>
.header {
  display: grid;
  grid-template-columns: 1fr 1fr;
  justify-content: center;
  align-items: center;
  justify-items: center;
}
</style>
