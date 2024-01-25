import "@testing-library/jest-dom";
import { fireEvent, render } from "@testing-library/react";

import { SearchBar } from "./index";

describe("SearchBar", () => {
  it("検索入力が表示されること", () => {
    const { getByRole } = render(<SearchBar />);
    const searchInput = getByRole("textbox");

    expect(searchInput).toBeInTheDocument();
  });

  it("検索入力が変更された時にonChangeが呼ばれること", () => {
    const onChangeMock = jest.fn();
    const { getByRole } = render(<SearchBar onChange={onChangeMock} />);
    const searchInput = getByRole("textbox");

    fireEvent.change(searchInput, { target: { value: "テスト" } });

    expect(onChangeMock).toHaveBeenCalledWith("テスト");
  });

  it("検索入力に初期値が設定されること", () => {
    const { getByRole } = render(<SearchBar initValue="テスト" />);
    const searchInput = getByRole("textbox");

    expect(searchInput).toHaveValue("テスト");
  });
});
