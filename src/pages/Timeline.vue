<template>
  <n-layout-header bordered position="absolute">
    <div ref="headerRef" style="padding: 4px">
      <ReportSearchPanel
        :query="reportList.query.value"
        @search="onSearch"
      />
    </div>
  </n-layout-header>

  <n-layout
    ref="scrollRef"
    position="absolute"
    :style="{
      top: heights.headerHeight.value + 'px',
      bottom: heights.footerHeight.value + 'px',
    }"
    :native-scrollbar="false"
  >
    <div style="padding-right: 20px">
      <ReportTimeline
        v-model:is-initial="reportList.isInitial.value"
        :reports="reportList.reports.value"
        @load="reportList.onInfiniteLoad"
        @update:after="reportList.add($event)"
        @delete:after="reportList.remove($event)"
      />
    </div>

    <n-back-top
      :bottom="heights.backtoHeight.value"
      right="20"
      :visibility-height="10"
    />
  </n-layout>

  <n-layout-footer bordered position="absolute">
    <div ref="footerRef">
      <ReportEditSheet
        v-model:expand="configStore.expand_editor"
        :show-expand="true"
        @save:after="reportList.add($event)"
      />
    </div>
  </n-layout-footer>
</template>

<script setup lang="ts">
import { inject, ref } from 'vue'
import { useConfigStore } from '../stores/config'
import { NLayout } from 'naive-ui/lib/components'
import { useReportList } from '../composables/timelines/useReportList'
import { useHeights } from '../composables/timelines/useHeights'
import { injectKey, ReportQueryType } from '../composables/timelines/useReportQuery'

const configStore = useConfigStore()
const reportQuery = inject(injectKey) as ReportQueryType

reportQuery.addChangeEvent((query: string) => { onSearch(query) })

/// ////////////////////////////////////////////////////////////
/// 高さ計算機能

const headerRef = ref<HTMLDivElement>()
const footerRef = ref<HTMLDivElement>()
const heights = useHeights(headerRef, footerRef)

/// ////////////////////////////////////////////////////////////
/// レポート管理

const scrollRef = ref<typeof NLayout>()
const reportList = useReportList({
  // 追加した際にスクロールする
  scrollTo: (index: number) => {
    if (index === -1) {
      scrollRef.value?.scrollTo({ top: 0, behavior: 'smooth' })
    }
  }
})

// 再検索する
const onSearch = (query: string) => {
  reportList.query.value = query
  reportList.reload()
}
</script>
