import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { AppConfig } from '@/types'

const defaultConfig: AppConfig = {
  petEnabled: true,
  petInterval: 60,
  theme: 'light',
  autoStart: false,
}

export const useConfigStore = defineStore('config', () => {
  const config = ref<AppConfig>({ ...defaultConfig })
  const loading = ref(false)

  async function loadConfig() {
    try {
      loading.value = true
      const result = await invoke<AppConfig>('get_config')
      config.value = { ...defaultConfig, ...result }
    } catch (error) {
      console.error('加载配置失败:', error)
    } finally {
      loading.value = false
    }
  }

  async function updateConfig(updates: Partial<AppConfig>) {
    try {
      const newConfig = { ...config.value, ...updates }
      await invoke('update_config', { config: newConfig })
      config.value = newConfig
    } catch (error) {
      console.error('更新配置失败:', error)
      throw error
    }
  }

  return {
    config,
    loading,
    loadConfig,
    updateConfig,
  }
})