import { formatDate } from "./index";

describe("formatDate", () => {
  it("日付が正しくフォーマットされるか", () => {
    const date = "2024-01-01T00:00:00Z";
    const formattedDate = formatDate(date);
    expect(formattedDate).toBe("2024/01/01");
  });

  it("UTCとJSTで日付が違う場合", () => {
    const date = "2024-01-01T20:00:00Z";
    const formattedDate = formatDate(date);
    expect(formattedDate).toBe("2024/01/02");
  });

  it("日付のみの場合", () => {
    const date = "2024-01-01";
    const formattedDate = formatDate(date);
    expect(formattedDate).toBe("2024/01/01");
  });

  it("UTC以外のタイムゾーンの場合", () => {
    const date = "2024-01-01T20:00:00+09:00";
    const formattedDate = formatDate(date);
    expect(formattedDate).toBe("2024/01/01");
  });
});
