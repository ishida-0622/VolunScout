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

  // パスからアカウントの種類を取得
  const accountType: AccountType = getAccountTypeFromPath(pathname);

  // ホームページへのリンク
  const homeHref =
    accountType === "group" ? URL_PATH_GROUP.HOME : URL_PATH_PARTICIPANT.HOME;

  return (
    <>
      {/* フッター上の余白 */}
      <div style={{ minHeight: "8vh" }} />
      {/* フッター */}
      <footer
        className={joinClassnames(
          "fixed-bottom",
          accountType === "participant" ? styles.participant : styles.group,
          className
        )}
      >
        {/* コンテナ */}
        <Container className="d-flex align-items-center p-1">
          {/* VolunScoutへのリンク */}
          <div>
            <Link href={homeHref} className="text-decoration-none text-dark">
              <h2>VolunScout</h2>
            </Link>
          </div>
          {/* リンク一覧 */}
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
