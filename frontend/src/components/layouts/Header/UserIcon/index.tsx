"use client";

import Image from "next/image";
import Link from "next/link";
import { useState } from "react";
import { AiOutlineQuestionCircle } from "react-icons/ai";
import { HiOutlineCurrencyYen } from "react-icons/hi2";
import { IoDocumentTextOutline, IoLogOutOutline } from "react-icons/io5";

import styles from "./index.module.css";

import type { AccountType } from "@/features/auth/types";

import { IconConfig } from "@/components/layouts/IconConfig";
import { URL_PATH, URL_PATH_GROUP, URL_PATH_PARTICIPANT } from "@/consts";
import { useLogout } from "@/features/auth/hooks/useLogout";
import { useUser } from "@/features/auth/hooks/useUser";

type Props = {
  accountType: AccountType;
};

export const UserIcon = ({ accountType }: Props) => {
  const { user, isLoading } = useUser();
  const { logout } = useLogout();

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
    <IconConfig>
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
              <Link href={toMyPage()}>
                <IoDocumentTextOutline />
                マイページ
              </Link>
            </li>
            <li>
              <Link href={URL_PATH.CONTACT}>
                <AiOutlineQuestionCircle />
                お問い合わせ
              </Link>
            </li>
            <li>
              <Link href={URL_PATH.DONATE}>
                <HiOutlineCurrencyYen />
                運営への寄付
              </Link>
            </li>
            <li>
              <span onClick={logout} className={styles.logout}>
                <IoLogOutOutline />
                ログアウト
              </span>
            </li>
          </ul>
        </div>
      )}
    </IconConfig>
  );
};
