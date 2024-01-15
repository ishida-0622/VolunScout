"use client";

import Image from "next/image";
import { useForm, type SubmitHandler } from "react-hook-form";
import type { FormValues } from "..";

import styles from "./index.module.css"; // CSSモジュールのインポート

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
    <section className={styles.section}>
      <div className={styles.userIcon}>
        <Image
          src="/icon.svg"
          alt="User Icon"
          width={100}
          height={100}
          className={styles.user_icon_image}
        />
      </div>
      <div className={styles.main_contents}>
        <h2 className={styles.h2}>新規会員登録</h2>
        <form className={styles.form} onSubmit={handleSubmit(onSubmit)}>
          <label className={styles.label}>
            <span className={styles.required}>※</span>
            <span className={styles.name}>氏名</span>
            <span className={styles.colon}>：</span>
            <input
              type="text"
              placeholder="山田 太郎"
              {...register("name", { required: true, maxLength: 80 })}
              className={styles.input}
            />
          </label>
          <label className={styles.label}>
            <span className={styles.required}>※</span>
            <span className={styles.furigana}>フリガナ</span>
            <span className={styles.colon}>：</span>
            <input
              type="text"
              placeholder="ヤマダ タロウ"
              {...register("furigana", { required: true })}
              className={styles.input}
            />
          </label>
          <label className={styles.label}>
            <span className={styles.required}>※</span>
            <span className={styles.birthday}>生年月日</span>
            <span className={styles.colon}>：</span>
            <input
              type="date"
              placeholder="birthday"
              {...register("birthday", { required: true })}
              className={styles.input}
            />
          </label>
          <label className={styles.label}>
            <span className={styles.required}>※</span>
            <span className={styles.phone}>電話番号</span>
            <span className={styles.colon}>：</span>
            <input
              type="tel"
              placeholder="080xxxxxxxx"
              {...register("phone", {
                required: true,
                maxLength: 11,
                minLength: 10,
              })}
              className={styles.input}
            />
          </label>
          <label className={styles.label}>
            <span className={styles.required}>※</span>
            <span className={styles.gender}>性別</span>
            <span className={styles.colon}>：</span>
            <select
              {...register("gender", { required: true })}
              className={styles.input}
            >
              <option value={0}>男性</option>
              <option value={1}>女性</option>
              <option value={2}>その他</option>
            </select>
          </label>

          <button type="submit" className={styles.button}>
            ボランティア希望の入力へ ➤
          </button>
        </form>
      </div>
    </section>
  );
};
