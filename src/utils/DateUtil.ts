import { parse, subMinutes, formatDistanceToNow } from 'date-fns'

export const parseUtc = (date: string) => {
  const utcDate = parse(date, 'yyyy-MM-dd HH:mm:ss', new Date())
  const localDate = subMinutes(utcDate, utcDate.getTimezoneOffset())
  return localDate
}
