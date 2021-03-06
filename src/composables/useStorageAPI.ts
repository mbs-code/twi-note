import { invoke } from '@tauri-apps/api/tauri'

export type StorageInfo = {
  path: string
  exist: boolean
  size?: number
}

export const useStorageAPI = () => {
  const load = async () => {
    const storage: StorageInfo = await invoke('get_storage_info')
    return storage
  }

  const open = async () => {
    await invoke('open_directory')
  }

  return {
    load,
    open,
  }
}
