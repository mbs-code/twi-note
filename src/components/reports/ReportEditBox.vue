<template>
  <n-space
    vertical
    @keydown.ctrl.enter.exact="onSave"
  >
    <n-input
      v-model:value="formTitle"
      placeholder="(Title)"
      clearable
      @keyup.enter.exact="onTitleEnter"
    />

    <n-input
      ref="inputBodyRef"
      v-model:value="formBody"
      type="textarea"
      placeholder="Body"
      clearable
      :autosize="{ minRows: 3 }"
    />

    <ArrayTagForm
      ref="tagNamesRef"
      v-model:value="formTagNames"
    />

    <n-space>
      <n-button
        round
        :type="isEdit ? 'warning' : 'primary'"
        :disabled="!validate"
        @click="onSave"
      >
        保存(Ctrl+Enter)
      </n-button>

      <n-button
        round
        type="default"
        @click="reset"
      >
        リセット
      </n-button>
    </n-space>
  </n-space>
</template>

<script setup lang="ts">
import { useMessage } from 'naive-ui'
import { computed, nextTick, onMounted, ref } from 'vue'
import { useRoute } from 'vue-router'
import { FormReport, Report, useReportAPI } from '../../composables/useReportAPI'
import ArrayTagForm from '../ArrayTagForm.vue'

const props = defineProps<{ report?: Report }>()
const emit = defineEmits<{ (e: 'saved', report: Report): void }>()
const message = useMessage()

// フォーカス処理
const inputBodyRef = ref<HTMLTextAreaElement | null>(null)
const onTitleEnter = () => {
  nextTick(() => { inputBodyRef.value?.focus() })
}

const defaultTags = () => {
  // デフォタグ配列を作る
  const name = route.query?.tag as string // url parameter
  return name ? [name] : []
}

// 初期化処理
const formTitle = ref<string>()
const formBody = ref<string>()
const formTagNames = ref<string[]>([])

const route = useRoute()
const reset = () => {
  formTitle.value = props.report?.title ?? ''
  formBody.value = props.report?.body ?? ''
  formTagNames.value = props.report?.tags.map((tag) => tag.name) ?? defaultTags()

  // インプットも初期化
  tagNamesRef.value?.onClear()
}
onMounted(() => reset())

const isEdit = computed(() => props.report?.id)

// バリデーション
const validate = () => {
  if (!formBody.value)  return false // body は必須
  return true
}

// 保存処理
const reportAPI = useReportAPI()
const tagNamesRef = ref<typeof ArrayTagForm | null>()
const onSave = async () => {
  // バリデーション
  if (!validate()) {
    message.error('入力値エラー')
    return
  }

  // 入力中のタグがあったら確定させる
  tagNamesRef.value?.onInput()

  // データ成形
  const id = props.report?.id
  const item: FormReport = {
    title: formTitle.value,
    body: formBody.value ?? '',
    tag_names: formTagNames.value ?? [],
  }

  // 実行
  const newReport = id
    ? await reportAPI.update(id, item)
    : await reportAPI.create(item)

  emit('saved', newReport)
  reset()
}
</script>
