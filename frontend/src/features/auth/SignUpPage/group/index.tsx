"use client";

import Image from "next/image";
import { useState } from "react";

import { Confirmation } from "./Confirmation";
import { useInputForm } from "./useInputForm";

export const SignUpPage = () => {
  const [page, setPage] = useState<"input" | "confirm">("input");
  const toConfirm = () => setPage("confirm");
  const toInput = () => setPage("input");

  const { formValues, InputForm } = useInputForm({ onSubmit: toConfirm });

  return (
    <div>
      <div>
        {/* TODO:アイコンを差し替える */}
        <Image src={"/icon.svg"} alt="user icon" width={100} height={100} />
      </div>
      {page === "input" && InputForm}
      {page === "confirm" && (
        <Confirmation values={formValues} onPrevPage={toInput} />
      )}
    </div>
  );
};
