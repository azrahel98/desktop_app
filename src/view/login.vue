<template>
  <section class="bg-light py-3 py-md-5">
    <div class="container">
      <div class="row justify-content-center">
        <div class="col-12 col-sm-8 col-md-7 col-lg-6 col-xl-4 col-xxl-4">
          <div class="card border border-light-subtle rounded-3 shadow-sm">
            <div class="card-body p-3 p-md-4 p-xl-5">
              <h2 class="fs-3 fw-bold text-start mb-4">Sign In</h2>
              <span class="text-secondary">Ingresa su usuario y contraseña</span>

              <form class="pt-3" @submit.prevent="login" autocomplete="off">
                <div class="row gy-2 overflow-hidden justify-content-center">
                  <div class="col-12">
                    <div class="d-flex gap-1">
                      <label for="mail" class="form-label fw-medium fs-4">Correo </label>
                      <div class="text-primary">*</div>
                    </div>
                    <input
                      type="text"
                      class="form-control"
                      :class="{ 'is-invalid': errores?.username != null }"
                      v-model="form.username"
                      id="mail"
                      placeholder="rscl"
                      required
                      autocomplete="off"
                    />
                    <div v-if="errores?.username" class="text-danger">
                      <span v-for="er in errores.username?._errors">{{ er }}</span>
                    </div>
                  </div>
                  <div class="col-12 mt-4">
                    <div class="d-flex gap-1">
                      <label for="mail" class="form-label fw-medium fs-4">Contraseña </label>
                      <div class="text-primary">*</div>
                    </div>
                    <input
                      type="password"
                      :class="{ 'is-invalid': errores?.password != null }"
                      v-model="form.password"
                      required
                      class="form-control"
                      id="password"
                      placeholder="name@example.com"
                      autocomplete="off"
                    />
                    <div v-if="errores?.password" class="text-danger">
                      <span v-for="er in errores.password?._errors">{{ er }}</span>
                    </div>
                  </div>

                  <div class="col-7">
                    <div class="d-grid my-3 align-items-center">
                      <button
                        class="btn btn-primary btn-md fs-4"
                        :disabled="loading"
                        :class="{ 'is-loading': loading }"
                        type="submit"
                      >
                        Log in
                      </button>
                    </div>
                  </div>
                </div>
              </form>
            </div>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>
<script lang="ts" setup>
import { reactive, ref } from 'vue'
import * as z from 'zod'
import { router } from '../router'
import { invoke } from '@tauri-apps/api/core'

const form = reactive({
  username: '',
  password: ''
})

const loading = ref(false)

const formSchema = z.object({
  username: z.string().min(1, { message: 'No debe ser vacio' }),
  password: z.string().min(1, { message: 'No debe ser vacio' })
})

type formSchemaType = z.infer<typeof formSchema>
const errores = ref<z.ZodFormattedError<formSchemaType> | null>(null)

const login = async () => {
  loading.value = true
  try {
    const valid = formSchema.safeParse(form)
    errores.value = null

    if (!valid.success) {
      errores.value = valid.error.format()
    } else {
      const data: any = await invoke('login', { username: form.username, password: form.password })
      localStorage.setItem('token', data)
      await router.push({ name: 'dashboard' })
    }
  } catch (error) {
    console.log(error)
    var er = error as any
    if (er.code == 1) {
      errores.value = {
        _errors: [],
        username: {
          _errors: [er.message]
        }
      }
    } else if (er.code == 2) {
      errores.value = {
        _errors: [],
        password: {
          _errors: [er.message]
        }
      }
    }
  }
  loading.value = false
}
</script>

<style lang="scss" scoped>
.is-loading {
  cursor: not-allowed;
}

.is-invalid {
  border: 1px solid red;
  transition: border-color 1.2s linear;
}
section {
  height: 100vh;
}
</style>
