<template>
  <n-modal
    v-model:show="show"
    :mask-closable="false"
    preset="dialog"
    title="Dialog"
    content="Are you sure?"
    positive-text="保存"
    negative-text="キャンセル"
    @positive-click="onSave"
    @negative-click="onNegativeClick"
  >
    <n-space vertical>
      <n-form-item label="タグ名">
        <n-input
          v-model:value="formName"
          clearable
        />
      </n-form-item>

      <n-form-item label="色">
        <n-color-picker
          :modes="['hex']"
          v-model:value="formColor"
          :show-alpha="false"
          :swatches="swatches"
        />
      </n-form-item>

      <n-checkbox v-model:checked="formHasPinned">
        ピン留めする
      </n-checkbox>

      <n-form-item label="優先度">
        <n-input-number
          v-model:value="formPriority"
          clearable
        />
      </n-form-item>

    </n-space>
  </n-modal>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import { FormTag, Tag, useTagAPI } from '../../composables/useTagAPI'

const props = defineProps<{ show: boolean, tag: Tag | undefined }>()
const emit = defineEmits<{
  (e: 'update:show', value: boolean): void,
  (e: 'onChanged', tag: Tag): void,
}>()

const show = computed({
  get: () => props.show,
  set: (value) => emit('update:show', value),
})

///

const formName = ref<string>()
const formColor = ref<string>()
const formHasPinned = ref<boolean>()
const formPriority = ref<number>()

const init = () => {
  formName.value = props.tag?.name ?? ''
  formColor.value = props.tag?.color ?? ''
  formHasPinned.value = props.tag?.is_pinned ? true : false
  formPriority.value = props.tag?.priority ?? 0
}
onMounted(() => init())
const onReset = () => init()
watch(() => props.tag, () => init())

///

// 保存処理
const tagAPI = useTagAPI()
const onSave = async () => {
  if (!formName.value) return // name は必須

  // データ成形
  const id = props.tag?.id
  const item: FormTag = {
    name: formName.value,
    color: formColor.value,
    has_pinned: formHasPinned.value ?? false,
    priority: formPriority.value ?? 0,
  }

  // 実行
  const newTag = id
    ? await tagAPI.update(id, item)
    : await tagAPI.create(item)

  init()
  emit('onChanged', newTag)
}

const onNegativeClick = () => {
  show.value = false
}

// const swatches = [
//   '#f44336', '#E91E63', '#9C27B0', '#673AB7', '#3F51B5', '#2196F3',
//   '#03A9F4', '#00BCD4', '#009688', '#4CAF50', '#8BC34A', '#CDDC39',
//   '#FFEB3B', '#FFC107', '#FF9800', '#FF5722', '#795548', '#eeeeee',
//   '#9E9E9E', '#607D8B', '#424242',
// ]

const swatches = [
  '#E57373', '#F06292', '#BA68C8', '#9575CD', '#7986CB', '#64B5F6',
  '#4FC3F7', '#4DD0E1', '#4DB6AC', '#81C784', '#AED581', '#DCE775',
  '#FFF176', '#FFD54F', '#FFB74D', '#FF8A65', '#A1887F', '#E0E0E0',
  '#90A4AE',
]
</script>
