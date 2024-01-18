import { useForm } from "react-hook-form";

import styles from "./index.module.css"; // CSSモジュールのインポート

export type FormValues = {
  name: string;
  furigana: string;
  representativeName: string;
  representativeFurigana: string;
  address: string;
  phone: string;
  contents: string;
};

type Props = {
  onSubmit?: (data: FormValues) => void;
};

const noop = () => {};

export const useInputForm = ({ onSubmit = noop }: Props) => {
  const { register, handleSubmit, getValues } = useForm<FormValues>({
    defaultValues: {
      name: "",
      furigana: "",
      representativeName: "",
      representativeFurigana: "",
      address: "",
      phone: "",
      contents: "",
    },
  });

  const formValues = getValues();

  const InputForm: JSX.Element = (
    <form onSubmit={handleSubmit(onSubmit)}>
      <div className={styles.h2}>
        <h1>新規会員登録</h1>
      </div>
      <div className={styles.main_contents}>
        <label className={styles.label}>
          <span className={styles.name}>団体名</span>
          <span className={styles.colon}>：</span>
          <input
            type="text"
            {...register("name", { required: true })}
            placeholder="NPO法人 VolunScout"
            className={styles.input}
          />
        </label>
        <label className={styles.label}>
          <span className={styles.furigana}>フリガナ</span>
          <span className={styles.colon}>：</span>
          <input
            type="text"
            {...register("furigana", { required: true })}
            placeholder="エヌピーオーホウジン ボランスカウト"
            className={styles.input}
          />
        </label>
        <label className={styles.label}>
          <span className={styles.address}>所在地</span>
          <span className={styles.colon}>：</span>
          <input
            type="text"
            {...register("address", { required: true })}
            placeholder="東京都千代田区神田神保町 1-50"
            className={styles.input}
          />
        </label>
        <label className={styles.label}>
          <span className={styles.phone}>電話番号</span>
          <span className={styles.colon}>：</span>
          <input
            type="text"
            {...register("phone", { required: true })}
            className={styles.input}
            placeholder="0312345678"
          />
        </label>
        <label className={styles.label}>
          <span className={styles.contents}>
            紹介メッセージ
            <br />
            （後から変更可能です）
          </span>
          <span className={styles.colon}>：</span>
          <textarea
            {...register("contents")}
            className={styles.textarea}
          ></textarea>
        </label>
        <button type="submit" className={styles.button}>
          入力情報の確認へ
        </button>
      </div>
    </form>
  );
  return { formValues, InputForm };
};
