<template>
  <div
    class="flex flex-col h-full content-center items-center text-center justify-between"
    :class="{ 'bg-brand-500 rounded-lg': isnow === true }"
  >
    <time
      class="text-sm w-full font-mono"
      :class="{
        ' hoydia': isnow === true
      }"
    >
      {{ day }}
    </time>

    <div class="w-full gap-0 p-0 m-0">
      <div class="p-0 m-0 h-min">
        <span
          class="badge border-0 text-orange-600 font-medium text-xs"
          v-if="cumpleaños.length != 0"
        >
          <cake-icon class="text-orange-600" size="16" />
          {{ (cumpleaños as any)[0].nombres.length }}
        </span>
      </div>
      <div class="p-0 m-0 h-min">
        <span
          class="badge bg-brandLinear border-0 text-white text-nowrap text-xs h-full"
          v-if="ocurrencias.length != 0"
        >
          {{ ocurrencias.length }} Eventos
        </span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps({
  disable: {
    type: Boolean,
    default: false
  },
  day: {
    type: String,
    required: true
  },
  isnow: {
    type: Boolean,
    default: false
  },
  ocurrencias: { type: Array, required: true },
  cumpleaños: { type: Array, required: true }
})
</script>

<style lang="scss" scoped>
.grid {
  display: grid !important;

  .hoydia {
    font-weight: 800;
    border-radius: 15px;
    font-size: 1.1rem;
    text-align: center;
    vertical-align: middle;
    color: white;
  }
}
</style>
