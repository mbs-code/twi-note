export type ReportQueryParams = {
  text?: string
  after?: string
  before?: string
  tags: string[]
}

export class ReportQueryUtil {
  /**
   * クエリ文字列を分解する.
   *
   * @param {string} query クエリ文字列
   * @returns {string[]} ワード配列
   */
  static splitWords = (query: string): string[] => {
    return (query ?? '')
      .replace('　', ' ')
      .split(' ')
      .filter((e) => e.trim().length > 0)
  }

  /**
   * クエリ文字列を解析する.
   * @param {string} query クエリ文字列
   * @returns {ReportQueryParams} パラメタ
   */
  static parse(query: string): ReportQueryParams {
    const params: ReportQueryParams = {
      tags: [],
    }

    const words = this.splitWords(query)
    for (const word of words) {
      if (word.startsWith('after:')) {
        params.before = word.replace('after:', '')
      } else if (word.startsWith('before:')) {
        params.after = word.replace('before:', '')
      } else if (word.startsWith('tag:')) {
        params.tags.push(word.replace('tag:', ''))
      } else {
        params.text = [params.text?.trim(), word].filter((e) => e).join(' ')
      }
    }

    return params
  }

  /**
   * クエリ文字列を生成する.
   *
   * @param {ReportQueryParams} パラメタ
   * @returns {string} クエリ文字列
   */
  static generate(params: ReportQueryParams): string {
    const words = []

    if (params.text) words.push(params.text.trim())
    if (params.text) words.push(...params.tags.map((name) => `tag:${name}`))
    if (params.after) words.push(`after:${params.after}`)
    if (params.before) words.push(`after:${params.before}`)

    return words.filter((e) => e).join(' ')
  }
}
