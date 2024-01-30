"use client";

import Image from "next/image";
import { useState } from "react";

import { Confirmation } from "./Confirmation";
import styles from "./index.module.css"; // CSSモジュールのインポート
import { useInputForm } from "./useInputForm";

export const SignUpPage = () => {
  const [page, setPage] = useState<"input" | "confirm">("input");
  const toConfirm = () => setPage("confirm");
  const toInput = () => setPage("input");

  const { formValues, InputForm } = useInputForm({ onSubmit: toConfirm });

  return (
    <section className={styles.section}>
      <div>
        <div className={styles.user_icon}>
          {/* TODO:アイコンを差し替える */}
          <Image
            src={"/logo_color.png"}
            alt="user icon"
            width={200}
            height={150}
            className={styles.user_icon_image}
          />
        </div>
        {page === "input" && InputForm}
        {page === "confirm" && (
          <Confirmation values={formValues} onPrevPage={toInput} />
        )}
      </div>
    </section>
  );
};
