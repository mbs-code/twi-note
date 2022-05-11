import { LoadAction } from '@ts-pro/vue-eternal-loading'
import { ref } from 'vue'
import { useConfigStore } from '../../stores/config'
import { Report, useReportAPI } from '../useReportAPI'
import { parse } from 'date-fns'

// NOTE: onMounted は onInfinite で処理される
// NOTE: 初回取得は行う必要が無い

type Events = {
  scrollTo?: (index: number) => void
}

export const useReportList = (events?: Events) => {
  const configStore = useConfigStore()
  const reportAPI = useReportAPI()

  const reports = ref<Report[]>([])
  const query = ref<string>('')
  const isInitial = ref<boolean>(false) // infinite load用
  const _page = ref<number>(0) // load 時に +1 する

  /// ////////////////////////////////////////////////////////////
  /// 設定系イベントバッファ

  const tlOnceCountBuffer = ref<number>(10)
  const refUpdatedAtBuffer = ref<boolean>(false)
  configStore.$subscribe((mutation, state) => {
    // 設定が変わった際に自動で再読み込みする
    if (tlOnceCountBuffer.value !== state.tl_once_count) {
      reload()
    }

    if (refUpdatedAtBuffer.value !== state.use_updated_at) {
      reload()
    }
  })

  /// ////////////////////////////////////////////////////////////
  /// データ取得系

  const _fetchReports = async () => {
    // 設定バッファ
    tlOnceCountBuffer.value = configStore.tl_once_count
    refUpdatedAtBuffer.value = configStore.use_updated_at

    // TZオフセット計算
    const date = parse(configStore.start_of_day, 'HH:mm', new Date())
    const offset = (date.getHours() * 60 + date.getMinutes()) * 60
    const offsetSec = configStore.timezone_offset_sec - offset

    // データ取得
    const items = await reportAPI.getAll({
      text: query.value || undefined,
      page: _page.value || 1,
      count: configStore.tl_once_count,
      latest: true,
      useUpdatedAt: configStore.use_updated_at,
      timezoneOffsetSec: offsetSec,
    })

    reports.value.push(...items)
  }

  const reload = () => {
    reports.value = []
    isInitial.value = true
    _page.value = 0
  }

  const onInfiniteLoad = async ({ loaded, noMore, error }: LoadAction) => {
    const beforeReportLength = reports.value.length
    _page.value++ // ページカウント

    try {
      await _fetchReports()

      const afterReportLength = reports.value.length
      if (beforeReportLength === afterReportLength) {
        noMore()
      } else {
        loaded()
      }
    } catch (err) {
      error()
    }
  }

  /// ////////////////////////////////////////////////////////////
  /// 配列更新機能

  const add = (add: Report) => {
    const index = reports.value.findIndex((report) => report.id === add.id)
    if (index >= 0) {
      // 同IDがあれば置き換える
      reports.value.splice(index, 1, add)
      events?.scrollTo?.(1)
    } else {
      // 該当が無かったら先頭に追加
      reports.value.unshift(add)
      events?.scrollTo?.(-1)
    }
  }

  const remove = (remove: Report) => {
    const index = reports.value.findIndex((report) => report.id === remove.id)
    if (index >= 0) {
      // 同IDがあれば削除する
      reports.value.splice(index, 1)
      events?.scrollTo?.(1)
    }
    return -1
  }

  return {
    reports,
    query,
    isInitial,
    onInfiniteLoad,

    reload,
    add,
    remove,
  }
}
