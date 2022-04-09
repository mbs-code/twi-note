<template>
  <div>{{ report.title }}</div>
  <div>{{ report.body }}</div>
</template>

<script setup lang="ts">
import { Report } from '../composables/useReportAPI'

const props = defineProps<{ report: Report }>()
</script>
