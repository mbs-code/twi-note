<template>
  <n-modal
    v-model:show="_show"
    preset="card"
    style="width: 480px"
    size="small"
    segmented
    :mask-closable="false"
    @keydown.ctrl.enter.exact="onSave"
  >
    <template #header>
      <div class="d-flex">
        <n-icon-wrapper :size="24" color="var(--n-color)">
          <n-icon :component="DialogIcon" />
        </n-icon-wrapper>
        <span>検索フレーズの保存</span>
      </div>
    </template>

    <n-space vertical>
      <n-form-item label="フレーズ">
        <n-input :value="formText" />
      </n-form-item>

      <n-form-item label="色">
        <n-color-picker
          v-model:value="formColor"
          :modes="['hex']"
          :show-alpha="false"
          :swatches="swatches"
        />
      </n-form-item>

      <n-form-item label="優先度">
        <n-input-number
          v-model:value="formPriority"
          clearable
        />
      </n-form-item>
    </n-space>

    <template #action>
      <div class="d-flex">
        <n-button size="small" @click="onReset">
          リセット
        </n-button>

        <div class="flex-grow-1" />

        <n-button
          type="primary"
          size="small"
          :disabled="!isValidated"
          @click="onSave"
        >
          保存
        </n-button>
      </div>
    </template>
  </n-modal>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import { BookmarkOutline as DialogIcon } from '@vicons/ionicons5'
import { FormPhrase, Phrase, usePhraseAPI } from '../../composables/usePhraseAPI'
import { useMessage } from 'naive-ui'

const props = defineProps<{
  show: boolean,
  text: string,
  phrase?: Phrase,
}>()
const emit = defineEmits<{
  (e: 'update:show', value: boolean): void,
  (e: 'change', value: Phrase): void,
}>()

const _show = computed({
  get: () => props.show,
  set: (value) => emit('update:show', value),
})

const phraseAPI = usePhraseAPI()
const message = useMessage()

/// ////////////////////////////////////////////////////////////
/// フォーム管理

const formText = ref<string>('')
const formColor = ref<string>('')
const formPriority = ref<number>(0)

const init = () => {
  formText.value = (props.text.trim() || props.phrase?.text.trim()) || ''
  formColor.value = props.phrase?.color || ''
  formPriority.value = props.phrase?.priority ?? 0
}

const onReset = () => init()
watch(_show, () => {
  if (_show.value) init()
})

/// ////////////////////////////////////////////////////////////
/// フォーム処理

// バリデーション
const isValidated = computed(() => {
  if (!formText.value?.trim())  return false // text は必須
  return true
})

// 保存処理
const onSave = async () => {
  try {
    // バリデーションを通るか確認
    if (!isValidated.value) {
      message.error('入力値エラー')
      return
    }

    // データ成形
    const id = props.phrase?.id
    const item: FormPhrase = {
      text: formText.value.trim() || '',
      color: formColor.value || undefined,
      priority: formPriority.value ?? 0,
    }

    // 実行
    const newPhrase = id
      ? await phraseAPI.update(id, item)
      : await phraseAPI.create(item)

    message.success(`フレーズを保存しました (${newPhrase.id})`)
    emit('change', newPhrase)
    onClose()
  } catch (err) {
    console.log(err)
    message.error('内部エラーが発生しました')
  }
}

const onClose = () => {
  _show.value = false
}

const swatches = [
  '#E57373', '#F06292', '#BA68C8', '#9575CD', '#7986CB', '#64B5F6',
  '#4FC3F7', '#4DD0E1', '#4DB6AC', '#81C784', '#AED581', '#DCE775',
  '#FFF176', '#FFD54F', '#FFB74D', '#FF8A65', '#A1887F', '#E0E0E0',
  '#90A4AE',
]
</script>
