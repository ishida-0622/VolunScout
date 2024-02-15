"use client";

import { useState } from "react";

import { Confirmation } from "./Confirmation";
import { InputContact } from "./InputContact";

export type FormValues = {
  name: string;
  furigana: string;
  email: string;
  phone: string;
  title: string;
  text: string;
};

export const ContactPage = () => {
  const [formValues, setFormValues] = useState<FormValues>({
    name: "",
    furigana: "",
    email: "",
    phone: "",
    title: "",
    text: "",
  });

  const [page, setPage] = useState<"input" | "confirm">("input");
  const toConfirm = () => setPage("confirm");
  const toInput = () => setPage("input");
  const handleNextPage = (values: FormValues) => {
    setFormValues(values);
    toConfirm();
  };
  const handlePrevPage = () => {
    toInput();
  };

  return (
    <div>
      {page === "input" && <InputContact onNextPage={handleNextPage} />}
      {page === "confirm" && (
        <Confirmation values={formValues} onPrevPage={handlePrevPage} />
      )}
    </div>
  );
};
