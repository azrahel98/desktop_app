import App from './App.vue'

import { createApp } from 'vue'
import { router } from './router'
import { createPinia } from 'pinia'

import '../src/assets/tabler.css'

createApp(App).use(router).use(createPinia()).mount('#app')
