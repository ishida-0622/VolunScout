import { useRouter } from "next/navigation";
import { useRef } from "react";

import { TermsOfUseAndPrivacyPolicyModal } from "../../TermsOfUseAndPrivacyPolicyModal";

import type { CreateGroupAccountRequestBody } from "@/__generated__/command";
import type { FormValues } from "../useInputForm";

import { apiClientGroup } from "@/api/command";
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
    <div>
      <h1>入力情報確認</h1>
      <button onClick={onPrevPage}>戻る</button>
      <div>
        <span>{values.furigana}</span>
        <h1>{values.name}</h1>
        <p>{values.address}</p>
        <p>{values.phone}</p>
      </div>
      <div>
        <p>{values.contents}</p>
      </div>
      <label>
        <CheckBox label="" onChange={handleAgreed}>
          <TermsOfUseAndPrivacyPolicyModal />
        </CheckBox>
      </label>
      <button onClick={handleSubmit}>会員登録する</button>
    </div>
  );
};
