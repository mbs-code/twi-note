<template>
  <n-card>
    <n-space class="expand-first">
      <div>
        <template v-if="isEdit">
          <ReportEditBox :report="report" @saved="handleUpdated"></ReportEditBox>
        </template>
        <template v-else>
          <ReportShowBox :report="report"></ReportShowBox>
        </template>
      </div>

      <n-space vertical justify="space-between" style="height: 100%">
        <n-button quaternary circle type="primary" @click="onEdit">
          <template #icon>
            <n-icon v-if="isEdit"><CloseIcon /></n-icon>
            <n-icon v-else><EditIcon /></n-icon>
          </template>
        </n-button>

        <n-button v-if="isEdit" circle type="error" @click="onDelete">
          <template #icon>
            <n-icon><DeleteIcon /></n-icon>
          </template>
        </n-button>
      </n-space>
    </n-space>
  </n-card>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useDialog } from 'naive-ui'
import { Report, useReportAPI } from '../../composables/useReportAPI'
import {
  CreateOutline as EditIcon,
  Trash as DeleteIcon,
  Close as CloseIcon,
} from '@vicons/ionicons5'

const props = defineProps<{ report: Report }>()
const emit = defineEmits<{
  (e: 'updated', reports: Report): void,
  (e: 'deleted', reports: Report): void,
}>()

// 編集・保存処理
const isEdit = ref(false)
const onEdit = () => { isEdit.value = !isEdit.value }
const handleUpdated = (report: Report) => {
  isEdit.value = false
  emit('updated', report)
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
        .then(report_id => {
          isEdit.value = false
          emit('deleted', props.report)
        })
    },
    onNegativeClick: () => {}
  })
}
</script>
