<template>
  <div class="main-container" :class="configStore.config.theme">
    <el-container>
      <el-header>
        <div class="header-content">
          <h1>待办助手</h1>
          <div class="header-actions">
            <el-button @click="showSettings = true" circle>
              <el-icon><Setting /></el-icon>
            </el-button>
          </div>
        </div>
      </el-header>

      <el-main>
        <el-tabs v-model="activeTab">
          <el-tab-pane label="今日待办" name="today">
            <TodayTasks />
          </el-tab-pane>
          <el-tab-pane label="全部任务" name="all">
            <AllTasks />
          </el-tab-pane>
          <el-tab-pane label="AI 录入" name="ai">
            <AiInput />
          </el-tab-pane>
        </el-tabs>
      </el-main>
    </el-container>

    <SettingsDialog v-model="showSettings" />
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { Setting } from '@element-plus/icons-vue'
import { useConfigStore } from '@/stores/config'
import TodayTasks from '@/components/TodayTasks.vue'
import AllTasks from '@/components/AllTasks.vue'
import AiInput from '@/components/AiInput.vue'
import SettingsDialog from '@/components/SettingsDialog.vue'

const configStore = useConfigStore()
const activeTab = ref('today')
const showSettings = ref(false)
</script>

<style scoped lang="scss">
.main-container {
  height: 100vh;
  background: var(--el-bg-color);

  &.dark {
    background: #1a1a1a;
  }
}

.el-header {
  border-bottom: 1px solid var(--el-border-color);
  padding: 0 20px;

  .header-content {
    display: flex;
    justify-content: space-between;
    align-items: center;
    height: 100%;

    h1 {
      margin: 0;
      font-size: 20px;
      font-weight: 600;
    }
  }
}

.el-main {
  padding: 20px;
}
</style>