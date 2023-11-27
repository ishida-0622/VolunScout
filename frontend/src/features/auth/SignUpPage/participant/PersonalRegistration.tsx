"use client";

import { useForm, type SubmitHandler } from "react-hook-form";

import type { FormValues } from ".";

type Personal = Pick<
  FormValues,
  "name" | "furigana" | "phone" | "birthday" | "gender"
>;

type Props = {
  onNextPage: (values: Partial<FormValues>) => void;
  values: Partial<FormValues>;
};

export const PersonalRegistration = ({ onNextPage, values }: Props) => {
  const { register, handleSubmit } = useForm<Personal>({
    defaultValues: values,
  });

  const onSubmit: SubmitHandler<Personal> = (data) => onNextPage(data);

  return (
    <section>
      <h2>新規会員登録</h2>
      <form onSubmit={handleSubmit(onSubmit)}>
        <label>
          氏名
          <input
            type="text"
            placeholder="name"
            {...register("name", { required: true, maxLength: 80 })}
          />
        </label>
        <label>
          フリガナ
          <input
            type="text"
            placeholder="furigana"
            {...register("furigana", { required: true })}
          />
        </label>
        <label>
          生年月日
          <input
            type="date"
            placeholder="birthday"
            {...register("birthday", { required: true })}
          />
        </label>
        <label>
          電話番号
          <input
            type="tel"
            placeholder="phone"
            {...register("phone", {
              required: true,
              maxLength: 11,
              minLength: 10,
            })}
          />
        </label>
        <label>
          性別
          <select {...register("gender", { required: true })}>
            <option value={0}>男性</option>
            <option value={1}>女性</option>
            <option value={2}>その他</option>
          </select>
        </label>

        <button type="submit">ボランティア希望の入力へ</button>
      </form>
    </section>
  );
};
