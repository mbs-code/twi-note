import { invoke } from '@tauri-apps/api/tauri'

export type TimestampMode = 'relative' | 'absolute'

export type AppConfig = {
  is_dark: boolean
  expand_side: boolean
  expand_editor: boolean
  timestamp_mode: TimestampMode
}

export const useAppConfigAPI = () => {
  const load = async () => {
    const config: AppConfig = await invoke('load_config')
    return config
  }

  const save = async (config: AppConfig) => {
    const res: boolean = await invoke('save_config', {
      config,
    })
    return res
  }

  return {
    load,
    save,
  }
}
