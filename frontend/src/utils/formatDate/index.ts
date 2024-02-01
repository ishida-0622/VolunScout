/**
 * Date型の日付を文字列に変換する
 *
 * なお, ロケールはja-JPを使用するため, UTCとJSTで日付が違う場合は注意
 *
 * - `Date(2024-01-01T00:00:00Z)` -> `2024/1/1`
 * - `Date(2024-01-01T20:00:00Z)` -> `2024/1/2`
 *
 * @param date Date型の日付
 * @returns yyyy/MM/dd形式の文字列
 */
export const formatDate = (date: Date | string): string => {
  return (typeof date === "string" ? new Date(date) : date).toLocaleDateString(
    "ja-JP"
  );
};
