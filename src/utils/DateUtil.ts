import { parse, subMinutes, format, formatDistanceToNow } from 'date-fns'
import ja from 'date-fns/locale/ja'

export const parseLocal = (datetime: string) => {
  const utcDate = parse(datetime, 'yyyy-MM-dd HH:mm:ss', new Date())
  const localDate = subMinutes(utcDate, utcDate.getTimezoneOffset())
  return localDate
}

export const formatString = (date: Date) => {
  return format(date, 'yyyy-MM-dd HH:mm:ss')
}

export const formatDistance = (date: Date) => {
  return formatDistanceToNow(date, { addSuffix: true, locale: ja })
}
