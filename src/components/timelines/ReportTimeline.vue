<template>
  <!-- カード配列 -->
  <transition-group tag="div" name="list">
    <ReportPanel
      v-for="report in reports"
      :key="report.id"
      :report="report"
      style="margin: 4px"
      @update:after="emit('update:after', $event)"
      @delete:after="emit('delete:after', $event)"
      @click:tag="emit('click:tag', $event)"
    />
  </transition-group>

  <!-- 無限スクロール -->
  <VueEternalLoading
    v-model:is-initial="_isInitial"
    :load="onLoad"
    position="top"
  >
    <template #loading>
      <n-space justify="center">
        <n-spin />
      </n-space>
    </template>

    <template #no-more>
      <n-empty description="No Contents" style="margin: 12px" />
    </template>

    <template #error="{ retry }">
      <n-alert
        title="内部エラーが発生しました。"
        type="error"
      >
        <n-button
          strong
          secondary
          type="error"
          @click="retry"
        >
          再試行
        </n-button>
      </n-alert>
    </template>
  </VueEternalLoading>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { Report } from '../../composables/useReportAPI'
import { VueEternalLoading, LoadAction } from '@ts-pro/vue-eternal-loading'

const props = defineProps<{
  reports: Report[],
  isInitial: boolean,
}>()
const emit = defineEmits<{
  (e: 'update:is-initial', val: boolean): void,
  (e: 'load', action: LoadAction): void,
  (e: 'update:after', report: Report): void, // bridge
  (e: 'delete:after', report: Report): void, // bridge
  (e: 'click:tag', name: string): void, // bridge
}>()

/// ////////////////////////////////////////////////////////////
/// 無限ロード機能

const _isInitial = computed({
  get: () => props.isInitial,
  set: (val: boolean) => emit('update:is-initial', val)
})

const onLoad = (action: LoadAction) => {
  emit('load', action)
}

</script>

<style scoped>
.list-item {
  display: inline-block;
  margin-right: 10px;
}
.list-enter-active,
.list-leave-active {
  transition: all 0.2s ease;
}

.list-enter-from,
.list-leave-to {
  opacity: 0;
  transform: translateY(-30px);
}

.list-move {
  transition: transform 0.2s ease;
}
</style>
