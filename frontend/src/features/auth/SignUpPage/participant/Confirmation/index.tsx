import { useRouter } from "next/navigation";
import { useRef } from "react";

import styles from "./index.module.css";

import type { CreateParticipantAccountRequestBody } from "@/__generated__/command";
import type { FormValues } from "..";

import { apiClientParticipant } from "@/api/command";
import { CheckBox } from "@/components/ui-parts/CheckBox";
import { CONDITIONS, THEMES, URL_PATH_PARTICIPANT } from "@/consts";
import { joinClassnames } from "@/components/@joinClassnames";
import { useAuthContext } from "@/contexts/AuthContext";
import { TermsOfUseAndPrivacyPolicyModal } from "@/features/auth/SignUpPage/TermsOfUseAndPrivacyPolicyModal";
import { stringToNumber } from "@/utils/stringToNumber";

type Props = {
  values: FormValues;
  prevPage: () => void;
};

export const Confirmation = ({ values, prevPage }: Props) => {
  const router = useRouter();
  const { user } = useAuthContext();
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
    const uid = user?.uid;
    if (uid === undefined) {
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
      region: values.regions,
      theme: values.themes.filter((theme) => !themesRequiredSet.has(theme)),
      required_theme: values.themesRequired,
      condition: values.conditions.filter(
        (condition) => !conditionsRequiredSet.has(condition),
      ),
      required_condition: values.conditionsRequired,
      target_status: values.targetStatuses,
    };
    try {
      await apiClientParticipant.createParticipantAccount(body);
      alert("会員登録が完了しました");
      router.push(URL_PATH_PARTICIPANT.HOME);
    } catch (error) {
      console.error(error);
      alert("エラーが発生しました");
    }
  };

  return (
    <section>
      <button
        onClick={prevPage}
        className={joinClassnames("btn btn-secondary", styles.return)}
      >
        戻る
      </button>
      <div className={styles.main_contents}>
        <h2>入力情報の確認</h2>
        <div className={styles.name}>
          <p>{values.name}</p>
        </div>
        <div className={styles.profile}>
          <p className={styles.profile_contents_wrapper}>
            <span className={styles.content_title}>生年月日</span>
            <span className={styles.colon}>：</span>
            <span className={styles.contents}>{values.birthday}</span>
          </p>
          <p className={styles.profile_contents_wrapper}>
            <span className={styles.content_title}>性別</span>
            <span className={styles.colon}>：</span>
            <span className={styles.contents}>
              {values.gender === "0"
                ? "男性"
                : values.gender === "1"
                ? "女性"
                : "その他"}
            </span>
          </p>
          <p className={styles.profile_contents_wrapper}>
            <span className={styles.content_title}>電話番号</span>
            <span className={styles.colon}>：</span>
            <span className={styles.contents}>{values.phone}</span>
          </p>
          <p className={styles.profile_contents_wrapper}>
            <span className={styles.content_title}>プロフィール</span>
            <span className={styles.colon}>：</span>
            <span className={styles.contents}>
              {values.profile || "未入力"}
            </span>
          </p>
        </div>
        <div className={styles.select}>
          <details open>
            <summary className={styles.main}>地域</summary>
            {values.regions.map((region) => (
              <p key={region}>{region}</p>
            ))}
          </details>
          <details open>
            <summary className={styles.main}>テーマ</summary>
            {THEMES.map((theme) => {
              if (themesRequiredSet.has(theme)) {
                return (
                  <p key={theme}>
                    <span className={styles.asterisk}>※</span>
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
            <summary className={styles.main}>活動希望条件</summary>
            {CONDITIONS.map((condition) => {
              if (conditionsRequiredSet.has(condition)) {
                return (
                  <p key={condition}>
                    <span className={styles.asterisk}>※</span>
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
        </div>
        <label>
          <CheckBox label="" onChange={handleAgreed} />
          <TermsOfUseAndPrivacyPolicyModal />
        </label>
        <button
          onClick={handleSubmit}
          className={joinClassnames("btn btn-success", styles.button)}
        >
          会員登録する
        </button>
      </div>
    </section>
  );
};
