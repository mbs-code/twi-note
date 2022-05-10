import { ref } from 'vue'

export const useReportSearchOld = () => {
  const text = ref<string>('')
  const after = ref<string | null>()
  const before = ref<string | null>()
  const selectedTags = ref<string[]>([])

  /// ////////////////////////////////////////////////////////////

  const clear = () => {
    text.value = ''
    after.value = null
    before.value = null
    selectedTags.value = []
    console.log('clear')
  }

  const bind = (searchText: string) => {
    clear()

    const words = searchText.replaceAll('　', ' ').split(' ')
    for (const word of words) {
      if (word.startsWith('after:')) {
        before.value = word.replace('after:', '')
      } else if (word.startsWith('before:')) {
        after.value = word.replace('before:', '')
      } else if (word.startsWith('tag:')) {
        selectedTags.value.push(word.replace('tag:', ''))
      } else {
        text.value = [text.value.trim(), word].join(' ')
      }
    }
  }

  const generate = () => {
    const tags = selectedTags.value.map((name) => `tag:${name}`)

    const words = [text.value.trim(), ...tags]

    if (after.value) words.push(`after:${after.value}`)
    if (before.value) words.push(`before:${before.value}`)

    return words.join(' ')
  }

  return {
    text,
    after,
    before,
    selectedTags,

    clear,
    bind,
    generate,
  }
}
