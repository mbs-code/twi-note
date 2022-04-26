<template>
  <n-space vertical>
    <n-card>
      <ReportEditBox @saved="handleCreated"></ReportEditBox>
    </n-card>

    <template v-for="report in reports"  :key="report.id">
      <ReportPanel
        :report="report"
        @update="handleUpdated"
        @delete="handleDeleted"
      ></ReportPanel>
    </template>

    <VueEternalLoading :load="onInfinite">
      <template #loading>
        <n-space justify="center">
          <n-spin></n-spin>
        </n-space>
      </template>

      <template #no-more>
        <n-space justify="center">
          ～ココマデ～
        </n-space>
      </template>

      <template #error="{ retry }">
        <n-alert title="内部エラーが発生しました。" type="error">
          <n-button strong secondary type="error" @click="retry">
            再試行
          </n-button>
        </n-alert>
      </template>
    </VueEternalLoading>
  </n-space>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { ReportWithTag, useReportAPI } from '../composables/useReportAPI'
import { VueEternalLoading, LoadAction } from '@ts-pro/vue-eternal-loading'
import { useRoute } from 'vue-router';

const route = useRoute()
const reportAPI = useReportAPI()
const reports = ref<ReportWithTag[]>([])

// 検索処理
let page = 1;
const fetchReports = async () => {
  const tag = route.query?.tag as string

  const data = await reportAPI.getAll({
    tagName: tag ?? undefined,
    page: page,
    count: 20,
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

// 保持リスト更新処理
const handleCreated = (report: ReportWithTag) => {
  reports.value.unshift(report)
}
const handleUpdated = (report: ReportWithTag) => {
  const index = reports.value.findIndex((rp) => rp.report.id === report.report.id)
  if (index >= 0) {
    reports.value.splice(index, 1, report)
  } else {
    // 無いはず
    reports.value.unshift(report)
  }
}
const handleDeleted = (report: ReportWithTag) => {
  const index = reports.value.findIndex((rp) => rp.report.id === report.report.id)
  console.log(index)
  if (index >= 0) {
    reports.value.splice(index, 1)
  }
}
</script>
