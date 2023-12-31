"use client";

import { SubmitButton } from "../SubmitButton";

import styles from "./index.module.css";

import type { DeleteParticipantAccountRequestBody } from "@/__generated__/command";

import { apiClientParticipant } from "@/api/command";
import { BackButton } from "@/components/ui-parts/BackButton";
import { useAuthContext } from "@/contexts/AuthContext";
import { useLogout } from "@/features/auth/hooks/useLogout";

export const AccountDeletePage = () => {
  const { logout } = useLogout();
  const { user } = useAuthContext();

  const handleOnClick = async () => {
    const pid = user?.uid;
    if (pid === undefined) {
      throw new Error("uid is undefined");
    }
    const body: DeleteParticipantAccountRequestBody = {
      pid,
    };

    try {
      await apiClientParticipant.deleteParticipantAccount(body);
      logout();
    } catch (e) {
      console.error(e);
      alert("アカウント削除に失敗しました");
    }
  };

  return (
    <div className={styles.base}>
      <BackButton />
      <h1>退会（アカウントの削除）に関する確認</h1>
      <p>
        アカウントを削除すると、これまでに登録した以下のような情報が全て削除され、
        アカウントの復元をすることができなくなります。
      </p>
      <ul>
        <li>個人情報・登録情報</li>
        <li>ボランティア活動・応募の履歴</li>
        <li>お気に入りに登録したボランティアの履歴</li>
        <li>その他のアカウントに紐づいているデータ</li>
      </ul>
      <p>
        尚、現在参加予定のボランティアがある場合、退会することができません。
        また、この操作は取り消すことができません。
      </p>
      <p>退会してもよろしいですか？</p>
      <SubmitButton onClick={handleOnClick} />
    </div>
  );
};
