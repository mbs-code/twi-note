<template>
  <n-space vertical>
    <n-input
      v-model:value="formTitle"
      placeholder="Title"
      clearable
      @keyup.enter.exact="onTitleEnter"
    />

    <n-input
      v-model:value="formBody"
      ref="bodyRef"
      type="textarea"
      placeholder="Body"
      clearable
      @keydown.ctrl.enter.exact="onSave"
      :autosize="{ minRows: 3 }"
    />

    <ArrayTagForm v-model:value="formTagNames" />

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
import { ReportWithTag, FormReport, useReportAPI } from '../../composables/useReportAPI'

const props = defineProps<{ report?: ReportWithTag }>()
const emit = defineEmits<{ (e: 'onChanged', report: ReportWithTag): void }>()
const bodyRef = ref<HTMLTextAreaElement | null>(null)

// フォーカス処理
const onTitleEnter = () => {
  bodyRef.value?.focus()
}

// 初期化処理
const formTitle = ref<string>()
const formBody = ref<string>()
const formTagNames = ref<string[]>([])

const init = () => {
  formTitle.value = props.report?.report.title ?? ''
  formBody.value = props.report?.report.body ?? ''
  formTagNames.value = props.report?.tags.map((tag) => tag.name) ?? []
}
onMounted(() => init())
const onReset = () => init()

// 保存処理
const reportAPI = useReportAPI()
const onSave = async () => {
  if (!formBody.value) return // body は必須

  // データ成形
  const id = props.report?.report.id
  const item: FormReport = {
    title: formTitle.value,
    body: formBody.value,
    tagNames: formTagNames.value,
  }

  // 実行
  const newReport = id
    ? await reportAPI.update(id, item)
    : await reportAPI.create(item)

  init()
  emit('onChanged', newReport)
}
</script>
