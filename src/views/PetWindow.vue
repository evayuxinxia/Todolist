<template>
  <div class="pet-window">
    <div class="pet-content">
      <div class="pet-animation">
      </div>
      <div class="pet-message">
        <p>起身活动 5 分钟，多喝温水 💧</p>
      </div>
      <div class="pet-actions">
        <el-button size="small" @click="handleLater">稍后提醒</el-button>
        <el-button size="small" @click="handleClose">关闭</el-button>
        <el-button size="small" type="danger" @click="handleDisable">永久关闭</el-button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useConfigStore } from '@/stores/config'

const configStore = useConfigStore()

onMounted(() => {
})

async function handleLater() {
  await invoke('hide_window', { label: 'pet' })
}

async function handleClose() {
  await invoke('hide_window', { label: 'pet' })
}

async function handleDisable() {
  await configStore.updateConfig({ petEnabled: false })
  await invoke('hide_window', { label: 'pet' })
}
</script>

<style scoped lang="scss">
.pet-window {
  width: 100%;
  height: 100%;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 20px;

  .pet-content {
    text-align: center;
    color: white;

    .pet-animation {
      width: 120px;
      height: 120px;
      margin: 0 auto 20px;
      background: rgba(255, 255, 255, 0.2);
      border-radius: 50%;
      display: flex;
      align-items: center;
      justify-content: center;
      font-size: 60px;
      animation: bounce 2s infinite;
    }

    .pet-message {
      font-size: 18px;
      margin-bottom: 20px;
      font-weight: 500;
    }

    .pet-actions {
      display: flex;
      gap: 8px;
      justify-content: center;

      :deep(.el-button) {
        background: rgba(255, 255, 255, 0.2);
        border: none;
        color: white;

        &:hover {
          background: rgba(255, 255, 255, 0.3);
        }
      }
    }
  }
}

@keyframes bounce {
  0%, 100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-10px);
  }
}
</style>