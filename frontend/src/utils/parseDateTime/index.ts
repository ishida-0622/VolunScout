/**
 * バックエンドから取得した日時をパースする
 *
 * バックエンドからのレスポンスは UTC だがタイムゾーンが指定されていないため, タイムゾーンを考慮してパースする
 *
 * @param dateTime バックエンドから取得した日時
 */
export const parseDateTime = (dateTime: string) => {
  const reg = new RegExp(/Z|[+-](0\d|1[012])(:?[012345]\d)/);
  // MEMO: バックエンドからのレスポンスは UTC で返ってくるため, タイムゾーンが指定されていない場合はここで UTC と明示的に指定する
  if (!reg.test(dateTime)) {
    // eslint-disable-next-line no-param-reassign
    dateTime += "Z";
  }
  return new Date(dateTime);
};
