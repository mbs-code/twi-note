import { invoke } from '@tauri-apps/api/tauri'

export type Tag = {
  id: number
  name: string
  color?: string
  is_pinned: number
  priority: number
  created_at: string
  updated_at: string
}

export type FormTag = {
  name: string
  color?: string
  hasPinned: boolean
  priority: number
}

export type SearchTag = {
  hasPinned?: boolean
}

export const useTagAPI = () => {
  const getAll = async (search: SearchTag) => {
    const tags: Tag[] = await invoke('tag_get_all', search)
    return tags
  }

  const create = async (form: FormTag) => {
    const tag: Tag = await invoke('tag_create', {
      params: form,
    })
    return tag
  }

  const update = async (tag_id: number, form: FormTag) => {
    const tag: Tag = await invoke('tag_update', {
      tag_id: tag_id,
      params: form,
    })
    return tag
  }

  const remove = async (tag_id: number) => {
    const result: boolean = await invoke('tag_remove', {
      tag_id: tag_id,
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
