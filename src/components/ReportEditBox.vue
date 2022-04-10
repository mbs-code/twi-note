<template>
  <n-space vertical>
    <n-input
      v-model:value="form.title"
      placeholder="Title"
      clearable
      @keyup.enter.exact="onTitleEnter"
    />

    <n-input
      v-model:value="form.body"
      ref="bodyRef"
      type="textarea"
      placeholder="Body"
      clearable
      @keydown.ctrl.enter.exact="onSave"
      :autosize="{ minRows: 3 }"
    />

    <n-space>
      <n-button round type="primary" @click="onSave">
        保存
      </n-button>

      <n-button round type="default" @click="onReset">
        リセット
      </n-button>
    </n-space>
  </n-space>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { Report, FormReport, useReportAPI } from '../composables/useReportAPI'

const props = defineProps<{ report?: Report }>()
const emit = defineEmits<{ (e: 'onChanged', report: Report): void }>()
const bodyRef = ref<HTMLTextAreaElement | null>(null)

// フォーカス処理
const onTitleEnter = () => {
  bodyRef.value?.focus()
}

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
  const newReport = id
    ? await reportAPI.update(id, item)
    : await reportAPI.create(item)

  init()
  emit('onChanged', newReport)
}
</script>
