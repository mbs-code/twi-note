<template>
  <TagTable
    :tags="tags"
    @edit="onEdit"
  />

  <TagEditDialog
    v-model:show="showTagDialog"
    :tag="selectedTag"
    @save:after="listAdd"
  />
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { Tag, useTagAPI } from '../composables/useTagAPI'

const tagAPI = useTagAPI()

const tags = ref<Tag[]>([])
const fetchTags = async () => {
  const data = await tagAPI.getAll({
    hasPinned: false
  })
  tags.value = data
}

onMounted(async () => await fetchTags())

///

const showTagDialog = ref(false)
const selectedTag = ref<Tag>()
const onEdit = (tag: Tag) => {
  selectedTag.value = tag
  showTagDialog.value = true
}

// 保持リスト更新処理
const listAdd = (updTag: Tag) => {
  const index = tags.value.findIndex((tag) => tag.id === updTag.id)
  if (index >= 0) {
    tags.value.splice(index, 1, updTag)
  } else {
    // 無いはず
    tags.value.unshift(updTag)
  }
}
// const listRemove = (delTag: Tag) => {
//   const index = tags.value.findIndex((tag) => tag.id === delTag.id)
//   if (index >= 0) {
//     tags.value.splice(index, 1)
//   }
// }
</script>
