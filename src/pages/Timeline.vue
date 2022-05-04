<template>
  <n-layout-header bordered position="absolute">
    <div ref="headerRef" style="padding: 4px">
      <SearchPanel @search="onSearch" />
    </div>
  </n-layout-header>

  <n-layout
    position="absolute"
    :style="{ top: headerHeight + 'px', bottom: footerHeight + 'px' }"
    :native-scrollbar="false"
  >
    <div style="padding-right: 16px">
      <ReportTimeline
        v-model:is-initial="isInitial"
        :reports="reports"
        @load="onInfiniteLoad"
        @update:after="listUpdated"
        @delete:after="listDeleted"
      />
    </div>
  </n-layout>

  <n-layout-footer bordered position="absolute">
    <div ref="footerRef">
      <n-card class="card-dense">
        <ReportEditBox
          v-model:expand="configStore.expand_editor"
          :show-expand="true"
          @save:after="listCreated"
        />
      </n-card>
    </div>
  </n-layout-footer>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue'
import { LoadAction } from '@ts-pro/vue-eternal-loading'
import { Report, SearchReport, useReportAPI } from '../composables/useReportAPI'
import { useConfigStore } from '../stores/config'

const reportAPI = useReportAPI()
const configStore = useConfigStore()

/// ////////////////////////////////////////////////////////////
/// 検索機能

const reports = ref<Report[]>([])
const isInitial = ref<boolean>(false)
const searchParams = ref<SearchReport>({
  text: undefined,
  page: 1,
  count: 5,
  latest: true,
})

const fetchReports = async () => {
  const items = await reportAPI.getAll(searchParams.value)
  reports.value.push(...items)
}

const resetReports = () => {
  reports.value = []
  isInitial.value = true
  searchParams.value.page = 1
}

// NOTE: onMounted は onInfinite で処理される
// NOTE: 初回取得は行う必要が無い

///

const onSearch = async (text: string) => {
  searchParams.value.text = text ?? undefined
  resetReports()
}

const onInfiniteLoad = async ({ loaded, noMore, error }: LoadAction) => {
  const beforeReportLength = reports.value.length
  searchParams.value.page++

  try {
    await fetchReports()

    const afterReportLength = reports.value.length
    if (beforeReportLength === afterReportLength) {
      noMore()
    } else {
      loaded()
    }
  } catch (err) {
    error()
  }
}

/// ////////////////////////////////////////////////////////////
/// 配列更新機能

const listCreated = (createReport: Report) => {
  reports.value.unshift(createReport)
}

const listUpdated = (updateReport: Report) => {
  const index = reports.value.findIndex((report) => report.id === updateReport.id)
  if (index >= 0) {
    // 同IDがあれば置き換える
    reports.value.splice(index, 1, updateReport)
  } else {
    // 該当が無かったら先頭に追加
    reports.value.unshift(updateReport)
  }
}

const listDeleted = (deleteReport: Report) => {
  const index = reports.value.findIndex((report) => report.id === deleteReport.id)
  if (index >= 0) {
    // 同IDがあれば削除する
    reports.value.splice(index, 1)
  }
}

/// ////////////////////////////////////////////////////////////
/// 高さ計算機能

const headerHeight = ref(0)
const footerHeight = ref(0)

const headerRef = ref<HTMLDivElement>()
const footerRef = ref<HTMLDivElement>()
const sizeObserver = ref<ResizeObserver>()

onMounted(() => {
  const observer = new ResizeObserver(() => {
    headerHeight.value = (headerRef.value?.clientHeight ?? 0) + 1
    footerHeight.value = (footerRef.value?.clientHeight ?? 0) + 1
  })

  if (headerRef.value) observer.observe(headerRef.value)
  if (footerRef.value) observer.observe(footerRef.value)
  sizeObserver.value = observer
})

onUnmounted(() => {
  const observer = sizeObserver.value
  if (observer) {
    if (headerRef.value) observer.unobserve(headerRef.value)
    if (footerRef.value) observer.unobserve(footerRef.value)
  }
})
</script>
