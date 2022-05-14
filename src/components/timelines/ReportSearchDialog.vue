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
          v-model:value="text"
          placeholder="Label"
          clearable
        />
      </n-form-item>

      <n-form-item label="投稿日時">
        <n-space align="center" :wrap="false">
          <n-date-picker
            v-model:formatted-value="after"
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
            v-model:formatted-value="before"
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
        <TagSelect v-model:value="selectedTags" />
      </n-form-item>
    </n-space>

    <template #action>
      <div class="d-flex">
        <n-button size="small" @click="onClear">
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
import { computed, ref, watch } from 'vue'
import { Search as SearchIcon } from '@vicons/ionicons5'
import { ReportQueryUtil } from '../../utils/ReportQueryUtil'

const props = defineProps<{
  show: boolean,
  query: string,
}>()
const emit = defineEmits<{
  (e: 'update:show', value: boolean): void,
  (e: 'search', text: string): void,
}>()

const _show = computed({
  get: () => props.show,
  set: (value) => emit('update:show', value),
})

/// ////////////////////////////////////////////////////////////
/// フォーム管理

const text = ref<string>('')
const after = ref<string | null>()
const before = ref<string | null>()
const selectedTags = ref<string[]>([])

watch(_show, () => {
  // 開いたときに _search を各フォームにバインドする
  onReset()
})

///

const onClear = () => {
  text.value = ''
  after.value = undefined
  before.value = undefined
  selectedTags.value = []
}

const onReset = () => {
  const params = ReportQueryUtil.parse(props.query)
  text.value = params.text ?? ''
  after.value = params.after
  before.value = params.before
  selectedTags.value = params.tags
}

const onSearch = () => {
  const newQuery = ReportQueryUtil.generate({
    text: text.value,
    after: after.value ?? undefined,
    before: before.value ?? undefined,
    tags: selectedTags.value,
  })

  emit('search', newQuery)
  onClose()
}

const onClose = () => {
  _show.value = false
}
</script>
