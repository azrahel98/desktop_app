<template>
  <div class="home">
    <div class="header">
      <div class="botones">
        <button class="btn btn-sm btn-info" @click="() => (ahora = subMonths(ahora, 1))">
          <IconArrowLeft size="17" />
        </button>
        <button class="btn btn-info btn-sm" @click="() => (ahora = new Date())">Hoy</button>
        <button class="btn btn-sm btn-info" @click="() => (ahora = addMonths(ahora, 1))">
          <IconArrowRight size="17" />
        </button>
      </div>
      <span class="title fs-3">{{ meses[ahora.getMonth()] }} {{ ahora.getFullYear() }}</span>
    </div>
    <div class="calendario">
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
        <div class="card fs-4 fw-bold" v-for="x in getDaysInMonth(ahora)">
          {{ x }}
          <span class="badge text-bg-info" v-for="d in notion_data!.filter((e) => e.fecha == x)">{{
            d.descripcion
          }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { IconArrowLeft, IconArrowRight } from '@tabler/icons-vue'
import { invoke } from '@tauri-apps/api/core'
import { addMonths, getDay, getDaysInMonth, subMonths, getYear, getMonth } from 'date-fns'
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

onMounted(async () => {
  await consulta()
})

watch(ahora, async (_x, _y) => {
  await consulta()
})

const consulta = async () => {
  notion_data.value = []
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
}

const semana = () => {
  let dia = getDay(new Date(ahora.value.getFullYear(), ahora.value.getMonth(), 1)) - 1
  if (dia < 0) return 6
  return dia
}
</script>

<style lang="scss" scoped>
.home {
  display: flex;
  flex-direction: column;
  padding-top: 7vh;
  gap: 2vh;

  .header {
    display: grid;
    grid-template-columns: min-content 1fr;
    padding: 0 4vw 0 4vw;
    flex-wrap: wrap;
    width: 100%;
    text-align: center;
    align-items: center;

    .botones {
      display: flex;
      justify-content: space-around;
      gap: 2vw;
    }
  }

  .calendario {
    display: flex;
    flex-direction: column;
    height: 100%;
    background-color: white;
    padding: 1vh 1vw 1vh 1vw;
    border-radius: 20px;

    .semana,
    .cuerpo {
      display: grid;
      justify-content: center;
      align-items: center;
      row-gap: 0.25vh;
      column-gap: 0.2vw;
      text-align: center;
      grid-template-columns: repeat(7, 1fr);
    }

    .cuerpo {
      height: 100%;
      grid-template-rows: repeat(auto-fit, minmax(50px, 1fr));
      grid-auto-flow: row;

      .card {
        min-height: 50px;
        max-height: 15vh;
        height: 100%;
        display: flex;
        flex-direction: column;
        justify-content: start;
        align-items: center;
        padding: 0.5em;
        gap: 0.2vh;
        overflow: hidden;
        border: 1px solid #ddd;
        border-radius: 10px;
        background-color: #f9f9f9;
        span {
          height: max-content;
          width: 100%;
          white-space: wrap;
        }
      }

      .card:hover {
        overflow-y: auto;
      }
    }
  }
}
</style>
