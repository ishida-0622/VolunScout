"use client";

import { useRef } from "react";

import styles from "./index.module.css";

import type { FormValues } from "..";

import { joinClassnames } from "@/components/@joinClassnames";
import { useTermsForm } from "@/features/volunteer/useTermsForm";

type Volunteer = Pick<
  FormValues,
  | "profile"
  | "region"
  | "theme"
  | "required_theme"
  | "condition"
  | "required_condition"
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
  const { InputForm, getValues } = useTermsForm({});
  const profileRef = useRef<HTMLTextAreaElement>(null);

  const onSubmit = () => {
    const data: Volunteer = {
      ...getValues(),
      profile: profileRef.current?.value || "",
    };
    onNextPage(data);
  };
  const onSubmitPrev = () => {
    const data: Volunteer = {
      ...getValues(),
      profile: profileRef.current?.value || "",
    };
    onPrevPage(data);
  };

  return (
    <section>
      <form onSubmit={onSubmit}>
        <button
          type="button"
          className={joinClassnames("btn btn-secondary")}
          onClick={() => onSubmitPrev()}
        >
          戻る
        </button>
        <div className={styles.main_contents}>
          <h2 className={styles.h2}>自己紹介（後からでも設定可能です）</h2>
          <label>
            <textarea
              className={styles.textarea}
              defaultValue={values.profile}
            />
          </label>
          <h2 className={styles.h2}>活動希望条件（後からでも設定可能です）</h2>

          {InputForm}

          <div className={styles.container}>
            <button
              type="submit"
              className={joinClassnames("btn btn-success", styles.button)}
            >
              入力情報の確認へ
            </button>
          </div>
        </div>
      </form>
    </section>
  );
};
