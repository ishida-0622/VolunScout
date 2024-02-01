import { stringToNumber } from "./index";

describe("stringToNumber", () => {
  it("正の整数", () => {
    const result = stringToNumber("123");
    expect(result).toBe(123);
  });

  it("負の整数", () => {
    const result = stringToNumber("-123");
    expect(result).toBe(-123);
  });

  it("正の小数", () => {
    const result = stringToNumber("123.45");
    expect(result).toBe(123.45);
  });

  it("負の小数", () => {
    const result = stringToNumber("-123.45");
    expect(result).toBe(-123.45);
  });

  it("0", () => {
    const result = stringToNumber("0");
    expect(result).toBe(0);
  });

  it("0.0", () => {
    const result = stringToNumber("0.0");
    expect(result).toBe(0);
  });

  it("数値でない場合", () => {
    expect(() => stringToNumber("abc")).toThrow();
  });

  it("空文字の場合", () => {
    expect(() => stringToNumber("")).toThrow();
  });
});
