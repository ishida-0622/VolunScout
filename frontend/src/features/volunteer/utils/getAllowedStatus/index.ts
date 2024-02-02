import { ALLOWED_STATUS } from "@/consts/allowedStatus";

export const getAllowedStatus = (status: number) => {
  switch (status) {
    case ALLOWED_STATUS.PENDING:
      return "未承認";
    case ALLOWED_STATUS.ACCEPT:
      return "承認済み";
    case ALLOWED_STATUS.REJECT:
      return "棄却済み";
    default:
      throw new Error("invalid status");
  }
};
