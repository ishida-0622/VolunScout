import { dateTimeToGoogleCalendarFormat } from "./index";

describe("dateTimeToGoogleCalendarFormat", () => {
  it("UTCの日時を変換する", () => {
    const utcDate = new Date("2024-01-01T00:00:00Z");
    const utcDateString = "2024-01-01T00:00:00Z";
    const utcResult = dateTimeToGoogleCalendarFormat(utcDate);
    const utcResult2 = dateTimeToGoogleCalendarFormat(utcDateString);
    expect(utcResult).toBe("20240101T0900Z");
    expect(utcResult2).toBe("20240101T0900Z");
  });

  it("JSTの日時を変換する", () => {
    const jstDate = new Date("2024-01-01T20:00:00+09:00");
    const jstDateString = "2024-01-01T20:00:00+09:00";
    const jstResult = dateTimeToGoogleCalendarFormat(jstDate);
    const jstResult2 = dateTimeToGoogleCalendarFormat(jstDateString);
    expect(jstResult).toBe("20240101T2000Z");
    expect(jstResult2).toBe("20240101T2000Z");
  });
});
