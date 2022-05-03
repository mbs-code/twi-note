<template>
  <n-space vertical>
    <n-input v-model:value="search" placeholder="検索" @change="onSearch">
      <template #prefix>
        <n-button text>
          <n-icon :component="SearchIcon" />
        </n-button>
      </template>
    </n-input>

    <n-card class="card-dense">
      <ReportEditBox @saved="handleCreated" />
    </n-card>

    <template
      v-for="report in reports"
      :key="report.id"
    >
      <ReportPanel
        :report="report"
        @updated="handleUpdated"
        @deleted="handleDeleted"
      />
    </template>

    <VueEternalLoading :load="onInfinite">
      <template #loading>
        <n-space justify="center">
          <n-spin />
        </n-space>
      </template>

      <template #no-more>
        <n-space justify="center">
          ～ココマデ～
        </n-space>
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
  </n-space>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import {
  Search as SearchIcon,
} from '@vicons/ionicons5'
import { Report, useReportAPI } from '../composables/useReportAPI'
import { VueEternalLoading, LoadAction } from '@ts-pro/vue-eternal-loading'
import { useRoute, useRouter } from 'vue-router'

const router = useRouter()
const route = useRoute()
const reportAPI = useReportAPI()
const reports = ref<Report[]>([])

// 検索処理
let page = 1
const search = ref<string>('')
const onSearch = () => {
  console.log(search.value)
  router.push({ path: '/', query: {
    ...route.query,
    text: search.value,
  }})
}

const fetchReports = async () => {
  const tag = route.query?.tag as string // url parameter
  const text = route.query?.text as string

  const data = await reportAPI.getAll({
    text: text ?? undefined,
    tagName: tag ?? undefined,
    page: page,
    count: 20,
    latest: true
  })
  return data
}

// 無限スクロール処理
const endInfinite = ref(false)
const onInfinite  = async ({ loaded, noMore, error }: LoadAction) => {
  // 一度 noMore になったら触らない（バグ？対策）
  if (endInfinite.value) {
    noMore()
    return
  }

  try {
    const data = await fetchReports()
    if (data.length === 0) {
      noMore()
      endInfinite.value = true
    } else {
      reports.value.push(...data)
      page ++
      loaded()
    }
  } catch (err) {
    error()
  }
}

// 保持リスト更新処理
const handleCreated = (creReport: Report) => {
  reports.value.unshift(creReport)
}
const handleUpdated = (updReport: Report) => {
  const index = reports.value.findIndex((report) => report.id === updReport.id)
  if (index >= 0) {
    // 置き換える
    reports.value.splice(index, 1, updReport)
  } else {
    // 該当が無かったら先頭に追加しておく（無いはず）
    reports.value.unshift(updReport)
  }
}
const handleDeleted = (delReport: Report) => {
  const index = reports.value.findIndex((report) => report.id === delReport.id)
  if (index >= 0) {
    // 置き換える
    reports.value.splice(index, 1)
  }
}
</script>
