"use client";

import styles from "./index.module.css";

import { joinClassnames } from "@/components/@joinClassnames";

type Props = {
  onClick: () => void;
  className?: string;
};

export const SubmitButton = ({ onClick, className }: Props) => {
  return (
    <button
      className={joinClassnames(styles.base, className)}
      onClick={onClick}
    >
      退会する
    </button>
  );
};
