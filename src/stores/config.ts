import { defineStore } from 'pinia'
import { ref } from 'vue'
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
  /** 編集済みを隠す */
  const hide_edited = ref<boolean>(false)

  /** タイムゾーン値 */
  const timezone_offset_sec = ref<number>(0)
  /** 一日の開始時刻 */
  const start_of_day = ref<string>('00:00')

  /** フレーズ保存機能を使用するか */
  const use_phrase = ref<boolean>(false)

  return {
    is_dark,
    expand_side,
    expand_editor,

    timestamp_mode,
    use_updated_at,
    tl_once_count,
    hide_edited,

    timezone_offset_sec,
    start_of_day,

    use_phrase,
  }
})
