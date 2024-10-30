<template>
  <div class="org-chart-node">
    <h5 @click="toggleCollapse(data)">{{ data.name }}</h5>
    <div v-if="data.expanded" class="collapsible">
      <organo v-for="(sub, index) in data.subareas" :key="index" :data="sub" />
    </div>
  </div>
</template>

<script>
export default {
  props: ['data'],
  methods: {
    toggleCollapse(node) {
      node.expanded = !node.expanded
    }
  }
}
</script>
