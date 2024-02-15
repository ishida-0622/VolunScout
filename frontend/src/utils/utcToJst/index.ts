import { parseDateTime } from "../parseDateTime";

export const utcToJst = (dateTime: string): string => {
  const parsedDate = parseDateTime(dateTime);

  return new Intl.DateTimeFormat("ja-JP", {
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
    timeZone: "Asia/Tokyo",
  })
    .format(parsedDate)
    .replace(/\//g, "-");
};
