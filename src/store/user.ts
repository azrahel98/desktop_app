import { defineStore } from 'pinia'

export const userStore = defineStore('userstore', {
  state: () => {
    return { nombre: '',id:0 }
  }
  // could also be defined as
  // state: () => ({ count: 0 })
  // actions: {
  // 	increment() {
  // 		this.count++
  // 	}
  // }
})
