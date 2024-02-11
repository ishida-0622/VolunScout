import { parseDateTime } from "../parseDateTime";

/**
 * 日付を文字列に変換する
 *
 * なお, ロケールはja-JPを使用するため, UTCとJSTで日付が違う場合は注意
 *
 * - `2024-01-01T00:00:00Z` -> `2024/1/1`
 * - `2024-01-01T20:00:00Z` -> `2024/1/2`
 *
 * @param date 日付
 * @returns yyyy/MM/dd形式の文字列
 */
export const formatDate = (date: string): string => {
  const parsedDate = parseDateTime(date);

  return new Intl.DateTimeFormat("ja-JP", {
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
    timeZone: "Asia/Tokyo",
  }).format(parsedDate);
};
