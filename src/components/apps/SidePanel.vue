<template>
  <n-space vertical>
    <div>あ</div>
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
import { listen } from '@tauri-apps/api/event'

const tagAPI = useTagAPI()
const pinTags = ref<Tag[]>([])
const loadTags = async () => {
  const data = await tagAPI.getAll({
    hasPinned: true,
  })
  pinTags.value = data
}

onMounted(async () => {
  await loadTags()

  // tauri event listener
  await listen('tag-changed', async (e) => {
    await loadTags()
  })
})

const router = useRouter()
const onSelectedTag = (tag: Tag) => {
  router.push(`/?tag=${tag.name}`)
}

</script>
