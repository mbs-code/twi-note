<template>
  <ReportCreatePanel @change="fetchData"></ReportCreatePanel>

  <pre>
    {{ reports }}
  </pre>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { Report, useReport } from '../composables/useReport'

const reportRepo = useReport()
const reports = ref<Report[]>([])

const fetchData = async () => {
  const data = await reportRepo.getAll()
  reports.value = data
}

onMounted(async () => {
  await fetchData()
})
</script>
