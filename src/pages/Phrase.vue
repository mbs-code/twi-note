<template>
  <PhraseTable
    :phrases="phrases"
    @edit="onEdit"
  />

  <PhraseEditDialog
    v-model:show="showPhraseDialog"
    text=""
    :phrase="selectedPhrase"
    @save:after="listAdd"
    @delete:after="listRemove"
  />
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { Phrase, usePhraseAPI } from '../composables/usePhraseAPI'

const phraseAPI = usePhraseAPI()

const phrases = ref<Phrase[]>([])
const fetchPhrases = async () => {
  const data = await phraseAPI.getAll()
  phrases.value = data
}

onMounted(async () => await fetchPhrases())

///

const showPhraseDialog = ref(false)
const selectedPhrase = ref<Phrase>()
const onEdit = (phrase: Phrase) => {
  selectedPhrase.value = phrase
  showPhraseDialog.value = true
}

// 保持リスト更新処理
const listAdd = (updPhrase: Phrase) => {
  const index = phrases.value.findIndex((phrase) => phrase.id === updPhrase.id)
  if (index >= 0) {
    phrases.value.splice(index, 1, updPhrase)
  } else {
    phrases.value.unshift(updPhrase)
  }
}
const listRemove = (delPhrase: Phrase) => {
  const index = phrases.value.findIndex((phrase) => phrase.id === delPhrase.id)
  if (index >= 0) {
    phrases.value.splice(index, 1)
  }
}
</script>
