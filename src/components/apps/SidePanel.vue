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
        @click="onClickPhrease(phrase)"
      >
        <n-avatar
          :class="{ 'avatar-block': expand }"
          :color="phrase.color || 'gray'"
        >
          {{ expand ? expandDisplayName(phrase) : shortDisplayName(phrase) }}
        </n-avatar>
      </n-button>
    </n-el>
  </n-space>
</template>

<script setup lang="ts">
import { inject, onMounted, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { listen } from '@tauri-apps/api/event'
import { Phrase, usePhraseAPI } from '../../composables/usePhraseAPI'
import { ReportQueryType, injectKey } from '../../composables/timelines/useReportQuery'

defineProps<{ expand?: boolean }>()

const router = useRouter()
const route = useRoute()
const phraseAPI = usePhraseAPI()
const reportQuery = inject(injectKey) as ReportQueryType

/// ////////////////////////////////////////////////////////////

// タグリスト取得
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

///

const onClickPhrease = (phrase: Phrase) => {
  // 既に選択されていたら削除、それ以外は上書き
  reportQuery.query.value = reportQuery.isSame(phrase.text)
    ? ''
    : phrase.text

  // ページがTLでないなら遷移する
  if (route.name !== 'timeline') {
    router.push({ name: 'timeline' })
  }
}

///

// アクティブタグを判定
const isActive = (phrase: Phrase) => {
  return reportQuery.isSame(phrase.text)
}

// ショート名を表示
const shortDisplayName = (phrase: Phrase) => {
  const splitIdx = phrase.name.indexOf(':')
  return splitIdx >= 0
    ? phrase.name.substring(0, splitIdx)
    : phrase.name
}

// フル名を表示
const expandDisplayName = (phrase: Phrase) => {
  const splitIdx = phrase.name.indexOf(':')
  return splitIdx >= 0
    ? phrase.name.substring(splitIdx + 1)
    : phrase.name
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
