<template>
  <div class="tree-node">
    <div class="node-content" :class="{ 'has-children': node.children }">
      <button v-if="node.children" class="toggle-btn" @click="toggleExpand">
        {{ isExpanded ? '▼' : '►' }}
      </button>
      <div class="node-info">
        <span class="node-id">{{ node.id }}</span>
        <span class="node-name">{{ node.name }}</span>
      </div>
    </div>
    <div v-if="isExpanded && node.children" class="node-children">
      <organo v-for="child in node.children" :key="child.id" :node="child" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

interface TreeNodeType {
  id: string
  name: string
  children?: TreeNodeType[]
}

const props = defineProps<{
  node: TreeNodeType
}>()

const isExpanded = ref(true)

const toggleExpand = () => {
  isExpanded.value = !isExpanded.value
}
</script>

<style scoped>
.tree-node {
  margin-bottom: 0.5rem;
}

.node-content {
  display: flex;
  align-items: center;
  background-color: #f8f9fa;
  border-radius: 0.25rem;
  padding: 0.5rem;
  transition: background-color 0.3s ease;
}

.node-content:hover {
  background-color: #e9ecef;
}

.node-content.has-children {
  background-color: #e9ecef;
  font-weight: bold;
}

.toggle-btn {
  background: none;
  border: none;
  cursor: pointer;
  font-size: 0.8rem;
  padding: 0 0.5rem;
}

.node-info {
  display: flex;
  flex-direction: column;
}

.node-id {
  font-size: 0.8rem;
  color: #6c757d;
}

.node-name {
  font-size: 1rem;
}

.node-children {
  margin-left: 1.5rem;
  border-left: 1px solid #dee2e6;
  padding-left: 0.5rem;
}
</style>
