import { numberToGender } from "./index";

describe("numberToGender", () => {
  it("0を渡すと「男性」を返す", () => {
    expect(numberToGender(0)).toBe("男性");
  });

  it("1を渡すと「女性」を返す", () => {
    expect(numberToGender(1)).toBe("女性");
  });

  it("2を渡すと「その他」を返す", () => {
    expect(numberToGender(2)).toBe("その他");
  });

  it("その他の数値を渡すとエラーを返す", () => {
    expect(() => numberToGender(3)).toThrow("性別の値が不正です");
    expect(() => numberToGender(-1)).toThrow("性別の値が不正です");
    expect(() => numberToGender(1.5)).toThrow("性別の値が不正です");
  });
});
