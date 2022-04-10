<template>
  <div>{{ report.title }}</div>
  <div style="white-space: pre-wrap">{{ report.body }}</div>

  <n-space class="expand-first">
    <div>たぐ</div>


    <n-tooltip trigger="hover">
      <template #trigger>
        <div style="display: flex; align-items: center;">
          <div v-if="isEdited" style="color: silver">（編集済み）</div>

          <n-icon style="padding-right: 4px"><ClockIcon /></n-icon>
          <span>{{ updatedDistance }}</span>
        </div>
      </template>

      <div>作成日: {{ report.created_at }}</div>
      <div>更新日: {{ report.updated_at }}</div>
    </n-tooltip>
  </n-space>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { formatDistanceToNow } from 'date-fns'
import ja from 'date-fns/locale/ja'
import { Report } from '../composables/useReportAPI'
import { parseUtc } from '../utils/DateUtil'
import { TimeOutline as ClockIcon } from '@vicons/ionicons5'

const props = defineProps<{ report: Report }>()

const isEdited = computed(() => {
  return props.report.created_at !== props.report.updated_at
})
const updatedDistance = computed(() => {
  const updatedDate = parseUtc(props.report.updated_at)
  return formatDistanceToNow(updatedDate, { addSuffix: true, locale: ja })
})
</script>
