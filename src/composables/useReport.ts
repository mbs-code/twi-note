import { invoke } from '@tauri-apps/api/tauri'

export type Report = {
  id: number
  title?: string
  body?: string
  created_at: string
  updated_at: string
  deleted_at?: string
}

export const useReport = () => {
  const getAll = async () => {
    const reports: Report[] = (await invoke('report_get_all')) as []
    return reports
  }

  return {
    getAll,
  }
}
