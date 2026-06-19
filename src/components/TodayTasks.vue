<template>
  <div class="today-tasks">
    <div v-if="taskStore.loading" class="loading">
      <el-icon class="is-loading"><Loading /></el-icon>
      加载中...
    </div>
    <div v-else-if="taskStore.todayTasks.length === 0" class="empty">
      <el-empty description="今日暂无待办任务" />
    </div>
    <div v-else class="task-list">
      <div
        v-for="task in taskStore.todayTasks"
        :key="task.id"
        class="task-item"
        :class="{ completed: task.completed }"
      >
        <el-checkbox
          :model-value="task.completed"
          @change="taskStore.toggleComplete(task.id)"
        />
        <div class="task-content">
          <div class="task-text">{{ task.content }}</div>
          <div class="task-meta">
            <el-tag :type="getPriorityType(task.priority)" size="small">
              {{ task.priority }}
            </el-tag>
            <span class="deadline">{{ task.deadline }}</span>
            <span class="workload">工作量: {{ task.workload }}</span>
          </div>
        </div>
        <el-button
          type="danger"
          size="small"
          circle
          @click="taskStore.deleteTask(task.id)"
        >
          <el-icon><Delete /></el-icon>
        </el-button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useTaskStore } from '@/stores/task'
import { Loading, Delete } from '@element-plus/icons-vue'

const taskStore = useTaskStore()

function getPriorityType(priority: string) {
  const types: Record<string, any> = {
    '紧急': 'danger',
    '高': 'warning',
    '中': 'info',
    '低': ''
  }
  return types[priority] || ''
}
</script>

<style scoped lang="scss">
.today-tasks {
  .loading {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 40px;
  }

  .task-list {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .task-item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 16px;
    background: var(--el-fill-color-light);
    border-radius: 8px;
    transition: all 0.3s;

    &.completed {
      opacity: 0.6;

      .task-text {
        text-decoration: line-through;
      }
    }

    &:hover {
      background: var(--el-fill-color);
    }

    .task-content {
      flex: 1;

      .task-text {
        font-size: 16px;
        margin-bottom: 8px;
      }

      .task-meta {
        display: flex;
        align-items: center;
        gap: 12px;
        font-size: 12px;
        color: var(--el-text-color-secondary);

        .deadline {
          color: var(--el-color-warning);
        }
      }
    }
  }
}
</style>