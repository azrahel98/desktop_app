import App from './App.vue'

import { createApp } from 'vue'
import { router } from './router'
import { createPinia } from 'pinia'

import Prime from 'primevue/config'
import Aura from '@primevue/themes/aura'

import '../src/assets/tabler.css'

createApp(App)
  .use(router)
  .use(createPinia())
  .use(Prime, {
    theme: {
      preset: Aura
    }
  })
  .mount('#app')
