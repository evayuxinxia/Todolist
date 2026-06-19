export interface Task {
  id: number
  content: string
  deadline: string
  priority: '紧急' | '高' | '中' | '低'
  workload: number
  remark?: string
  completed: boolean
  is_remind: boolean
  createdAt: string
}

export interface AppConfig {
  petEnabled: boolean
  petInterval: number
  theme: 'light' | 'dark'
  autoStart: boolean
  apiKey?: string
}
