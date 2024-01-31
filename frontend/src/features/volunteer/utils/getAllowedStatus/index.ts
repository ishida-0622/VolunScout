export const getAllowedStatus = (status: number) => {
  switch (status) {
    case 0:
      return "未認証";
    case 1:
      return "承認済み";
    case 2:
      return "棄却済み";
    default:
      throw new Error("invalid status");
  }
};
