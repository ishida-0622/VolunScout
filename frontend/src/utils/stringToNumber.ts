/**
 * string型の数字をnumber型に変換する
 *
 * 変換できない場合はエラーを投げる
 * @param str 変換したい文字列
 * @returns 変換後の数値
 */
export const stringToNumber = (str: string) => {
  const reg = /^\d+$/;
  if (reg.test(str)) {
    return Number(str);
  } else {
    throw new Error("stringToNumber: string is not a number");
  }
};
