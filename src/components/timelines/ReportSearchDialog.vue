<template>
  <n-modal
    v-model:show="_show"
    preset="card"
    style="width: 480px"
    size="small"
    segmented
    :mask-closable="false"
    @keydown.ctrl.enter.exact="onSearch"
  >
    <template #header>
      <div class="d-flex">
        <n-icon-wrapper :size="24" color="var(--n-color)">
          <n-icon :component="SearchIcon" />
        </n-icon-wrapper>
        <span>高度な検索</span>
      </div>
    </template>

    <n-space vertical>
      <n-form-item label="文字列（スペースでAND検索）">
        <n-input
          v-model:value="reportSearch.text.value"
          placeholder="Label"
          clearable
        />
      </n-form-item>

      <n-form-item label="投稿日時">
        <n-space align="center" :wrap="false">
          <n-date-picker
            v-model:formatted-value="reportSearch.after.value"
            type="date"
            placeholder="Start Date"
            clearable
            close-on-select
            :actions="[]"
            format="yyyy-MM-dd (EEE)"
            value-format="yyyy-MM-dd"
          />

          <span>～</span>

          <n-date-picker
            v-model:formatted-value="reportSearch.before.value"
            type="date"
            placeholder="End Date"
            clearable
            close-on-select
            :actions="[]"
            format="yyyy-MM-dd (EEE)"
            value-format="yyyy-MM-dd"
          />
        </n-space>
      </n-form-item>

      <n-form-item label="タグ">
        <n-select
          v-model:value="reportSearch.selectedTags.value"
          filterable
          multiple
          clearable
          :options="options"
        />
      </n-form-item>
    </n-space>

    <template #action>
      <div class="d-flex">
        <n-button size="small" @click="reportSearch.clear">
          クリア
        </n-button>
        <n-button size="small" @click="onReset">
          リセット
        </n-button>

        <div class="flex-grow-1" />

        <n-button type="primary" size="small" @click="onSearch">
          検索
        </n-button>
      </div>
    </template>
  </n-modal>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import { Search as SearchIcon } from '@vicons/ionicons5'
import { Tag, useTagAPI } from '../../composables/useTagAPI'
import { useReportSearchOld } from '../../composables/timelines/useReportSearchOld'

const props = defineProps<{
  show: boolean,
  searchText: string,
}>()
const emit = defineEmits<{
  (e: 'update:show', value: boolean): void,
  (e: 'search', text: string): void,
}>()

const tagAPI = useTagAPI()
const reportSearch = useReportSearchOld()

const _show = computed({
  get: () => props.show,
  set: (value) => emit('update:show', value),
})

/// ////////////////////////////////////////////////////////////
/// タグ管理

const tags = ref<Tag[]>([])
onMounted(async () => {
  tags.value = await tagAPI.getAll({
    hasPinned: false,
  })
})

const options = computed(() => {
  return tags.value
    .map((tag: Tag) => {
      return {
        label: tag.name,
        value: tag.name,
      }
    })
})

/// ////////////////////////////////////////////////////////////
/// フォーム管理

watch(_show, () => {
  // 開いたときに _search を各フォームにバインドする
  if (props.searchText) {
    reportSearch.bind(props.searchText)
  }
})

const onSearch = () => {
  const searchText = reportSearch.generate()
  emit('search', searchText)
  onClose()
}

const onReset = () => {
  reportSearch.bind(props.searchText)
}

const onClose = () => {
  _show.value = false
}
</script>
