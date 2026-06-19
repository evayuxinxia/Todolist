<template>
  <div class="all-tasks">
    <div class="filter-bar">
      <el-select v-model="filterPriority" placeholder="优先级筛选" clearable>
        <el-option label="全部" value="" />
        <el-option label="紧急" value="紧急" />
        <el-option label="高" value="高" />
        <el-option label="中" value="中" />
        <el-option label="低" value="低" />
      </el-select>
      <el-checkbox v-model="showCompleted">显示已完成</el-checkbox>
    </div>

    <div class="task-list">
      <div
        v-for="task in filteredTasks"
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
          <div v-if="task.remark" class="task-remark">{{ task.remark }}</div>
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
import { ref, computed } from 'vue'
import { useTaskStore } from '@/stores/task'
import { Delete } from '@element-plus/icons-vue'

const taskStore = useTaskStore()
const filterPriority = ref('')
const showCompleted = ref(false)

const filteredTasks = computed(() => {
  return taskStore.tasks.filter(task => {
    if (filterPriority.value && task.priority !== filterPriority.value) {
      return false
    }
    if (!showCompleted.value && task.completed) {
      return false
    }
    return true
  })
})

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
.all-tasks {
  .filter-bar {
    display: flex;
    gap: 16px;
    margin-bottom: 20px;
    padding: 16px;
    background: var(--el-fill-color-light);
    border-radius: 8px;
  }

  .task-list {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .task-item {
    display: flex;
    align-items: flex-start;
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
        margin-bottom: 8px;

        .deadline {
          color: var(--el-color-warning);
        }
      }

      .task-remark {
        font-size: 13px;
        color: var(--el-text-color-secondary);
        padding: 8px;
        background: var(--el-fill-color);
        border-radius: 4px;
      }
    }
  }
}
</style>