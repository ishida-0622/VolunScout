"use client";

import { useRouter } from "next/navigation";

import styles from "./index.module.css";

import { joinClassnames } from "@/components/@joinClassnames";

type Props = {
  className?: string;
};

/**
 * 戻るボタン
 */
export const BackButton = ({ className }: Props) => {
  const router = useRouter();
  const handleOnClick = () => router.back();

  return (
    <button
      className={joinClassnames(styles.base, className)}
      onClick={handleOnClick}
    >
      戻る
    </button>
  );
};
