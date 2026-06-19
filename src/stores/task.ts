import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { Task } from '@/types'

export const useTaskStore = defineStore('task', () => {
  const tasks = ref<Task[]>([])
  const loading = ref(false)

  const todayTasks = computed(() => {
    const today = new Date().toISOString().split('T')[0]
    return tasks.value
      .filter(t => !t.completed && t.deadline >= today)
      .sort((a, b) => {
        const priorityOrder = { '紧急': 0, '高': 1, '中': 2, '低': 3 }
        return priorityOrder[a.priority] - priorityOrder[b.priority]
      })
  })

  const upcomingTasks = computed(() => {
    const today = new Date()
    const threeDaysLater = new Date(today.getTime() + 3 * 24 * 60 * 60 * 1000)
      .toISOString().split('T')[0]
    const todayStr = today.toISOString().split('T')[0]

    return tasks.value.filter(
      t => !t.completed &&
      t.deadline >= todayStr &&
      t.deadline <= threeDaysLater &&
      t.isRemind
    )
  })

  async function loadTasks() {
    try {
      loading.value = true
      const result = await invoke<Task[]>('get_all_tasks')
      tasks.value = result
    } catch (error) {
      console.error('加载任务失败:', error)
    } finally {
      loading.value = false
    }
  }

  async function addTask(task: Omit<Task, 'id' | 'createdAt'>) {
    try {
      const newTask = await invoke<Task>('add_task', { task })
      tasks.value.push(newTask)
    } catch (error) {
      console.error('添加任务失败:', error)
      throw error
    }
  }

  async function updateTask(id: number, updates: Partial<Task>) {
    try {
      const updated = await invoke<Task>('update_task', { id, updates })
      const index = tasks.value.findIndex(t => t.id === id)
      if (index !== -1) {
        tasks.value[index] = updated
      }
    } catch (error) {
      console.error('更新任务失败:', error)
      throw error
    }
  }

  async function deleteTask(id: number) {
    try {
      await invoke('delete_task', { id })
      tasks.value = tasks.value.filter(t => t.id !== id)
    } catch (error) {
      console.error('删除任务失败:', error)
      throw error
    }
  }

  async function toggleComplete(id: number) {
    const task = tasks.value.find(t => t.id === id)
    if (task) {
      await updateTask(id, { completed: !task.completed })
    }
  }

  async function parseAiTask(text: string) {
    try {
      loading.value = true
      const result = await invoke<{ taskList: Omit<Task, 'id' | 'createdAt'>[] }>(
        'ai_parse_task',
        { text }
      )
      for (const task of result.taskList) {
        await addTask(task)
      }
      return result.taskList
    } catch (error) {
      console.error('AI解析失败:', error)
      throw error
    } finally {
      loading.value = false
    }
  }

  return {
    tasks,
    loading,
    todayTasks,
    upcomingTasks,
    loadTasks,
    addTask,
    updateTask,
    deleteTask,
    toggleComplete,
    parseAiTask,
  }
})