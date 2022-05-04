<template>
  <n-card class="card-dense">
    <n-space class="expand-first" :wrap="false">
      <ReportEditBox
        v-if="isEdit"
        :report="report"
        @save="onUpdateAfter"
      />
      <ReportShowBox
        v-else
        :report="report"
      />

      <n-space vertical justify="space-between" style="height: 100%">
        <n-button
          quaternary
          circle
          type="primary"
          @click="onEdit"
        >
          <template #icon>
            <n-icon v-if="isEdit" :component="CloseIcon" />
            <n-icon v-else :component="EditIcon" />
          </template>
        </n-button>

        <n-button
          v-if="isEdit"
          circle
          type="error"
          @click="openDeleteDialog"
        >
          <template #icon>
            <n-icon :component="DeleteIcon" />
          </template>
        </n-button>
      </n-space>
    </n-space>
  </n-card>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useDialog, useMessage } from 'naive-ui'
import { Report, useReportAPI } from '../../composables/useReportAPI'
import {
  CreateOutline as EditIcon,
  Trash as DeleteIcon,
  Close as CloseIcon,
} from '@vicons/ionicons5'

const props = defineProps<{ report: Report }>()
const emit = defineEmits<{
  (e: 'update:after', reports: Report): void,
  (e: 'delete:after', reports: Report): void,
}>()

const message = useMessage()
const dialog = useDialog()
const reportAPI = useReportAPI()

/// ////////////////////////////////////////////////////////////
/// フォーム基本機能

// 編集・保存処理
const isEdit = ref(false)
const onEdit = () => { isEdit.value = !isEdit.value }
const onUpdateAfter = (report: Report) => {
  isEdit.value = false
  emit('update:after', report)
}

// 削除ダイアログ
const openDeleteDialog = () => {
  dialog.error({
    title: '確認',
    content: '削除しますか？',
    positiveText: 'はい',
    negativeText: 'いいえ',
    onPositiveClick: async () => {
      await onDelete()
    },
  })
}

const onDelete = async () => {
  const report = props.report

  // 実行
  try {
    const res = await reportAPI.remove(props.report.id)
    if (!res) throw new Error('Failed to delete')

    emit('delete:after', report)
  } catch (err) {
    console.log(err)
    message.error('内部エラーが発生しました。')
  }
}
</script>
