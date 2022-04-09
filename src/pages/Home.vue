<template>
  <n-card>
    <ReportCreateBox @change="fetchData"></ReportCreateBox>
  </n-card>

  <template v-for="report in reports">
    <ReportPanel :report="report" @change="fetchData"></ReportPanel>
  </template>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { Report, useReportAPI } from '../composables/useReportAPI'

const reoirtAPI = useReportAPI()
const reports = ref<Report[]>([])

const fetchData = async () => {
  const data = await reoirtAPI.getAll()
  reports.value = data
}

onMounted(async () => {
  await fetchData()
})
</script>
