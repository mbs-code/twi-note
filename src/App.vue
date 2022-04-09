<template>
  <pre>
    {{ reports }}
  </pre>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { Report, useReport } from './composables/useReport'

const reportRepo = useReport()
const reports = ref<Report[]>([])

onMounted(async () => {
  const data = await reportRepo.getAll()
  reports.value = data
})
</script>
