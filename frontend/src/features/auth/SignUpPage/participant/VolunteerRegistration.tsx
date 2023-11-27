"use client";

import { useForm, type SubmitHandler } from "react-hook-form";

import type { FormValues } from ".";

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
        <button type="button" onClick={handleSubmit(onSubmitPrev)}>
          戻る
        </button>
        <h2>自己紹介（後からでも設定可能です）</h2>
        <label>
          <textarea {...register("profile")} />
        </label>
        <h2>活動希望条件（後からでも設定可能です）</h2>

        <details>
          <summary>地域</summary>
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

        <details>
          <summary>テーマ</summary>
          {THEMES.map((theme) => (
            <div key={theme}>
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
              />
            </div>
          ))}
        </details>

        <details>
          <summary>条件</summary>
          {CONDITIONS.map((condition) => (
            <div key={condition}>
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
              />
            </div>
          ))}
        </details>

        <button type="submit">入力情報の確認へ</button>
      </form>
    </section>
  );
};
