/**
 * 生年月日から年齢を計算する
 * @param date 生年月日
 * @returns 年齢
 */
export const ageCalc = (date: string | Date) => {
  const today = new Date();
  const birthDate = new Date(date);
  let age = today.getFullYear() - birthDate.getFullYear();
  const month = today.getMonth() - birthDate.getMonth();
  if (month < 0 || (month === 0 && today.getDate() < birthDate.getDate())) {
    age--;
  }
  return age;
};
