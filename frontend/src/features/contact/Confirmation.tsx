"use client";

import type { FormValues } from ".";

type Props = {
  values: FormValues;
  onPrevPage: () => void;
};

export const Confirmation = ({ values, onPrevPage }: Props) => {
  const handleOnClick = () => {
    // TODO: 開発者への送信処理
    // 恐らくLambda関数を呼び出すことになる
    console.log(values);
  };

  return (
    <div>
      <h1>お問い合わせ内容確認</h1>
      <button onClick={onPrevPage}>戻る</button>
      <section>
        <p>
          <span>氏名</span>
          <span>{values.name}</span>
        </p>
        <p>
          <span>フリガナ</span>
          <span>{values.furigana}</span>
        </p>
        <p>
          <span>メールアドレス</span>
          <span>{values.email}</span>
        </p>
        <p>
          <span>電話番号</span>
          <span>{values.phone}</span>
        </p>
        <p>
          <span>お問い合わせ種別</span>
          <span>{values.title}</span>
        </p>
      </section>
      <section>
        <h2>お問い合わせ内容</h2>
        <p>{values.text}</p>
      </section>
      <button onClick={handleOnClick}>送信</button>
    </div>
  );
};
