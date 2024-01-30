/**
 * Date型の日付時刻を文字列に変換する
 *
 * なお, ロケールはja-JPを使用するため, UTCとJSTで日付が違う場合は注意
 *
 * - `Date(2024-01-01T00:00:00Z)` -> `2024/1/1 09:00`
 * - `Date(2024-01-01T20:00:00Z)` -> `2024/1/2 05:00`
 *
 * @param date Date型もしくはパース前の日付時刻(string)
 * @returns yyyy/MM/dd hh:mm 形式の文字列
 */
export const formatDateTime = (date: Date | string): string => {
  if (typeof date === "string") {
    const reg = new RegExp(/Z|[+-](0\d|1[012])(:?[012345]\d)/);
    // MEMO: バックエンドからのレスポンスは UTC で返ってくるため, タイムゾーンが指定されていない場合はここで UTC と明示的に指定する
    if (!reg.test(date)) {
      // ESLint を黙らせる
      // eslint-disable-next-line no-param-reassign
      date += "Z";
    }
    // eslint-disable-next-line no-param-reassign
    date = new Date(date);
  }

  return new Intl.DateTimeFormat("ja-JP", {
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
    timeZone: "Asia/Tokyo",
  }).format(date);
};
