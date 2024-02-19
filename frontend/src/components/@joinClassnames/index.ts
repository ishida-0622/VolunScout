/**
 * 複数の className を結合する
 * @param classNames className
 * @returns 結合後のclassName
 */
export const joinClassnames = (...classNames: (string | undefined)[]) =>
  classNames.filter(Boolean).join(" ");
