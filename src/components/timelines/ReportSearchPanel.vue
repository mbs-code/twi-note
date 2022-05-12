<template>
  <div class="d-flex flex-align-center" style="height: 24px">
    <n-input
      v-model:value="reportQuery.query.value"
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

    <n-button
      v-if="configStore.use_phrase"
      size="small"
      :disabled="!reportQuery.hasQuery()"
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
    :text="reportQuery.query.value"
  />

  <ReportSearchDialog
    v-model:show="showSearchDialog"
    :query="reportQuery.query.value"
    @search="onAdvanceSearch"
  />
</template>

<script setup lang="ts">
import { inject, ref } from 'vue'
import {
  Search as SearchIcon,
  EllipsisHorizontal as AdvanceIcon,
} from '@vicons/ionicons5'
import { reportQueryKey, ReportQueryType } from '../../composables/timelines/useReportQuery'
import { useConfigStore } from '../../stores/config'

defineProps<{ query: string }>()
const emit = defineEmits<{
  (e: 'search', text: string): void,
}>()

const configStore = useConfigStore()
const reportQuery = inject(reportQueryKey) as ReportQueryType

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

const onSearch = () => {
  emit('search', reportQuery.query.value)
}

const onAdvanceSearch = (text: string) => {
  reportQuery.setQuery(text)
}
</script>
