<template>
  <TagTable :tags="tags" @onEdit="onEdit"></TagTable>

  <TagDialog
    v-model:show="showTagModal"
    :tag="selectedTag"
    @onChanged="onUpdated"
  ></TagDialog>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { Tag, useTagAPI } from '../composables/useTagAPI'

const tagAPI = useTagAPI()
const tags = ref<Tag[]>([])

const fetchTags = async () => {
  const data = await tagAPI.getAll()
  return data
}

onMounted(async () => {
  const data = await fetchTags()
  tags.value = data
})

///

const showTagModal = ref(false)
const selectedTag = ref<Tag>()
const onEdit = (tag: Tag) => {
  selectedTag.value = tag
  showTagModal.value = true
}


// 保持リスト更新処理
// const onCreated = (report: ReportWithTag) => {
//   reports.value.unshift(report)
// }
const onUpdated = (tag: Tag) => {
  const index = tags.value.findIndex((tag) => tag.id === tag.id)
  if (index >= 0) {
    tags.value.splice(index, 1, tag)
  } else {
    // 無いはず
    tags.value.unshift(tag)
  }
}
</script>
