import { useRouter } from "next/navigation";
import { useRef } from "react";

import { TermsOfUseAndPrivacyPolicyModal } from "../../TermsOfUseAndPrivacyPolicyModal";

import styles from "./index.module.css"; // CSSモジュールのインポート

import type { CreateGroupAccountRequestBody } from "@/__generated__/command";
import type { FormValues } from "../useInputForm";

import { apiClientGroup } from "@/api/command";
import { joinClassnames } from "@/components/@joinClassnames";
import { CheckBox } from "@/components/ui-parts/CheckBox";
import { URL_PATH_GROUP } from "@/consts";
import { useAuthContext } from "@/contexts/AuthContext";

type Props = {
  values: FormValues;
  onPrevPage: () => void;
};

export const Confirmation = ({ values, onPrevPage }: Props) => {
  const router = useRouter();
  const { user } = useAuthContext();

  const isAgreed = useRef(false);

  const handleAgreed = (value: boolean) => {
    isAgreed.current = value;
  };

  const handleSubmit = async () => {
    if (!isAgreed.current) {
      alert("利用規約とプライバシーポリシーに同意してください");
      return;
    }
    const uid = user?.uid;
    if (!uid) {
      throw new Error("uid is null");
    }
    const body: CreateGroupAccountRequestBody = {
      ...values,
      gid: uid,
      representative_furigana: values.representativeName,
      representative_name: values.representativeFurigana,
    };

    try {
      await apiClientGroup.createGroupAccount(body);
      router.push(URL_PATH_GROUP.HOME);
    } catch (e) {
      console.error(e);
      alert("アカウント作成に失敗しました");
    }
  };

  return (
    <section>
      <div>
        <button onClick={onPrevPage} className={joinClassnames("btn btn-secondary")}>
          戻る
        </button>
        <div className={styles.main_contents}>
          <h1 className={styles.h1}>入力情報確認</h1>
          <div>
            <p className={styles.furigana}>{values.furigana}</p>
            <h1 className={styles.name}>{values.name}</h1>
            <p className={styles.address}>{values.address}</p>
            <p className={styles.phone}>{values.phone}</p>
          </div>
          <div>
            <p>{values.contents}</p>
          </div>
          <label>
            <CheckBox label="" onChange={handleAgreed}>
              <TermsOfUseAndPrivacyPolicyModal />
            </CheckBox>
          </label>
          <button
            onClick={handleSubmit}
            className={joinClassnames("btn btn-primary")}
          >
            会員登録する
          </button>
        </div>
      </div>
    </section>
  );
};
