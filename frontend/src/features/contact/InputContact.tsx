"use client";

import { useForm } from "react-hook-form";

import type { FormValues } from ".";

import { BackButton } from "@/components/ui-parts/BackButton";

type Props = {
  onNextPage: (values: FormValues) => void;
};

const TITLES = {
  BUG: "bug",
} as const;

export const InputContact = ({ onNextPage }: Props) => {
  const { register, handleSubmit } = useForm<FormValues>();
  const onSubmit = (data: FormValues) => {
    onNextPage(data);
  };

  return (
    <form onSubmit={handleSubmit(onSubmit)}>
      <BackButton />
      <h1>お問い合わせ</h1>
      <label>
        <input type="text" {...register("name")} />
      </label>
      <label>
        <input type="text" {...register("furigana")} />
      </label>
      <label>
        <input type="text" {...register("email")} />
      </label>
      <label>
        <input type="text" {...register("phone")} />
      </label>
      <label>
        <select {...register("title")}>
          <option value={TITLES.BUG}>システムの不具合</option>
        </select>
      </label>
      <label>
        <textarea {...register("text")}></textarea>
      </label>
      <button type="submit">確認画面へ</button>
    </form>
  );
};
