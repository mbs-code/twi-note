<template>
  <TagTable
    :tags="tags"
    @on-edit="onEdit"
  />

  <TagEditDialog
    v-model:show="showTagModal"
    :tag="selectedTag"
    @on-changed="onUpdated"
  />
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { Tag, useTagAPI } from '../composables/useTagAPI'

const tagAPI = useTagAPI()
const tags = ref<Tag[]>([])
const loadTags = async () => {
  const data = await tagAPI.getAll({
    hasPinned: false
  })
  tags.value = data
}

onMounted(async () => {
  await loadTags()
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
const onUpdated = (updTag: Tag) => {
  const index = tags.value.findIndex((tag) => tag.id === updTag.id)
  if (index >= 0) {
    tags.value.splice(index, 1, updTag)
  } else {
    // 無いはず
    tags.value.unshift(updTag)
  }
}
</script>
