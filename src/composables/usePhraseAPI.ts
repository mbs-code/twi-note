import { invoke } from '@tauri-apps/api/tauri'

export type Phrase = {
  id: number
  text: string
  color?: string
  priority: number
  created_at: string
  updated_at: string
}

export type FormPhrase = {
  text: string
  color?: string
  priority: number
}

export const usePhraseAPI = () => {
  const getAll = async () => {
    const phrases: Phrase[] = await invoke('phrase_get_all')
    return phrases
  }

  const create = async (form: FormPhrase) => {
    const phrase: Phrase = await invoke('phrase_create', {
      params: form,
    })
    return phrase
  }

  const update = async (phraseId: number, form: FormPhrase) => {
    const phrase: Phrase = await invoke('phrase_update', {
      phraseId: phraseId,
      params: form,
    })
    return phrase
  }

  const remove = async (phraseId: number) => {
    const result: boolean = await invoke('phrase_remove', {
      tagId: phraseId,
    })
    return result
  }

  return {
    getAll,
    create,
    update,
    remove,
  }
}
