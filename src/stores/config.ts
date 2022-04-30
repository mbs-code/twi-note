import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useConfigStore = defineStore('config', () => {
  const is_dark = ref(false)
  const expand_side = ref(false)

  return {
    is_dark,
    expand_side,
  }
})
