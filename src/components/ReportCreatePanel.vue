<template>
  <n-card>
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
    </n-space>
  </n-card>
</template>

<script setup lang="ts">
import { watch } from 'vue'
import { Report, FormReport, useReport } from '../composables/useReport'

const props = defineProps<{ report?: Report }>()
const emit = defineEmits<{ (e: 'change'): void }>()

const form: FormReport = {}
// @ts-ignore bug?
watch(() => props.report, (report: Report) => {
  form.title = report.title ?? undefined
  form.body = report.body ?? undefined
})

const reportRepo = useReport()
const onSave = async () => {
  if (!form.title && !form.body) return

  await reportRepo.create(form)
  emit('change')
}
</script>
