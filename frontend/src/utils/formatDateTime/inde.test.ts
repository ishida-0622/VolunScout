import { formatDateTime } from "./index";

describe("formatDateTime", () => {
  it("日付日時が正しくフォーマットされるか", () => {
    const date = "2024-01-01T00:00:00Z";
    const formattedDate = formatDateTime(date);
    expect(formattedDate).toBe("2024/01/01 09:00");
  });

  it("UTCとJSTで日付が違う場合", () => {
    const date = "2024-01-01T20:00:00Z";
    const formattedDate = formatDateTime(date);
    expect(formattedDate).toBe("2024/01/02 05:00");
  });

  it("UTC以外のタイムゾーンの場合", () => {
    const date = "2024-01-01T20:00:00+09:00";
    const formattedDate = formatDateTime(date);
    expect(formattedDate).toBe("2024/01/01 20:00");
  });

  it("タイムゾーンが無い場合", () => {
    const date = "2024-01-01T00:00:00";
    const formattedDate = formatDateTime(date);
    expect(formattedDate).toBe("2024/01/01 09:00");

    const date2 = "2024-01-01T20:00:00";
    const formattedDate2 = formatDateTime(date2);
    expect(formattedDate2).toBe("2024/01/02 05:00");
  });
});
