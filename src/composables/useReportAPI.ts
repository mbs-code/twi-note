import { invoke } from '@tauri-apps/api/tauri'
import { Tag } from './useTagAPI'

export type Report = {
  id: number
  title?: string
  body: string
  created_at: string
  updated_at: string
  deleted_at?: string
  tags: Tag[]
}

export type FormReport = {
  title?: string
  body: string
  tagNames: String[]
}

export type SearchReport = {
  tagName?: string
  page: number
  count: number
  latest: boolean
}

export const useReportAPI = () => {
  const getAll = async (search: SearchReport) => {
    const reports: Report[] = await invoke('report_get_all', search)
    return reports
  }

  const create = async (form: FormReport) => {
    const report: Report = await invoke('report_create', {
      params: form,
    })
    return report
  }

  const update = async (report_id: number, form: FormReport) => {
    const report: Report = await invoke('report_update', {
      report_id: report_id,
      params: form,
    })
    return report
  }

  const remove = async (report_id: number) => {
    const result: boolean = await invoke('report_remove', {
      report_id: report_id,
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
