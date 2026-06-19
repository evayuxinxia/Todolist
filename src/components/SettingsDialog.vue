<template>
  <el-dialog
    v-model="dialogVisible"
    title="设置"
    width="500px"
    @close="handleClose"
  >
    <el-form :model="form" label-width="120px">
      <el-form-item label="宠物提醒">
        <el-switch v-model="form.petEnabled" />
      </el-form-item>

      <el-form-item label="提醒间隔">
        <el-select v-model="form.petInterval" :disabled="!form.petEnabled">
          <el-option label="30 分钟" :value="30" />
          <el-option label="1 小时" :value="60" />
          <el-option label="2 小时" :value="120" />
        </el-select>
      </el-form-item>

      <el-form-item label="主题">
        <el-radio-group v-model="form.theme">
          <el-radio label="light">浅色</el-radio>
          <el-radio label="dark">深色</el-radio>
        </el-radio-group>
      </el-form-item>

      <el-form-item label="开机自启">
        <el-switch v-model="form.autoStart" />
      </el-form-item>

      <el-divider />

      <el-form-item label="API Key">
        <el-input
          v-model="form.apiKey"
          type="password"
          placeholder="输入大模型 API Key"
          show-password
        />
      </el-form-item>
    </el-form>

    <template #footer>
      <el-button @click="handleClose">取消</el-button>
      <el-button type="primary" @click="handleSave">保存</el-button>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { ElMessage } from 'element-plus'
import { useConfigStore } from '@/stores/config'

interface Props {
  modelValue: boolean
}

const props = defineProps<Props>()
const emit = defineEmits<{
  (e: 'update:modelValue', value: boolean): void
}>()

const configStore = useConfigStore()
const dialogVisible = ref(props.modelValue)
const form = ref({ ...configStore.config })

watch(() => props.modelValue, (val) => {
  dialogVisible.value = val
  if (val) {
    form.value = { ...configStore.config }
  }
})

watch(dialogVisible, (val) => {
  emit('update:modelValue', val)
})

async function handleSave() {
  try {
    await configStore.updateConfig(form.value)
    ElMessage.success('设置已保存')
    dialogVisible.value = false
  } catch (error: any) {
    ElMessage.error('保存失败: ' + error)
  }
}

function handleClose() {
  dialogVisible.value = false
}
</script>

<style scoped lang="scss">
</style>