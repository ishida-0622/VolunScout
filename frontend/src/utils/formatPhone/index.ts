/**
 * 電話番号をハイフン付きにフォーマットする
 *
 * - `0312345678` -> `03-1234-5678`
 * - `08012345678` -> `080-1234-5678`
 * @param phone ハイフンなしの電話番号
 * @returns ハイフンが入った電話番号
 */
export const formatPhone = (phone: string) => {
  const reg = /^[0-9]+$/;
  if (!reg.test(phone)) {
    return phone;
  }

  switch (phone.length) {
    case 10:
      return `${phone.slice(0, 2)}-${phone.slice(2, 6)}-${phone.slice(6, 10)}`;
    case 11:
      return `${phone.slice(0, 3)}-${phone.slice(3, 7)}-${phone.slice(7, 11)}`;
    default:
      return phone;
  }
};
