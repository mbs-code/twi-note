import { invoke } from '@tauri-apps/api/tauri'

export type Tag = {
  id: number
  name: string
  color?: string
  isPinned: number
  priority: number
  created_at: string
  updated_at: string
}

export type FormTag = {
  name?: string
  color?: string
  isPinned?: boolean
  priority?: number
}

// export type SearchReport = {
//   page?: number
//   count?: number
//   latest?: boolean
// }

export const useTagAPI = () => {
  const getAll = async () => {
    const tags: Tag[] = (await invoke('tag_get_all')) as []
    return tags
  }

  const create = async (form: FormTag) => {
    const tag: Tag = await invoke('tag_create', {
      name: form.name ?? '',
      color: form.color ?? null,
      isPinned: form.isPinned ?? false,
      priority: form.priority ?? 0,
    })
    return tag
  }

  const update = async (id: number, form: FormTag) => {
    const tag: Tag = await invoke('tag_update', {
      id: id,
      name: form.name ?? '',
      color: form.color ?? null,
      isPinned: form.isPinned ?? false,
      priority: form.priority ?? 0,
    })
    return tag
  }

  // const remove = async (id: number) => {
  //   const report_id: number = await invoke('report_remove', { id })
  //   return report_id
  // }

  return {
    getAll,
    create,
    update,
    // remove,
  }
}
