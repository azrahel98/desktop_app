<template>
  <div class="armario-container">
    <div class="armario">
      <div
        v-for="fila in filas"
        :key="'fila-' + fila"
        class="fila"
        :style="{ height: `calc(100% / ${filas})` }"
      >
        <div
          v-for="columna in columnas"
          :key="'columna-' + columna"
          class="columna"
          @click="click(fila, columna, seleccionbale)"
          :style="{ width: `calc(100% / ${columnas})` }"
        >
          <div class="estante"></div>
          <div v-if="tieneArchivador(fila, columna, archivadores)" class="archivador" />
          <div class="archivador-anillas"></div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Reactive } from 'vue'

defineProps({
  filas: { type: Number, default: 2 },
  columnas: { type: Number, default: 3 },
  archivadores: { type: Set, required: true },
  seleccionbale: { type: Boolean, required: false, default: false }
})
const emit = defineEmits(['seleccionado'])

const click = (fila: number, col: number, seleccion: boolean) => {
  if (seleccion) {
    emit('seleccionado', { fila, col })
  }
}

const tieneArchivador = (fila: any, columna: any, archivador: Reactive<Set<unknown>>) => {
  return archivador.has(`${fila},${columna}`)
}
</script>

<style lang="scss" scoped>
$armario-bg: linear-gradient(145deg, #e6e6e6, #ffffff);
$archivador-color: #3a3a3a;
$etiqueta-color: #ffffff;
$anillas-color: #555555;

@mixin flex-column {
  display: flex;
  flex-direction: column;
}

@mixin absolute-fill {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
}

.armario-container {
  position: relative;
  height: 100%;
  padding-left: 2vh;
  padding-right: 2vh;
  border-radius: 10px;
}

.armario {
  background: $armario-bg;
  height: 100%;
  @include flex-column;
  padding: 10px;

  border-radius: 8px;
}

.fila {
  display: flex;
  flex: 1;
}

.columna {
  flex: 1;
  border-right: 1px solid #c0c0c0;
  border-bottom: 1px solid #c0c0c0;
  position: relative;
  box-sizing: border-box;
  transition: all 0.3s ease;

  &:hover {
    background-color: rgba(255, 255, 255, 0.5);
  }

  &:last-child {
    border-right: none;
  }

  .fila:last-child & {
    border-bottom: none;
  }
}

.estante {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  height: 8px;
  background: linear-gradient(to bottom, #f0f0f0, #d9d9d9);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.archivador {
  @include absolute-fill;
  top: 10px;
  left: 10px;
  right: 10px;
  bottom: 10px;
  background-color: $archivador-color;
  border-radius: 4px;
  box-shadow:
    0 4px 6px rgba(0, 0, 0, 0.1),
    inset 0 1px 0 rgba(255, 255, 255, 0.1);
  @include flex-column;
  justify-content: space-between;
  padding: 5px;
  transition: all 0.3s ease;

  &:hover {
    transform: translateY(-2px);
    box-shadow:
      0 6px 12px rgba(0, 0, 0, 0.15),
      inset 0 1px 0 rgba(255, 255, 255, 0.1);
  }
}

.archivador-etiqueta {
  position: absolute;
  left: 15%;
  top: 10px;
  right: 10px;
  height: 20px;
  background-color: $etiqueta-color;
  border-radius: 2px;

  &::after {
    content: 'ARTESCO';
    position: absolute;
    left: 10px;
    top: 50%;
    transform: translateY(-50%);
    color: $archivador-color;
    font-size: 12px;
    font-weight: bold;
  }
}

.archivador-anillas {
  position: absolute;
  left: 10px;
  top: 15%;
  bottom: 15%;
  width: 8px;
  background-color: $anillas-color;
  border-radius: 4px;
  @include flex-column;
  justify-content: space-around;

  &::before,
  &::after,
  &::before {
    content: '';
    height: 12px;
    width: 12px;
    //background-color: lighten($anillas-color, 20%);
    border-radius: 50%;
    position: absolute;
    left: -2px;
  }

  &::before {
    top: -6px;
  }
  &::after {
    bottom: -6px;
  }
  &::before {
    top: calc(50% - 6px);
  }
}

@media (max-width: 768px) {
  .armario {
    height: 300px;
  }

  .archivador-etiqueta::after {
    font-size: 10px;
  }
}
</style>
