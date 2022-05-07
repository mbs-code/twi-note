<template>
  <PhraseTable
    :phrases="phrases"
    @edit="onEdit"
  />

  <PhraseEditDialog
    v-model:show="showPhraseDialog"
    text=""
    :phrases="selectedPhrase"
    @change="onUpdated"
  />
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { Phrase, usePhraseAPI } from '../composables/usePhraseAPI'

const phraseAPI = usePhraseAPI()
const phrases = ref<Phrase[]>([])
const loadPhrases = async () => {
  const data = await phraseAPI.getAll()
  phrases.value = data
}

onMounted(async () => await loadPhrases())

///

const showPhraseDialog = ref(false)
const selectedPhrase = ref<Phrase>()
const onEdit = (phrase: Phrase) => {
  selectedPhrase.value = phrase
  showPhraseDialog.value = true
}

// 保持リスト更新処理
// const onCreated = (report: ReportWithTag) => {
//   reports.value.unshift(report)
// }
const onUpdated = (updPhrase: Phrase) => {
  const index = phrases.value.findIndex((phrase) => phrase.id === updPhrase.id)
  if (index >= 0) {
    phrases.value.splice(index, 1, updPhrase)
  } else {
    // 無いはず
    phrases.value.unshift(updPhrase)
  }
}
</script>
