<template>
  <n-card
    class="card-dense"
    embedded
    :bordered="false"
    size="small"
  >
    <n-space vertical>
      <div v-if="report.title">
        {{ report.title }}
      </div>

      <div style="white-space: pre-wrap">
        {{ report.body }}
      </div>

      <n-space class="expand-first" align="center">
        <n-space>
          <n-tag
            v-for="(tag, _) of report.tags"
            :key="_"
            @click="emit('click:tag', tag.name)"
          >
            <span
              v-if="tag.color"
              class="no-select"
              :style="{ color: tag.color }"
            >●</span>
            {{ tag.name }}
          </n-tag>
        </n-space>

        <n-tooltip trigger="hover">
          <template #trigger>
            <div class="d-flex flex-align-center flex-wrap-reverse">
              <div v-if="isEdited && !configStore.hide_edited" style="color: silver">
                （編集済み）
              </div>

              <div class="d-flex flex-align-center">
                <n-icon style="padding-right: 4px" :component="ClockIcon" />
                {{ timestampStr }}
              </div>
            </div>
          </template>

          <div>作成日: {{ createdStr }} ({{ createdDistStr }})</div>
          <div>更新日: {{ updatedStr }} ({{ updatedDistStr }})</div>
          <div v-if="isEdited">
            (編集済み)
          </div>
        </n-tooltip>
      </n-space>
    </n-space>
  </n-card>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { TimeOutline as ClockIcon } from '@vicons/ionicons5'
import { Report } from '../../composables/useReportAPI'
import { parseLocal, formatString, formatDistance } from '../../utils/DateUtil'
import { useConfigStore } from '../../stores/config'

const props = defineProps<{ report: Report }>()
const emit = defineEmits<{
  (e: 'click:tag', name: string): void,
}>()

const configStore = useConfigStore()

/// ////////////////////////////////////////////////////////////

const isEdited = computed(() =>  props.report.created_at !== props.report.updated_at)
const createdLocalDate = computed(() => parseLocal(props.report.created_at))
const updatedLocalDate = computed(() => parseLocal(props.report.updated_at))

const createdStr = computed(() => formatString(createdLocalDate.value))
const updatedStr = computed(() => formatString(updatedLocalDate.value))
const createdDistStr = computed(() => formatDistance(createdLocalDate.value))
const updatedDistStr = computed(() => formatDistance(updatedLocalDate.value))

const timestampStr = computed(() => {
  const refUpdatedAt = configStore.use_updated_at

  switch (configStore.timestamp_mode) {
    case 'relative':
      return refUpdatedAt ? updatedDistStr.value : createdDistStr.value
    case 'absolute':
      return refUpdatedAt ? updatedStr.value : createdStr.value
    default:
      return '-' // 起きないはず
  }
})
</script>
