<template>
  <n-space
    vertical
    style="padding: 8px 0 8px 8px;"
  >
    <n-el
      v-for="(tag, _) of pinTags"
      :key="`a${_}`"
      class="tag-btn"
      :class="{ 'tag-btn-active': isActive(tag.name) }"
      type="primary"
    >
      <n-button
        style="padding: 0px;"
        @click="onSelectedTag(tag)"
      >
        <n-avatar
          :class="{ 'avatar-block': expand }"
          :color="tag.color || 'gray'"
        >
          {{ tag.name.substring(0, expand ? 12 : 3) }}
        </n-avatar>
      </n-button>
    </n-el>
  </n-space>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { Tag, useTagAPI } from '../../composables/useTagAPI'
import { listen } from '@tauri-apps/api/event'

defineProps<{ expand?: boolean }>()

// タグリスト取得
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
  await listen('tag-changed', async () => {
    await loadTags()
  })
})

const router = useRouter()
const onSelectedTag = (tag: Tag) => {
  const name = route.query?.tag as string // url parameter
  if (name === tag.name) {
    router.push({ name: 'timeline' })
  } else {
    router.push({ name: 'timeline', query: { tag: tag.name } })
  }
}

// アクティブタグを判定
const route = useRoute()
const isActive = (tagName: string) => {
  const name = route.query?.tag as string // url parameter
  return tagName === name
}
</script>

<style scoped lang="scss">
.avatar-block {
  width: 140px;

  :v-deep(.n-avatar__text) {
    scale: 1;
  }
}

.tag-btn {
  display: flex;

 &.tag-btn-active {
    border-right: 4px solid var(--primary-color);
  }
}
</style>
