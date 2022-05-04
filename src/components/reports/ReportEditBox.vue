<template>
  <n-space
    vertical
    @keydown.ctrl.enter.exact="onSave"
  >
    <n-input
      v-model:value="formTitle"
      placeholder="(Title)"
      clearable
    />

    <n-input
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
        :disabled="!isValidated"
        @click="onSave"
      >
        保存(Ctrl+Enter)
      </n-button>

      <n-button
        round
        type="default"
        @click="resetForm"
      >
        リセット
      </n-button>
    </n-space>
  </n-space>
</template>

<script setup lang="ts">
import { useMessage } from 'naive-ui'
import { computed, onMounted, ref } from 'vue'
import { useRoute } from 'vue-router'
import { FormReport, Report, useReportAPI } from '../../composables/useReportAPI'
import ArrayTagForm from '../ArrayTagForm.vue'

const props = defineProps<{ report?: Report }>()
const emit = defineEmits<{ (e: 'save:after', createReport: Report): void }>()

const route = useRoute()
const message = useMessage()
const reportAPI = useReportAPI()

/// ////////////////////////////////////////////////////////////
/// フォーム基本機能

const isEdit = computed(() => props.report?.id)

const formTitle = ref<string>()
const formBody = ref<string>()
const formTagNames = ref<string[]>([])

const defaultTags = () => {
  const name = route.query?.tag as string // url parameter
  return name ? [name] : []
}

const resetForm = () => {
  formTitle.value = props.report?.title ?? ''
  formBody.value = props.report?.body ?? ''
  formTagNames.value = props.report?.tags.map((tag) => tag.name) ?? defaultTags()

  // インプットも初期化
  tagNamesRef.value?.onClear()
}

onMounted(() => resetForm())

/// ////////////////////////////////////////////////////////////
/// フォームアクション

// バリデーション
const isValidated = () => {
  if (!formBody.value)  return false // body は必須
  return true
}

// 保存処理
const tagNamesRef = ref<typeof ArrayTagForm>()
const onSave = async () => {
  // バリデーションを通るか確認
  console.log(isValidated())
  if (!isValidated()) {
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
  try {
    const newReport = id
      ? await reportAPI.update(id, item)
      : await reportAPI.create(item)

    emit('save:after', newReport)
    resetForm()
  } catch (err) {
    console.log(err)
    message.error('内部エラーが発生しました。')
  }
}
</script>
