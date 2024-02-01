"use client";

import { SubmitButton } from "../SubmitButton";

import styles from "./index.module.css";

import type { DeleteGroupAccountRequestBody } from "@/__generated__/command";

import { apiClientGroup } from "@/api/command";
import { joinClassnames } from "@/components/@joinClassnames";
import { BackButton } from "@/components/ui-parts/BackButton";
import { useAuthContext } from "@/contexts/AuthContext";
import { useLogout } from "@/features/auth/hooks/useLogout";

export const AccountDeletePage = () => {
  const { logout } = useLogout();
  const { user } = useAuthContext();

  const handleOnClick = async () => {
    const gid = user?.uid;
    if (gid === undefined) {
      throw new Error("gid is undefined");
    }
    const body: DeleteGroupAccountRequestBody = { gid };

    try {
      await apiClientGroup.deleteGroupAccount(body);
      logout();
    } catch (e) {
      console.error(e);
      alert("アカウント削除に失敗しました");
    }
  };

  return (
    <div className={styles.base}>
      <BackButton className={joinClassnames("btn btn-primary")} />
      <h1 className={styles.top}>退会（アカウントの削除）に関する確認</h1>
      <div className={styles.main_contents}>
        <p>
          アカウントを削除すると、これまでに登録した以下のような情報が全て削除され、
          アカウントの復元をすることができなくなります。
        </p>
        <ul>
          <li>団体情報・登録情報</li>
          <li>登録したボランティア活動</li>
          <li>参加者の履歴</li>
          <li>その他のアカウントに紐づいているデータ</li>
        </ul>
        <p>
          尚、現在活動予定のボランティアがある場合、退会することができません。
          また、この操作は取り消すことができません。
        </p>
        <p>退会してもよろしいですか？</p>
      </div>
      <div className={styles.button}>
        <SubmitButton
          onClick={handleOnClick}
          className={joinClassnames("btn btn-danger")}
        />
      </div>
    </div>
  );
};
