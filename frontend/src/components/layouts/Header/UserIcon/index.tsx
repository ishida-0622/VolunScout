"use client";

import Image from "next/image";
import { useRouter } from "next/navigation";

import styles from "./index.module.css";

import type { AccountType } from "@/features/auth/types";

import { URL_PATH_GROUP, URL_PATH_PARTICIPANT } from "@/consts";
import { useUser } from "@/features/auth/hooks/useUser";

type Props = {
  accountType: AccountType;
};

export const UserIcon = ({ accountType }: Props) => {
  const router = useRouter();
  const { user, isLoading } = useUser();

  const handleOnClick = () => {
    if (accountType === "group") {
      router.push(URL_PATH_GROUP.ACCOUNT);
    } else if (accountType === "participant") {
      router.push(URL_PATH_PARTICIPANT.ACCOUNT);
    }
  };

  if (isLoading) return null;

  return (
    <Image
      src={user?.photoURL ?? "/icon.svg"}
      alt="user icon"
      // TODO: 画像のサイズを調整する
      width={100}
      height={100}
      onClick={handleOnClick}
      className={styles.base}
    />
    // TODO: カーソルを合わせたら
    // ・マイページ
    // ・お問い合わせ
    // ・ログアウト
    // みたいなのが表示されるようにする
  );
};
