import { parseDateTime } from "../parseDateTime";

/**
 * 日付時刻を文字列に変換する
 *
 * なお, ロケールはja-JPを使用するため, UTCとJSTで日付が違う場合は注意
 *
 * - `2024-01-01T00:00:00Z` -> `2024/01/01 09:00`
 * - `2024-01-01T20:00:00Z` -> `2024/01/02 05:00`
 *
 * @param date 日付時刻
 * @returns yyyy/MM/dd hh:mm 形式の文字列
 */
export const formatDateTime = (date: string): string => {
  const parsedDate = parseDateTime(date);

  return new Intl.DateTimeFormat("ja-JP", {
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
    timeZone: "Asia/Tokyo",
  }).format(parsedDate);
};
