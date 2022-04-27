<template>
  <n-space vertical>
    <div>{{ report.title }}</div>
    <div style="white-space: pre-wrap">{{ report.body }}</div>

    <n-space class="expand-first">
      <n-space>
        <n-tag v-for="tag of report.tags">
          <span v-if="tag.color" :style="{ color: tag.color, userSelect: 'none' }">●</span>
          {{ tag.name }}
        </n-tag>
      </n-space>

      <n-tooltip trigger="hover">
        <template #trigger>
          <div style="display: flex; align-items: center;">
            <div v-if="isEdited" style="color: silver">（編集済み）</div>

            <n-icon style="padding-right: 4px"><ClockIcon /></n-icon>
            <span>{{ updatedDistance }}</span>
          </div>
        </template>

        <div>作成日: {{ createdStr }}</div>
        <div>更新日: {{ updatedStr }}</div>
      </n-tooltip>
    </n-space>
  </n-space>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { TimeOutline as ClockIcon } from '@vicons/ionicons5'
import { Report } from '../../composables/useReportAPI'
import { parseLocal, formatString, formatDistance } from '../../utils/DateUtil'

const props = defineProps<{ report: Report }>()

const isEdited = computed(() =>  props.report.created_at !== props.report.updated_at)
const createdLocalDate = computed(() => parseLocal(props.report.created_at))
const updatedLocalDate = computed(() => parseLocal(props.report.updated_at))

const createdStr = computed(() => formatString(createdLocalDate.value))
const updatedStr = computed(() => formatString(updatedLocalDate.value))
const updatedDistance = computed(() => formatDistance(updatedLocalDate.value))
</script>
