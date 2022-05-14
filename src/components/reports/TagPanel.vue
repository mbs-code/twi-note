<template>
  <span
    v-if="color"
    class="no-select"
    :style="{ color }"
  >
    ●
  </span>
  <span v-else>&emsp;</span>

  {{ name }}
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { Tag } from '../../composables/useTagAPI'

const props = defineProps<{
  tag?: Tag,
  text?: string,
}>()

const color = computed(() => props.tag?.color)
const name = computed(() => props.tag?.name ?? props.text)
</script>
