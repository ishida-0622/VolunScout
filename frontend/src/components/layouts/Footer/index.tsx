"use client";

import Image from "next/image";
import Link from "next/link";
import { usePathname } from "next/navigation";

import styles from "./index.module.css";

import type { AccountType } from "@/features/auth/types";

import { joinClassnames } from "@/components/@joinClassnames";
import { URL_PATH } from "@/consts";
import { getAccountTypeFromPath } from "@/features/auth/utils/getAccountType";

type Props = {
  className?: string;
};

/**
 * フッター
 */
export const Footer = ({ className }: Props) => {
  const pathname = usePathname();

  const accountType: AccountType = getAccountTypeFromPath(pathname);

  return (
    <footer
      className={joinClassnames(
        styles.base,
        accountType === "participant" ? styles.participant : styles.group,
        className,
      )}
    >
      <div className={styles.image}>
        <Image src={"/icon.svg"} alt="Icon" width={50} height={50} />
      </div>
      <div>
        <h1>VolunScout</h1>
      </div>
      <div>
        <Link href={URL_PATH.TERMS_OF_SERVICE}>利用規約</Link>
      </div>
      <div className={styles.privacy_link}>
        <Link href={URL_PATH.PRIVACY_POLICY}>プライバシーポリシー</Link>
      </div>
      <div>
        <Link href={URL_PATH.CONTACT}>お問い合わせ</Link>
      </div>
    </footer>
  );
};
