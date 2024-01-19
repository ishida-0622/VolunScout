"use client";

import { useState } from "react";

import { Confirmation } from "./Confirmation";
import { PersonalRegistration } from "./PersonalRegistration";
import { VolunteerRegistration } from "./VolunteerRegistration";

export type FormValues = {
  pid: string;
  name: string;
  furigana: string;
  phone: string;
  gender: string;
  birthday: string;
  profile: string;
  regions: string[];
  themes: string[];
  themesRequired: string[];
  conditions: string[];
  conditionsRequired: string[];
  targetStatuses: string;
};

export const SignUpPage = () => {
  const [formValues, setFormValues] = useState<FormValues>({
    pid: "",
    name: "",
    furigana: "",
    gender: "0",
    phone: "",
    birthday: "",
    profile: "",
    regions: [],
    themes: [],
    themesRequired: [],
    conditions: [],
    conditionsRequired: [],
    targetStatuses: "",
  });

  const [pageCounter, setPageCounter] = useState(0);
  const nextPage = () => setPageCounter((prev) => prev + 1);
  const prevPage = () => setPageCounter((prev) => prev - 1);

  const handleNextPage = (values: Partial<FormValues>) => {
    setFormValues((prev) => ({ ...prev, ...values }));
    nextPage();
  };

  const handlePrevPage = (values: Partial<FormValues>) => {
    setFormValues((prev) => ({ ...prev, ...values }));
    prevPage();
  };

  return (
    <main>
      {pageCounter === 0 && (
        <PersonalRegistration onNextPage={handleNextPage} values={formValues} />
      )}
      {pageCounter === 1 && (
        <VolunteerRegistration
          onNextPage={handleNextPage}
          onPrevPage={handlePrevPage}
          values={formValues}
        />
      )}
      {pageCounter === 2 && (
        <Confirmation values={formValues} prevPage={prevPage} />
      )}
    </main>
  );
};
