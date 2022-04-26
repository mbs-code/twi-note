<template>
  <n-space vertical>
    <n-button v-for="tag of pinTags" class="p-0" @click="onSelectedTag(tag)">
      <n-avatar :color="tag.color">
        {{ tag.name.substring(0, 3) }}
      </n-avatar>
    </n-button>
  </n-space>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import { Tag, useTagAPI } from '../../composables/useTagAPI'

const tagAPI = useTagAPI()
const pinTags = ref<Tag[]>([])
onMounted(async () => {
  const data = await tagAPI.getAll({
    hasPinned: true,
  })
  pinTags.value = data
})

const router = useRouter()
const onSelectedTag = (tag: Tag) => {
  router.push(`/?tag=${tag.name}`)
}
</script>
