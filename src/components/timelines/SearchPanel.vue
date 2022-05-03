<template>
  <n-space class="expand-first">
    <n-input
      v-model:value="text"
      placeholder="検索"
      size="small"
      clearable
      @change="onSearch"
    >
      <template #prefix>
        <n-button text>
          <n-icon :component="SearchIcon" />
        </n-button>
      </template>
    </n-input>

    <n-button size="small">
      検索
    </n-button>
  </n-space>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import {
  Search as SearchIcon,
} from '@vicons/ionicons5'
import { useRoute } from 'vue-router'

const emit = defineEmits<{ (e: 'search', text: string): void }>()
const route = useRoute()

/// ////////////////////////////////////////////////////////////
/// 検索機能

const text = ref<string>('')
onMounted(() => {
  const q = route.query?.text as string
  text.value = q ?? ''
})

const onSearch = () => {
  emit('search', text.value)
}
</script>
