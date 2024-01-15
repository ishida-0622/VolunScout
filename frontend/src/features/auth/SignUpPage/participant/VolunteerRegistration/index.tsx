"use client";

import { useForm, type SubmitHandler } from "react-hook-form";

import styles from "./index.module.css"; // CSSモジュールのインポート

import type { FormValues } from "..";

import { CheckBoxControl } from "@/components/ui-parts/CheckBoxControl";
import { ToggleSwitchControl } from "@/components/ui-parts/ToggleSwitchControl";
import { CONDITIONS, REGIONS, THEMES } from "@/consts";

type Volunteer = Pick<
  FormValues,
  | "profile"
  | "regions"
  | "themes"
  | "themesRequired"
  | "conditions"
  | "conditionsRequired"
>;

type Props = {
  onNextPage: (values: Partial<FormValues>) => void;
  onPrevPage: (values: Partial<FormValues>) => void;
  values: Partial<FormValues>;
};

export const VolunteerRegistration = ({
  onNextPage,
  onPrevPage,
  values,
}: Props) => {
  const { register, handleSubmit } = useForm<Volunteer>({
    defaultValues: values,
  });

  const onSubmit: SubmitHandler<Volunteer> = (data) => onNextPage(data);
  const onSubmitPrev: SubmitHandler<Volunteer> = (data) => onPrevPage(data);

  return (
    <section>
      <form onSubmit={handleSubmit(onSubmit)}>
        <button
          type="button"
          className={styles.return}
          onClick={handleSubmit(onSubmitPrev)}
        >
          戻る
        </button>
        <div className={styles.main_contents}>
          <h2 className={styles.h2}>自己紹介（後からでも設定可能です）</h2>
          <label>
            <textarea className={styles.textarea} {...register("profile")} />
          </label>
          <h2 className={styles.h2}>活動希望条件（後からでも設定可能です）</h2>

          <div className={styles.select}>
            <details open>
              <summary className={styles.main}>地域</summary>
              {REGIONS.map((region) => (
                <div key={region}>
                  <label>
                    <input
                      type="checkbox"
                      value={region}
                      {...register("regions")}
                    />
                    {region}
                  </label>
                </div>
              ))}
            </details>

            <details open>
              <summary className={styles.main}>テーマ</summary>
              {THEMES.map((theme) => (
                // TODO
                <div key={theme} style={{ display: "flex" }}>
                  <CheckBoxControl
                    name="themes"
                    value={theme}
                    register={register}
                    label={theme}
                  />
                  <ToggleSwitchControl
                    name="themesRequired"
                    value={theme}
                    register={register}
                    label=""
                    // TODO
                    className={styles.toggle}
                  />
                </div>
              ))}
            </details>

            <details open>
              <summary className={styles.main}>条件</summary>
              {CONDITIONS.map((condition) => (
                <div key={condition} style={{ display: "flex" }}>
                  <CheckBoxControl
                    name="conditions"
                    value={condition}
                    register={register}
                    label={condition}
                  />
                  <ToggleSwitchControl
                    name="conditionsRequired"
                    value={condition}
                    register={register}
                    label=""
                    className={styles.toggle}
                  />
                </div>
              ))}
            </details>
          </div>
          <div className={styles.container}>
            <button type="submit" className={styles.button}>
              入力情報の確認へ
            </button>
          </div>
        </div>
      </form>
    </section>
  );
};
