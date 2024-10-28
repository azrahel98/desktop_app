<template>
  <div class="home">
    <div class="text-start pb-5">
      <div class="page-pretitle fw-medium">Dashboard</div>
      <h2 class="page-title">Calendario / Inicio</h2>
    </div>
    <div></div>

    <div class="calendario card">
      <div class="header">
        <div class="botones">
          <button
            class="btn btn-sm p-2 btn-ghost-facebook"
            @click="() => (ahora = subMonths(ahora, 1))"
          >
            <IconArrowLeft size="17" />
          </button>
          <button class="btn btn-sm p-2 btn-outline" @click="() => (ahora = new Date())">
            Hoy
          </button>
          <button
            class="btn btn-sm p-2 btn-ghost-facebook"
            @click="() => (ahora = addMonths(ahora, 1))"
          >
            <IconArrowRight size="17" />
          </button>
        </div>
        <span class="page-title fs-1">{{ meses[ahora.getMonth()] }} {{ ahora.getFullYear() }}</span>
      </div>
      <div class="semana text-secondary">
        <span>Lun</span>
        <span>Mar</span>
        <span>Mie</span>
        <span>Jue</span>
        <span>Vie</span>
        <span>Sab</span>
        <span>Dom</span>
      </div>
      <div class="cuerpo">
        <div v-for="_x in semana()" class="card bg-white" />
        <div class="card dia fs-4 fw-bold hoydia" v-for="x in getDaysInMonth(ahora)">
          <h4 class="m-0 p-0 py-1">{{ x }}</h4>
          <div class="d-flex flex-wrap gap-1 justify-content-center">
            <span
              class="badge badges-list text-bg-facebook"
              v-for="d in notion_data!.filter((e) => e.fecha == x)"
              >{{ d.descripcion }}</span
            >
            <RouterLink
              class="w-100"
              :to="{ name: 'perfil', params: { dni: d.dni } }"
              v-for="d in cumples.filter((e: any) => e.dia == x)"
            >
              <span class="badge text-bg-flickr fs-6">{{ d.nombres }}</span>
            </RouterLink>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { IconArrowLeft, IconArrowRight } from '@tabler/icons-vue'
import { invoke } from '@tauri-apps/api/core'
import { addMonths, getDay, getDaysInMonth, subMonths, getYear, getMonth, isEqual } from 'date-fns'
import { onMounted, ref, watch } from 'vue'
const meses = [
  'Enero',
  'Febrero',
  'Marzo',
  'Abril',
  'Mayo',
  'Junio',
  'Julio',
  'Agosto',
  'Septiembre',
  'Octubre',
  'Noviembre',
  'Diciembre'
]
const ahora = ref<Date>(new Date())
const notion_data = ref<Array<any>>([])

const cumples = ref<Array<any>>([])

onMounted(async () => {
  // await consulta()
})

watch(ahora, async (_x, _y) => {
  await consulta()
})

const consulta = async () => {
  try {
    notion_data.value = []
    cumples.value = []
    const res: any = await invoke('fetch_notion', {
      año: getYear(ahora.value).toString(),
      mes: (getMonth(ahora.value) + 1).toString().padStart(2, '0'),
      dia: getDaysInMonth(ahora.value).toString()
    })

    JSON.parse(res).results.forEach(
      (e: {
        id: string
        properties: {
          Descripcion: { title: { text: { content: any } }[] }
          Fecha: { date: { start: any } }
        }
      }) => {
        notion_data.value?.push({
          descripcion: e.properties.Descripcion.title[0].text.content,
          fecha: parseInt(e.properties.Fecha.date.start.toString().split('-')[2]),
          id: e.id,
          database: false
        })
      }
    )
    cumples.value = await invoke('cumpleaños_lista', { mes: ahora.value.getMonth() + 1 })
  } catch (error) {
    console.log(error)
  }
}

const semana = () => {
  let dia = getDay(new Date(ahora.value.getFullYear(), ahora.value.getMonth(), 1)) - 1
  if (dia < 0) return 6
  return dia
}
</script>

<style lang="scss" scoped>
.home {
  height: 100vh;
  display: grid;
  grid-template-rows: min-content min-content auto;
  padding-top: 4vh;
  gap: 2vh;

  .header {
    display: grid;
    grid-template-columns: min-content 1fr;
    padding: 2vh 4vw 2vh 4vw;
    flex-wrap: wrap;
    width: 100%;
    justify-items: center;
    border-bottom: 1px solid #ddd;

    .botones {
      display: flex;
      justify-content: space-around;
      gap: 2vw;
    }
  }

  .calendario {
    display: grid;
    grid-template-rows: min-content min-content auto;
    padding: 0.4vh 1vw 1vh 1vw;
    align-items: start;
    height: 100%;
    overflow: hidden;
    .semana,
    .cuerpo {
      display: grid;
      row-gap: 0.25vh;
      align-items: start;
      align-content: start;
      vertical-align: middle;
      column-gap: 0.2vw;
      text-align: center;
      grid-template-columns: repeat(7, 1fr);
    }

    .semana {
      padding-top: 1.3vh;
      height: min-content;
    }

    .cuerpo {
      height: 100%;
      grid-auto-rows: 1fr;
      overflow: auto;

      .card {
        display: flex;
        flex-direction: column;
        height: 100%;
        padding: 0.5em;
        border: 1px solid #ddd;
        border-radius: 10px;
        background-color: #f9f9f9;
        span {
          height: min-content;
          min-height: 2vh;
          white-space: wrap;
          width: 100%;
        }
      }
    }
  }
}
</style>
