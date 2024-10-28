<template>
  <aside class="sidebar-container" :class="{ 'sidebar-collapsed': !open }">
    <div class="sidebar-content">
      <!-- Logo -->
      <div class="sidebar-logo">
        <img src="../assets/logo.png" alt="Logo" class="logo-img" />
      </div>

      <!-- Navigation Links -->
      <nav class="sidebar-nav">
        <router-link
          v-for="item in menuItems"
          :key="item.name"
          :to="{ name: item.route }"
          class="nav-link"
          :class="{
            active: isActiveRoute(item.route),
            'nav-link-collapsed': !open
          }"
        >
          <component :is="item.icon" :size="22" class="nav-icon text-secondary" />
          <span class="nav-text" v-show="open">{{ item.label }}</span>
        </router-link>

        <!-- Perfil Link (Conditional) -->
        <router-link
          v-if="router.currentRoute.value.name === 'perfil'"
          :to="{
            name: 'perfil',
            params: { dni: router.currentRoute.value.params.dni }
          }"
          class="nav-link"
          :class="{ active: isActiveRoute('perfil') }"
        >
          <IconListDetails :size="22" class="nav-icon" />
          <span class="nav-text text-primary" v-show="open">
            {{ router.currentRoute.value.name }}
          </span>
        </router-link>
      </nav>
    </div>

    <!-- User Profile - Now fixed at bottom -->
    <div class="sidebar-footer" :class="{ 'footer-collapsed': !open }">
      <div class="avatar">
        <img src="../assets/logo.png" alt="User avatar" class="avatar-img" />
      </div>
      <div class="user-info" v-if="open">
        <p class="user-name">{{ user.nombre }}</p>
        <p class="user-role">Administrador</p>
      </div>
    </div>
  </aside>
</template>

<script setup lang="ts">
import { IconHomeFilled, IconListDetails, IconSearch, IconSettings } from '@tabler/icons-vue'
import { router } from '../router'
import { userStore } from '@store/user'

defineProps({
  open: { type: Boolean, required: true }
})

const user = userStore()

// Menu items configuration
const menuItems = [
  {
    name: 'dashboard',
    route: 'dashboard',
    label: 'Dashboard',
    icon: IconHomeFilled
  },
  {
    name: 'settings',
    route: 'settings',
    label: 'Configuracion',
    icon: IconSettings
  },
  {
    name: 'buscar',
    route: 'buscar',
    label: 'Buscar',
    icon: IconSearch
  }
]

// Active route checker
const isActiveRoute = (routeName: string): boolean => {
  return router.currentRoute.value.name === routeName
}
</script>

<style lang="scss" scoped>
.sidebar-container {
  position: sticky;
  top: 0;
  height: 100vh;
  width: max-content;
  display: flex;
  flex-direction: column;
  background-color: white;
  box-shadow: 0 0 20px rgba(0, 0, 0, 0.05);
  transition: all 0.3s ease;
  overflow: hidden; // Changed from overflow-x to prevent scrolling

  &.sidebar-collapsed {
    width: 5vw;
  }
}

.sidebar-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow-y: auto;
  /* Prevent content from going under footer */
  padding-bottom: 80px; // Height of footer
}

.sidebar-logo {
  padding: 1.5rem;
  display: flex;
  justify-content: center;
  align-items: center;
  border-bottom: 1px solid #f0f0f0;
  background-color: #ffffff;

  .logo-img {
    width: 40px;
    height: auto;
    transition: all 0.3s ease;
  }
}

.sidebar-nav {
  padding: 1rem 0;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.nav-link {
  display: flex;
  align-items: center;
  padding: 0.75rem 1.5rem;
  color: #64748b;
  text-decoration: none;
  gap: 1rem;
  transition: all 0.2s ease;
  border-radius: 8px;
  margin: 0 0.5rem;

  &:hover {
    background-color: #f8fafc;
    color: #2563eb;
  }

  &.active {
    background-color: #eff6ff;
    color: #2563eb;
  }

  &.nav-link-collapsed {
    padding: 0.75rem;
    justify-content: center;
  }
}

.nav-icon {
  min-width: 22px;
}

.nav-text {
  font-size: 1.12rcap;
  font-weight: 600;
  white-space: nowrap;
}

.sidebar-footer {
  position: absolute; // Changed to absolute
  bottom: 0;
  left: 0;
  right: 0;
  padding: 1rem 1.5rem;
  display: flex;
  align-items: center;
  gap: 1rem;
  border-top: 1px solid #f0f0f0;
  background-color: #f8fafc;
  height: 80px; // Fixed height
  box-sizing: border-box;

  &.footer-collapsed {
    padding: 1rem;
    justify-content: center;
  }
}

.avatar {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  overflow: hidden;
  flex-shrink: 0;

  .avatar-img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
}

.user-info {
  overflow: hidden;

  .user-name {
    font-size: 0.875rem;
    font-weight: 600;
    color: #1e293b;
    margin: 0;
    line-height: 1.2;
  }

  .user-role {
    font-size: 0.75rem;
    color: #64748b;
    margin: 0;
    line-height: 1.2;
  }
}

@media (max-width: 1200px) {
  .sidebar-container {
    position: fixed;
    z-index: 1000;
    transform: translateX(0);

    &.sidebar-collapsed {
      transform: translateX(-100%);
    }
  }
}
</style>
