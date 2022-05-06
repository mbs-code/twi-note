import { parse } from 'date-fns'
import { defineStore } from 'pinia'
import { computed, ref } from 'vue'
import { TimestampMode } from '../composables/useAppConfigAPI'

export const useConfigStore = defineStore('config', () => {
  /** ダークモード */
  const is_dark = ref<boolean>(false)
  /** 左サイドパネルの拡縮 */
  const expand_side = ref<boolean>(false)
  /** 下部エディタの拡縮 */
  const expand_editor = ref<boolean>(false)

  /** 時刻の表示方法 */
  const timestamp_mode = ref<TimestampMode>('relative')
  /** 更新時間を使用するか */
  const use_updated_at = ref<boolean>(false)
  /** TLで一度に取得する量 */
  const tl_once_count = ref<number>(20)

  /** タイムゾーン値 */
  const timezone_offset_sec = ref<number>(0)
  const timezone_offset_hour = computed({
    get: () => timezone_offset_sec.value / 60 / 60,
    set: (hour: number) => {
      timezone_offset_sec.value = hour * 60 * 60
    },
  })
  /** 一日の開始時刻 */
  const start_of_day = ref<string>('00:00')
  /** タイムゾーン + 開始時刻オフセット */
  const offset_sec = computed(() => {
    const date = parse(start_of_day.value, 'HH:mm', new Date())
    const offset = (date.getHours() * 60 + date.getMinutes()) * 60
    console.log('calc', date, offset)
    return timezone_offset_sec.value - offset
  })

  return {
    is_dark,
    expand_side,
    expand_editor,

    timestamp_mode,
    use_updated_at,
    tl_once_count,

    timezone_offset_sec,
    timezone_offset_hour,
    start_of_day,
    offset_sec,
  }
})
