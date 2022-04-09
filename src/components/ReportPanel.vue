<template>
  <n-card>
    <template v-if="isEdit">
      <ReportCreateBox :report="report" @change="onSave"></ReportCreateBox>
    </template>
    <template v-else>
      <ReportShowBox :report="report"></ReportShowBox>
    </template>

    <n-button quaternary circle type="primary" @click="onDelete">
      <template #icon>
        <n-icon><TrashIcon /></n-icon>
      </template>
    </n-button>

    <n-button quaternary circle type="primary" @click="onEdit">
      <template #icon>
        <n-icon><PencilIcon /></n-icon>
      </template>
    </n-button>
  </n-card>
</template>

<script setup lang="ts">
import { useDialog } from 'naive-ui'
import { Report, useReportAPI } from '../composables/useReportAPI'
import {
  TrashBinOutline as TrashIcon,
  PencilSharp as PencilIcon,
} from '@vicons/ionicons5'
import { ref } from 'vue';

const props = defineProps<{ report: Report }>()
const emit = defineEmits<{ (e: 'change'): void }>()

// 編集・保存処理
const isEdit = ref(false)
const onEdit = () => { isEdit.value = !isEdit.value }
const onSave = () => {
  isEdit.value = false
  emit("change")
}

// 削除ダイアログ
const reportAPI = useReportAPI()
const dialog = useDialog()
const onDelete = () => {
  dialog.error({
    title: '確認',
    content: '削除しますか？',
    positiveText: 'はい',
    negativeText: 'いいえ',
    onPositiveClick: () => {
      return reportAPI.remove(props.report.id)
    },
    onNegativeClick: () => {}
  })
}
</script>
