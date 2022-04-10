import { invoke } from '@tauri-apps/api/tauri'

export type Report = {
  id: number
  title?: string
  body?: string
  created_at: string
  updated_at: string
  deleted_at?: string
}

export type FormReport = {
  title?: string
  body?: string
}

export type SearchReport = {
  page?: number
  count?: number
  latest?: boolean
}

export const useReportAPI = () => {
  const getAll = async (search: SearchReport = {}) => {
    const reports: Report[] = (await invoke('report_get_all', {
      page: search.page ?? 1,
      count: search.count ?? 20,
      latest: search.latest ?? false,
    })) as []
    return reports
  }

  const create = async (form: FormReport) => {
    const report: Report = await invoke('report_create', {
      title: form.title ?? null,
      body: form.body ?? null,
    })
    return report
  }

  const update = async (id: number, form: FormReport) => {
    const report: Report = await invoke('report_update', {
      id: id,
      title: form.title ?? null,
      body: form.body ?? null,
    })
    return report
  }

  const remove = async (id: number) => {
    const report: Report = await invoke('report_remove', { id })
    return report
  }

  return {
    getAll,
    create,
    update,
    remove,
  }
}
