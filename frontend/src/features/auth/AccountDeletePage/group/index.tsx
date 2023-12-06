import { SubmitButton } from "../SubmitButton";

import styles from "./index.module.css";

import type { DeleteGroupAccountRequestBody } from "@/__generated__/command";

import { apiClientGroup } from "@/api/command";
import { BackButton } from "@/components/ui-parts/BackButton";
import { getUid } from "@/features/auth/utils/getUid";

export const AccountDeletePage = () => {
  const handleOnClick = async () => {
    const gid = await getUid();
    if (gid === undefined) {
      throw new Error("gid is undefined");
    }
    const body: DeleteGroupAccountRequestBody = { gid };
    await apiClientGroup.deleteGroupAccount(body);
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
      <SubmitButton onClick={handleOnClick} />
    </div>
  );
};