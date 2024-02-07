/**
 * Google Calender に追加するための日時フォーマットに変換する
 *
 * なお, ロケールはja-JPを使用するため, UTCとJSTで日付が違う場合は注意
 *
 * - `2024-01-01T00:00:00Z` -> `20240101T0900Z`
 * - `2024-01-01T20:00:00Z` -> `20240102T0500Z`
 * @param date 日時
 * @returns Google Calender に追加するための日時フォーマット
 */
export const dateTimeToGoogleCalendarFormat = (date: Date | string) => {
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
  return `${new Intl.DateTimeFormat("ja-JP", {
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
    timeZone: "Asia/Tokyo",
  })
    .format(date)
    .replace(/\//g, "")
    .replace(/:/g, "")
    .replace(" ", "T")}Z`;
};
