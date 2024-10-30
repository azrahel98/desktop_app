import {
  NavigationGuardNext,
  RouteLocationNormalized,
  RouteRecordRaw,
  createRouter,
  createWebHistory
} from 'vue-router'

import login from './view/login.vue'
import dashboard from './view/dashboard.vue'
import { userStore } from '@store/user'
import { jwtDecode } from 'jwt-decode'
import { compareAsc } from 'date-fns'
import Home from './view/dashboard/home.vue'
import Buscar from './view/dashboard/buscar.vue'
import Settings from './view/dashboard/settings.vue'
import Perfil from './view/dashboard/perfil.vue'
import organigrama from './view/dashboard/organigrama.vue'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'home',
    component: dashboard,
    beforeEnter: (to, from, next) => middleware(to, from, next),
    children: [
      {
        path: '/',
        name: 'dashboard',
        component: Home
      },
      {
        path: '/buscar',
        name: 'buscar',
        component: Buscar
      },
      {
        path: '/settings',
        name: 'settings',
        component: Settings
      },
      {
        path: '/perfil/:dni',
        name: 'perfil',
        component: Perfil
      },
      {
        path: '/organigrama',
        name: 'organigrama',
        component: organigrama
      }
    ]
  },
  {
    path: '/login',
    name: 'login',
    beforeEnter: (to, from, next) => loginmid(to, from, next),
    component: login
  }
]

const middleware = async (
  _to: RouteLocationNormalized,
  _from: RouteLocationNormalized,
  next: NavigationGuardNext
) => {
  try {
    const token = localStorage.getItem('token')
    if (token === null) {
      return next({
        name: 'login'
      })
    }
    const store = userStore()
    const decote = jwtDecode(token) as any
    const isal = compareAsc(new Date(), new Date(decote.exp! * 1000))
    store.id = decote.id
    store.nombre = decote.nombre
    if (isal > 0) return next({ name: 'login' })
    return next()
  } catch (error) {
    return next({
      name: 'login'
    })
  }
}

const loginmid = async (
  _to: RouteLocationNormalized,
  _from: RouteLocationNormalized,
  next: NavigationGuardNext
) => {
  const token = localStorage.getItem('jwt')
  if (token != null) {
    return next({
      name: 'dashboard'
    })
  }

  return next()
}

export const router = createRouter({
  history: createWebHistory(),
  routes,
  linkExactActiveClass: 'active',
  linkActiveClass: 'currentlink'
})
