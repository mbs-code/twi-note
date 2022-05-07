<template>
  <div class="d-flex flex-align-center" style="height: 24px">
    <n-input
      v-model:value="bufferText"
      class="flex-grow-1"
      placeholder="検索"
      size="small"
      clearable
      @clear="onClearSearch"
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

    <n-button
      size="small"
      :disabled="!bufferText.length > 0"
      @click="openPhraseDialog"
    >
      保存
    </n-button>

    <n-tooltip trigger="hover">
      <template #trigger>
        <n-button size="small" @click="openSearchDialog">
          <template #icon>
            <n-icon :component="AdvanceIcon" />
          </template>
        </n-button>
      </template>

      <div>高度な検索</div>
    </n-tooltip>
  </div>

  <PhraseEditDialog
    v-model:show="showPhraseDialog"
    :text="bufferText"
  />

  <ReportSearchDialog
    v-model:show="showSearchDialog"
    :search-text="bufferText"
    @search="onAdvanceSearch"
  />
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import {
  Search as SearchIcon,
  EllipsisHorizontal as AdvanceIcon,
} from '@vicons/ionicons5'
import { computed } from '@vue/reactivity'

const props = defineProps<{ searchText: string }>()
const emit = defineEmits<{
  (e: 'search', text: string): void,
}>()

const _searchText = computed(() => props.searchText)
watch(_searchText, (text) => {
  // 値が変わったらバッファも更新する
  bufferText.value = text
})

/// ////////////////////////////////////////////////////////////
/// ダイアログ管理

const showSearchDialog = ref<boolean>(false)
const openSearchDialog = () => {
  showSearchDialog.value = true
}

const showPhraseDialog = ref<boolean>(false)
const openPhraseDialog = () => {
  showPhraseDialog.value = true
}

/// ////////////////////////////////////////////////////////////
/// フォーム管理

const bufferText = ref<string>('')

const onSearch = () => {
  emit('search', bufferText.value)
}

const onClearSearch = () => {
  emit('search', '')
}

const onAdvanceSearch = (text: string) => {
  emit('search', text)
}
</script>
