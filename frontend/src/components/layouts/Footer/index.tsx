"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import { Container } from "react-bootstrap";

import styles from "./index.module.css";

import type { AccountType } from "@/features/auth/types";

import { joinClassnames } from "@/components/@joinClassnames";
import { URL_PATH, URL_PATH_GROUP, URL_PATH_PARTICIPANT } from "@/consts";
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

  const homeHref =
    accountType === "group" ? URL_PATH_GROUP.HOME : URL_PATH_PARTICIPANT.HOME;

  return (
    <>
      <div style={{ minHeight: "12vh" }} />
      <footer
        className={joinClassnames(
          "fixed-bottom",
          accountType === "participant" ? styles.participant : styles.group,
          className
        )}
      >
        <Container className="d-flex align-items-center p-2">
          <div>
            <Link href={homeHref} className="text-decoration-none text-dark">
              <h1>VolunScout</h1>
            </Link>
          </div>
          <div className="w-75 d-flex justify-content-end">
            <Link href={URL_PATH.TERMS_OF_SERVICE} className="mx-4">
              利用規約
            </Link>
            <Link href={URL_PATH.PRIVACY_POLICY} className="mx-4">
              プライバシーポリシー
            </Link>
            <Link href={URL_PATH.CONTACT} className="mx-4">
              お問い合わせ
            </Link>
          </div>
        </Container>
      </footer>
    </>
  );
};
