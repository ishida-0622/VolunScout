"use client";

import { useEffect, useState } from "react";

import { Confirmation } from "./Confirmation";
import { PersonalRegistration } from "./PersonalRegistration";
import { VolunteerRegistration } from "./VolunteerRegistration";

import { auth } from "@/firebaseConfig";

export type FormValues = {
  pid: string;
  name: string;
  furigana: string;
  phone: string;
  gender: string;
  birthday: string;
  profile: string;
  region: string[];
  theme: string[];
  required_theme: string[];
  condition: string[];
  required_condition: string[];
  targetStatuses: string;
};

export const SignUpPage = () => {
  const [formValues, setFormValues] = useState<FormValues>({
    pid: "",
    name: "",
    furigana: "",
    gender: "",
    phone: "",
    birthday: "",
    profile: "",
    region: [],
    theme: [],
    required_theme: [],
    condition: [],
    required_condition: [],
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

  useEffect(() => {
    return () => {
      auth.signOut().catch(() => {});
    };
  }, []);

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
