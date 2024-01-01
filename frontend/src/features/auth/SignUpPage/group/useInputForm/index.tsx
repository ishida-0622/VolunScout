import { useForm } from "react-hook-form";

export type FormValues = {
  name: string;
  furigana: string;
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
      address: "",
      phone: "",
      contents: "",
    },
  });

  const formValues = getValues();

  const InputForm: JSX.Element = (
    <form onSubmit={handleSubmit(onSubmit)}>
      <h1>新規会員登録</h1>
      <label>
        <span>団体名</span>
        <input
          type="text"
          {...register("name")}
          placeholder="NPO法人 VolunScout"
        />
      </label>
      <label>
        <span>フリガナ</span>
        <input
          type="text"
          {...register("furigana")}
          placeholder="エヌピーオーホウジン ボランスカウト"
        />
      </label>
      <label>
        <span>所在地</span>
        <input
          type="text"
          {...register("address")}
          placeholder="東京都千代田区神田神保町 1-50"
        />
      </label>
      <label>
        <span>電話番号</span>
        <input type="text" {...register("phone")} placeholder="0312345678" />
      </label>
      <label>
        <span>紹介メッセージ（後から変更可能です）</span>
        <textarea {...register("contents")}></textarea>
      </label>
      <button type="submit">入力情報の確認へ</button>
    </form>
  );
  return { formValues, InputForm };
};
