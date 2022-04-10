<template>
  <n-card>
    <ReportCreateBox @change="onCreated"></ReportCreateBox>
  </n-card>

  <template v-for="report in reports">
    <ReportPanel :report="report" @change="onUpdated"></ReportPanel>
  </template>

  <VueEternalLoading :load="onInfinite"></VueEternalLoading>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { Report, useReportAPI } from '../composables/useReportAPI'
import { VueEternalLoading, LoadAction } from '@ts-pro/vue-eternal-loading'

const reportAPI = useReportAPI()
const reports = ref<Report[]>([])

// 検索処理
let page = 1;
const fetchReports = async () => {
  const data = await reportAPI.getAll({
    page: page,
    count: 20,
    latest: true,
  })
  return data
}

// 無限スクロール処理
const onInfinite  = async ({ loaded, noMore, error }: LoadAction) => {
  console.log('infinite')

  try {
    const data = await fetchReports()
    if (data.length === 0) {
      noMore()
    } else {
      reports.value.push(...data)
      page ++;
      loaded()
    }
  } catch (err) {
    error()
  }
}

// 更新処理
const onCreated = (report: Report) => {
  reports.value.unshift(report)
}
const onUpdated = (report: Report) => {
  const index = reports.value.findIndex((rp) => rp.id === report.id)
  if (index >= 0) {
    reports.value.splice(index, 1, report)
  } else {
    // 無いはず
    reports.value.unshift(report)
  }
}
</script>
