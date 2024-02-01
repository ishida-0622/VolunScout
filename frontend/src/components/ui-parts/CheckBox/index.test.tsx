import "@testing-library/jest-dom";
import { fireEvent, render } from "@testing-library/react";

import { CheckBox } from "./index";

describe("CheckBox コンポーネント", () => {
  it("正しく描画されているか", () => {
    const { getByLabelText } = render(<CheckBox label="テスト" />);
    const checkbox = getByLabelText("テスト");

    expect(checkbox).toBeInTheDocument();
    expect(checkbox).not.toBeChecked();
  });

  it("クリック時にチェックが切り替わるか", () => {
    const { getByLabelText } = render(<CheckBox label="テスト" />);
    const checkbox = getByLabelText("テスト");

    fireEvent.click(checkbox);

    expect(checkbox).toBeChecked();
  });

  it("onChange ハンドラが正しく動作しているか", async () => {
    const onChangeMock = jest.fn();
    const { getByLabelText } = render(
      <CheckBox label="テスト" onChange={onChangeMock} />
    );
    const checkbox = getByLabelText("テスト");

    fireEvent.click(checkbox);

    // MEMO: トグル切り替えの際に, onChange を 0ms の setTimeout 内で呼び出しているため, 1ms 待つ
    const sleep = (ms: number) =>
      new Promise((resolve) => setTimeout(resolve, ms));
    await sleep(1);

    expect(onChangeMock).toHaveBeenCalledWith(true);
  });
});
