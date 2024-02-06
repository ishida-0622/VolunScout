import { formatPhone } from "./index";

describe("formatPhone", () => {
  it("電話番号がハイフン付きにフォーマットされる", () => {
    expect(formatPhone("0312345678")).toBe("03-1234-5678");
    expect(formatPhone("08012345678")).toBe("080-1234-5678");
  });

  it("既にハイフンが含まれている場合は同じ電話番号を返す", () => {
    expect(formatPhone("03-1234-5678")).toBe("03-1234-5678");
    expect(formatPhone("080-1234-5678")).toBe("080-1234-5678");
  });

  it("10桁でも11桁でもない場合は何もしない", () => {
    expect(formatPhone("123456789")).toBe("123456789");
    expect(formatPhone("123456789012")).toBe("123456789012");
  });

  it("入力が空の場合は空の文字列を返す", () => {
    expect(formatPhone("")).toBe("");
  });

  it("非数字の文字が含まれている場合は同じ電話番号を返す", () => {
    expect(formatPhone("03a123b456c78")).toBe("03a123b456c78");
  });
});
