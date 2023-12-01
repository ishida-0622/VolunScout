import Router from "next/router";
import { useRef } from "react";

import type { CreateParticipantAccountRequestBody } from "@/__generated__/command";
import type { FormValues } from ".";

import { apiClientParticipant } from "@/api/command";
import { CheckBox } from "@/components/ui-parts/CheckBox";
import { CONDITIONS, THEMES, URL_PATH_PARTICIPANT } from "@/consts";
import { TermsOfUseAndPrivacyPolicyModal } from "@/features/auth/SignUpPage/TermsOfUseAndPrivacyPolicyModal";
import { getUid } from "@/features/auth/utils/getUid";
import { stringToNumber } from "@/utils/stringToNumber";

type Props = {
  values: FormValues;
  prevPage: () => void;
};

export const Confirmation = ({ values, prevPage }: Props) => {
  const themesSet = new Set(values.themes);
  const themesRequiredSet = new Set(values.themesRequired);
  const conditionsSet = new Set(values.conditions);
  const conditionsRequiredSet = new Set(values.conditionsRequired);

  const isAgreed = useRef(false);
  const handleAgreed = (value: boolean) => {
    isAgreed.current = value;
  };

  const handleSubmit = async () => {
    if (!isAgreed.current) {
      alert("利用規約とプライバシーポリシーに同意してください");
      return;
    }
    const uid = await getUid();
    if (!uid) {
      throw new Error("uid is null");
    }
    const body: CreateParticipantAccountRequestBody = {
      birthday: values.birthday,
      furigana: values.furigana,
      gender: stringToNumber(values.gender),
      name: values.name,
      phone: values.phone,
      pid: uid,
      profile: values.profile,
      regions: values.regions,
      themes: values.themes,
      themes_required: values.themesRequired,
      conditions: values.conditions,
      conditions_required: values.conditionsRequired,
    };
    try {
      await apiClientParticipant.createParticipantAccount(body);
      alert("会員登録が完了しました");
      await Router.push(URL_PATH_PARTICIPANT.HOME);
    } catch (error) {
      console.error(error);
      alert("エラーが発生しました");
    }
  };

  return (
    <section>
      <button onClick={prevPage}>戻る</button>
      <h2>入力情報の確認</h2>
      <p>{values.name}</p>
      <p>
        <span>生年月日</span>
        <span>{values.birthday}</span>
      </p>
      <p>
        <span>性別</span>
        <span>
          {values.gender === "0"
            ? "男性"
            : values.gender === "1"
            ? "女性"
            : "その他"}
        </span>
      </p>
      <p>
        <span>電話番号</span>
        <span>{values.phone}</span>
      </p>
      <p>{values.profile}</p>
      <details open>
        <summary>地域</summary>
        {values.regions.map((region) => (
          <p key={region}>{region}</p>
        ))}
      </details>
      <details open>
        <summary>テーマ</summary>
        {THEMES.map((theme) => {
          if (themesRequiredSet.has(theme)) {
            return (
              <p key={theme}>
                <span>※</span>
                <span>{theme}</span>
              </p>
            );
          }
          if (themesSet.has(theme)) {
            return <p key={theme}>{theme}</p>;
          }
          return null;
        })}
      </details>
      <details open>
        <summary>活動希望条件</summary>
        {CONDITIONS.map((condition) => {
          if (conditionsRequiredSet.has(condition)) {
            return (
              <p key={condition}>
                <span>※</span>
                <span>{condition}</span>
              </p>
            );
          }
          if (conditionsSet.has(condition)) {
            return <p key={condition}>{condition}</p>;
          }
          return null;
        })}
      </details>
      <label>
        <CheckBox label="" onChange={handleAgreed} />
        <TermsOfUseAndPrivacyPolicyModal />
      </label>
      <button onClick={handleSubmit}>会員登録する</button>
    </section>
  );
};
