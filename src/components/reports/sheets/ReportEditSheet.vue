<template>
  <!-- フルサイズモード -->
  <ReportEditFullSheet
    v-if="expand"
    v-model:title="formTitle"
    v-model:body="formBody"
    v-model:tag-names="formTagNames"
    :is-edit="isEdit"
    :show-expand="showExpand"
    :is-valid="isValidated"
    @save="onSave"
    @reset="resetForm"
    @expand="onExpandButton"
  />

  <!-- ミニマムモード -->
  <ReportEditSimpleSheet
    v-else
    v-model:body="formBody"
    :is-edit="isEdit"
    :show-expand="showExpand"
    :is-valid="isValidated"
    @save="onSave"
    @reset="resetForm"
    @expand="onExpandButton"
  />
</template>

<script setup lang="ts">
import { useMessage } from 'naive-ui'
import { computed, onMounted, ref } from 'vue'
import { useRoute } from 'vue-router'
import { FormReport, Report, useReportAPI } from '../../../composables/useReportAPI'
import ArrayTagForm from '../reports/ArrayTagForm.vue'

const props = defineProps<{
  report?: Report,
  expand: boolean,
  showExpand: boolean,
}>()
const emit = defineEmits<{
  (e: 'save:after', createReport: Report): void,
  (e: 'update:expand', val: boolean): void,
}>()

const route = useRoute()
const message = useMessage()
const reportAPI = useReportAPI()

const onExpandButton = () => {
  emit('update:expand', !props.expand)
}

/// ////////////////////////////////////////////////////////////
/// フォーム基本機能

const isEdit = computed(() => (props.report?.id ?? 0) > 0)

const formTitle = ref<string>('')
const formBody = ref<string>('')
const formTagNames = ref<string[]>([])

const defaultTags = () => {
  const tag = route.query?.tag as string // url parameter
  return tag ? [tag] : []
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
const isValidated = computed(() => {
  if (!formBody.value)  return false // body は必須
  return true
})

// 保存処理
const tagNamesRef = ref<typeof ArrayTagForm>()
const onSave = async () => {
  // バリデーションを通るか確認
  if (!isValidated.value) {
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

    message.success(`レポートを保存しました(${newReport.id})`)
    emit('save:after', newReport)
    resetForm()
  } catch (err) {
    console.log(err)
    message.error('内部エラーが発生しました')
  }
}
</script>
