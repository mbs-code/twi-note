<template>
  <n-space
    vertical
    style="padding: 8px 0 8px 8px;"
  >
    <n-el
      v-for="(phrase, _) of phrases"
      :key="`a${_}`"
      class="phrase-btn"
      :class="{ 'phrase-btn-active': isActive(phrase) }"
      type="primary"
    >
      <n-button
        style="padding: 0px;"
        @click="onSelect(phrase)"
      >
        <n-avatar
          :class="{ 'avatar-block': expand }"
          :color="phrase.color || 'gray'"
        >
          {{ phrase.text.substring(0, expand ? 12 : 3) }}
        </n-avatar>
      </n-button>
    </n-el>
  </n-space>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { listen } from '@tauri-apps/api/event'
import { Phrase, usePhraseAPI } from '../../composables/usePhraseAPI'

defineProps<{ expand?: boolean }>()

const router = useRouter()
const route = useRoute()

/// ////////////////////////////////////////////////////////////

// タグリスト取得
const phraseAPI = usePhraseAPI()
const phrases = ref<Phrase[]>([])
const fetchPhrase = async () => {
  const data = await phraseAPI.getAll()
  phrases.value = data
}

onMounted(async () => {
  await fetchPhrase()

  // tauri event listener
  await listen('phrase-changed', async () => {
    await fetchPhrase()
  })
})

const onSelect = (phrase: Phrase) => {
  const text = route.query?.phrase as string // url parameter
  if (text === phrase.text) {
    router.push({ name: 'timeline' })
  } else {
    router.push({ name: 'timeline', query: { phrase: phrase.text } })
  }
}

// アクティブタグを判定
const isActive = (phrase: Phrase) => {
  const text = route.query?.phrase as string // url parameter
  return text === phrase.text
}
</script>

<style scoped lang="scss">
.avatar-block {
  width: 140px;

  :v-deep(.n-avatar__text) {
    scale: 1;
  }
}

.phrase-btn {
  display: flex;

 &.phrase-btn-active {
    border-right: 4px solid var(--primary-color);
  }
}
</style>
