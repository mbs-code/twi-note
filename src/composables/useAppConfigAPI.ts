import { invoke } from '@tauri-apps/api/tauri'

export type TimestampMode = 'relative' | 'absolute'

export type AppConfig = {
  is_dark: boolean
  expand_side: boolean
  expand_editor: boolean

  timestamp_mode: TimestampMode
  use_updated_at: boolean
  hide_edited: boolean

  tl_once_count: number
  timezone_offset_sec: number
  start_of_day: string

  use_phrase: boolean
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
