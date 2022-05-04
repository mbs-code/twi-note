<template>
  <n-layout-header bordered position="absolute">
    <div ref="headerRef" style="padding: 4px">
      <SearchPanel v-model:value="search" @search="onSearch" />
    </div>
  </n-layout-header>

  <n-layout
    ref="scrollRef"
    position="absolute"
    :style="{ top: headerHeight + 'px', bottom: footerHeight + 'px' }"
    :native-scrollbar="false"
  >
    <div style="padding-right: 18px">
      <ReportTimeline
        v-model:is-initial="isInitial"
        :reports="reports"
        @load="onInfiniteLoad"
        @update:after="listUpdated"
        @delete:after="listDeleted"
        @click:tag="onTagClick"
      />
    </div>

    <n-back-top :bottom="backtoHeight" right="20" :visibility-height="10" />
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
import { Report, useReportAPI } from '../composables/useReportAPI'
import { useConfigStore } from '../stores/config'
import { NLayout } from 'naive-ui/lib/components'
import { useRoute } from 'vue-router'

const route = useRoute()
const reportAPI = useReportAPI()
const configStore = useConfigStore()

/// ////////////////////////////////////////////////////////////
/// 高さ計算機能

const headerHeight = ref(0)
const footerHeight = ref(0)
const backtoHeight = ref(0)

const headerRef = ref<HTMLDivElement>()
const footerRef = ref<HTMLDivElement>()
const sizeObserver = ref<ResizeObserver>()

onMounted(() => {
  const observer = new ResizeObserver(() => {
    headerHeight.value = (headerRef.value?.clientHeight ?? 0) + 1
    footerHeight.value = (footerRef.value?.clientHeight ?? 0) + 1
    backtoHeight.value = (footerRef.value?.clientHeight ?? 0) + 6
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

/// ////////////////////////////////////////////////////////////
/// 検索機能

const reports = ref<Report[]>([])
const isInitial = ref<boolean>(false) // infinite 用

const search = ref<string>('')
const page = ref<number>(0) // load 時に +1 する

const fetchReports = async () => {
  // 設定バッファ
  tlOnceCountBuffer.value = configStore.tl_once_count
  refUpdatedAtBuffer.value = configStore.ref_updated_at

  // データ取得
  const items = await reportAPI.getAll({
    text: search.value || undefined,
    page: page.value || 1,
    count: configStore.tl_once_count,
    latest: true,
    refUpdatedAt: configStore.ref_updated_at,
  })

  reports.value.push(...items)
}

const reloadReports = () => {
  reports.value = []
  isInitial.value = true
  page.value = 0
}

onMounted(() => {
  // もしタグが指定されていたら、検索パラメタに追加する
  const tag = route.query?.tag as string // url parameter
  if (tag) {
    search.value = `tag:${tag} `
  }
})

// NOTE: onMounted は onInfinite で処理される
// NOTE: 初回取得は行う必要が無い

///

const tlOnceCountBuffer = ref<number>(10)
const refUpdatedAtBuffer = ref<boolean>(false)
configStore.$subscribe((mutation, state) => {
  // 設定が変わった際に自動で再読み込みする
  if (tlOnceCountBuffer.value !== state.tl_once_count) {
    reloadReports()
  }

  if (refUpdatedAtBuffer.value !== state.ref_updated_at) {
    reloadReports()
  }
})

///

const onSearch = () => {
  // 再検索する
  reloadReports()
}

const onTagClick = (name: string) => {
  // タグクリック時に検索文字列に反映して、再検索する
  search.value = [search.value.trim(), `tag:${name} `].join(' ')
  reloadReports()
}

const onInfiniteLoad = async ({ loaded, noMore, error }: LoadAction) => {
  const beforeReportLength = reports.value.length
  page.value++ // ページカウント

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
  // 先頭に追加
  reports.value.unshift(createReport)
  scrollToTop()
}

const listUpdated = (updateReport: Report) => {
  const index = reports.value.findIndex((report) => report.id === updateReport.id)
  if (index >= 0) {
    // 同IDがあれば置き換える
    reports.value.splice(index, 1, updateReport)
  } else {
    // 該当が無かったら先頭に追加
    reports.value.unshift(updateReport)
    scrollToTop()
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
/// スクロール機能

const scrollRef = ref<typeof NLayout>()
const scrollToTop = () => {
  scrollRef.value?.scrollTo({ top: 0, behavior: 'smooth' })
}
</script>
