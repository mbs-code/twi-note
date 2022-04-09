<template>
  <n-input v-model:value="form.title" placeholder="Title" />

  <n-input
    v-model:value="form.body"
    type="textarea"
    placeholder="Body"
    :autosize="{
      minRows: 3,
      maxRows: 5
    }"
  />

  <n-space>
    <n-button round type="primary" @click="onSave">
      保存
    </n-button>

    <n-button round type="default" @click="onReset">
      リセット
    </n-button>
  </n-space>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { Report, FormReport, useReportAPI } from '../composables/useReportAPI'

const props = defineProps<{ report?: Report }>()
const emit = defineEmits<{ (e: 'change'): void }>()

// 初期化処理
const form = ref<FormReport>({})
const init = () => {
  form.value.title = props.report?.title ?? ''
  form.value.body = props.report?.body ?? ''
}
onMounted(() => init())
const onReset = () => init()

// 保存処理
const reportAPI = useReportAPI()
const onSave = async () => {
  const item = form.value
  if (!item.title && !item.body) return

  const id = props.report?.id
  if (id) {
    await reportAPI.update(id, item)
  } else {
    await reportAPI.create(item)
  }

  init()
  emit('change')
}
</script>
