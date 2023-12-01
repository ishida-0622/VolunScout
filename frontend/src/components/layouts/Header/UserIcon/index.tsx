"use client";

import Image from "next/image";
import Link from "next/link";
import { useState } from "react";

import styles from "./index.module.css";

import type { AccountType } from "@/features/auth/types";

import { URL_PATH, URL_PATH_GROUP, URL_PATH_PARTICIPANT } from "@/consts";
import { useUser } from "@/features/auth/hooks/useUser";
import { logout } from "@/features/auth/utils/logout";

type Props = {
  accountType: AccountType;
};

export const UserIcon = ({ accountType }: Props) => {
  const { user, isLoading } = useUser();

  const [isTooltipOpen, setIsTooltipOpen] = useState(false);
  const toggleTooltip = () => setIsTooltipOpen((prev) => !prev);

  const toMyPage = () => {
    if (accountType === "group") {
      return URL_PATH_GROUP.ACCOUNT;
    } else if (accountType === "participant") {
      return URL_PATH_PARTICIPANT.ACCOUNT;
    } else {
      throw new Error("accountType is invalid");
    }
  };

  if (isLoading) return null;

  return (
    <>
      <div>
        <Image
          src={user?.photoURL ?? "/icon.svg"}
          alt="user icon"
          // TODO: 画像のサイズを調整する
          width={100}
          height={100}
          onClick={toggleTooltip}
          className={styles.base}
        />
      </div>
      {isTooltipOpen && (
        <div className={styles.tooltip}>
          <ul>
            <li>
              <Link href={toMyPage()}>マイページ</Link>
            </li>
            <li>
              <Link href={URL_PATH.CONTACT}>お問い合わせ</Link>
            </li>
            <li>
              <Link href={URL_PATH.DONATE}>運営への寄付</Link>
            </li>
            <li>
              <span onClick={logout}>ログアウト</span>
            </li>
          </ul>
        </div>
      )}
    </>
  );
};
