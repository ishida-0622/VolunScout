"use client";

import { useEffect, useState } from "react";

import { Confirmation } from "./Confirmation";
import styles from "./index.module.css"; // CSSモジュールのインポート
import { useInputForm } from "./useInputForm";

import { auth } from "@/firebaseConfig";

export const SignUpPage = () => {
  const [page, setPage] = useState<"input" | "confirm">("input");
  const toConfirm = () => setPage("confirm");
  const toInput = () => setPage("input");

  const { formValues, InputForm } = useInputForm({ onSubmit: toConfirm });

  useEffect(() => {
    return () => {
      auth.signOut().catch(() => {});
    };
  }, []);

  return (
    <section className={styles.section}>
      <div>
        {page === "input" && InputForm}
        {page === "confirm" && (
          <Confirmation values={formValues} onPrevPage={toInput} />
        )}
      </div>
    </section>
  );
};
