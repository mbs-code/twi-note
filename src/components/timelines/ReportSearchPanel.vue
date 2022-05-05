<template>
  <div class="d-flex flex-align-center" style="height: 24px">
    <n-input
      v-model:value="_value"
      class="flex-grow-1"
      placeholder="検索"
      size="small"
      clearable
      @keydown.enter.exact="onSearch"
    >
      <template #prefix>
        <n-button text>
          <n-icon :component="SearchIcon" />
        </n-button>
      </template>
    </n-input>

    <n-button size="small" @click="onSearch">
      検索
    </n-button>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { Search as SearchIcon } from '@vicons/ionicons5'

const props = defineProps<{ value: string }>()
const emit = defineEmits<{
  (e: 'update:value', value: string): void,
  (e: 'search', text: string): void,
}>()

const _value = computed({
  get: () => props.value,
  set: (value: string) => emit('update:value', value)
})

const onSearch = () => emit('search', props.value)
</script>
