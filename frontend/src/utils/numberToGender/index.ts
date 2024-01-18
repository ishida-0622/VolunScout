export const numberToGender = (number: number): string => {
  switch (number) {
    case 0:
      return "男性";
    case 1:
      return "女性";
    case 2:
      return "その他";
    default:
      throw new Error("性別の値が不正です");
  }
};
