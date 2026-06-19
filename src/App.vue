<template>
  <router-view />
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { useTaskStore } from '@/stores/task'
import { useConfigStore } from '@/stores/config'

const taskStore = useTaskStore()
const configStore = useConfigStore()

onMounted(async () => {
  await configStore.loadConfig()
  await taskStore.loadTasks()
})
</script>

<style scoped>
</style>