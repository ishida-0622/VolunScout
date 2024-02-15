import { parseDateTime } from "./index";

describe("parseDateTime", () => {
  it("正しくパースされるか", () => {
    const dateTime = "2022-01-01T12:00:00Z";
    const parsedDateTime = parseDateTime(dateTime);
    expect(parsedDateTime).toEqual(new Date("2022-01-01T12:00:00Z"));
  });

  it("タイムゾーンが指定されていない場合, UTC としてパースされるか", () => {
    const dateTime = "2022-01-01T12:00:00";
    const parsedDateTime = parseDateTime(dateTime);
    expect(parsedDateTime).toEqual(new Date("2022-01-01T12:00:00Z"));
  });

  it("タイムゾーンが指定されている場合", () => {
    const dateTime = "2022-01-01T12:00:00+09:00";
    const parsedDateTime = parseDateTime(dateTime);
    expect(parsedDateTime).toEqual(new Date("2022-01-01T03:00:00Z"));
  });
});
