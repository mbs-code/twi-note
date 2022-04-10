<template>
  <div>{{ report.title }}</div>
  <div style="white-space: pre-wrap">{{ report.body }}</div>
</template>

<script setup lang="ts">
import { Report } from '../composables/useReportAPI'

const props = defineProps<{ report: Report }>()
</script>
