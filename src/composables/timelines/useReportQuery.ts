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

  return {
    query,

    isSame,
  }
}

///

export type ReportQueryType = ReturnType<typeof useReportQuery>

export const injectKey = 'reportQuery'

///

// クエリをスペースで分割する
const _splitWords = (query: string) => {
  return query
    .replace('　', ' ')
    .split(' ')
    .filter((e) => e.trim().length > 0)
}
