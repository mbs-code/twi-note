import { ref } from 'vue'

export const useReportQuery = () => {
  const query = ref<string>('')

  // 同等のクエリか判定する
  const isSame = (diff: string) => {
    const queryWords = _splitWords(query.value)
    const diffWords = _splitWords(diff)

    // TODO: 片側しか判定してないが良い？
    return diffWords.every((diffWord) => queryWords.includes(diffWord))
  }

  // クエリがあるか確認
  const hasQuery = () => {
    return query.value.length && query.value.length > 0
  }

  // クエリ中のタグ名配列を取得する
  const getTagWords = () => {
    const queryWords = _splitWords(query.value)
    return queryWords.filter((word) => word.startsWith('tag:')).map((word) => word.replace('tag:', ''))
  }

  ///

  // クエリを上書きする
  const setQuery = (text: string) => {
    query.value = text
    fireChangeEvent()
  }

  // クエリワードを追加 or 削除する
  const toggleWord = (word: string) => {
    const queryWords = _splitWords(query.value)

    // 既に含まれていたら削除する、それ以外は追加する
    const index = queryWords.indexOf(word)
    if (index >= 0) {
      queryWords.splice(index, 1)
    } else {
      queryWords.push(word)
    }

    query.value = queryWords.join(' ')
    fireChangeEvent()
  }

  ///

  const onChanges = ref<((query: string) => void)[]>([])

  // プログラム的に更新した際に実行する
  const fireChangeEvent = () => {
    for (const event of onChanges.value) {
      event(query.value)
    }
  }

  // 更新イベントリスナーを追加する
  const addChangeEvent = (event: (query: string) => void) => {
    onChanges.value.push(event)
  }

  return {
    query,
    addChangeEvent,

    isSame,
    hasQuery,
    getTagWords,

    setQuery,
    toggleWord,
  }
}

///

export type ReportQueryType = ReturnType<typeof useReportQuery>

export const injectKey = 'reportQuery'

///

// クエリをスペースで分割する
// TODO: deprecated
const _splitWords = (query: string) => {
  return query
    .replace('　', ' ')
    .split(' ')
    .filter((e) => e.trim().length > 0)
}
