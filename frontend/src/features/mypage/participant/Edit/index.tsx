"use client";

import { useLazyQuery } from "@apollo/client";
import Link from "next/link";
import { notFound, useRouter } from "next/navigation";
import { useEffect } from "react";
import { Container, Row, Spinner } from "react-bootstrap";

import styles from "./index.module.css";
import { useUserInfoForm } from "./useUserInfoForm";

import type { UpdateParticipantAccountRequestBody } from "@/__generated__/command";

import { gql } from "@/__generated__/query";
import { apiClientParticipant } from "@/api/command";
import { joinClassnames } from "@/components/@joinClassnames";
import { URL_PATH_PARTICIPANT } from "@/consts";
import { useAuthContext } from "@/contexts/AuthContext";
import { useTermsForm } from "@/features/volunteer/useTermsForm";

const GetParticipantAccountQuery = gql(/* GraphQL */ `
  query GetParticipantAccount($uid: String!) {
    user: getParticipantAccount(uid: $uid) {
      name
      furigana
      phone
      gender
      birthday
      profile
    }
    regions: getParticipantRegions(uid: $uid) {
      name
    }
    themes: getParticipantThemes(uid: $uid) {
      name
      isRequired
    }
    conditions: getParticipantConditions(uid: $uid) {
      name
      isRequired
    }
    targetStatus: getParticipantTargetStatus(uid: $uid) {
      name
    }
  }
`);

export const EditMyPage = () => {
  const router = useRouter();

  const { user } = useAuthContext();
  const {
    InputForm: UserInfoInputForm,
    getValues: getUserInfoValues,
    setFormValues: setUserInfoValues,
  } = useUserInfoForm({});
  const {
    InputForm: TermsInputForm,
    getValues: getTermsValues,
    setFormValues: setTermsValues,
  } = useTermsForm({ isOpen: true });

  const [fetchParticipantAccount, { loading, error, data }] = useLazyQuery(
    GetParticipantAccountQuery,
    {
      fetchPolicy: "cache-and-network",
    }
  );

  const onSubmit = async () => {
    if (user === null) {
      throw new Error("user is null");
    }

    const body: UpdateParticipantAccountRequestBody = {
      pid: user.uid,
      ...getUserInfoValues(),
      ...getTermsValues(),
    };

    try {
      await apiClientParticipant.updateParticipantAccount(body);
    } catch (e) {
      alert("更新に失敗しました");
      return;
    }

    router.push(URL_PATH_PARTICIPANT.ACCOUNT);
  };

  useEffect(() => {
    if (typeof user?.uid === "string") {
      fetchParticipantAccount({ variables: { uid: user.uid } }).catch(() => {});
    }
  }, [fetchParticipantAccount, user?.uid]);

  useEffect(() => {
    if (data) {
      const { user, regions, themes, conditions, targetStatus } = data;
      setUserInfoValues({
        ...user,
        target_status: targetStatus.name,
        birthday: user.birthday.toString(),
      });
      setTermsValues({
        region: regions.map((region) => region.name),
        theme: themes.flatMap((theme) => (theme.isRequired ? [] : theme.name)),
        required_theme: themes.flatMap((theme) =>
          theme.isRequired ? theme.name : []
        ),
        condition: conditions.flatMap((condition) =>
          condition.isRequired ? [] : condition.name
        ),
        required_condition: conditions.flatMap((condition) =>
          condition.isRequired ? condition.name : []
        ),
      });
    }
  }, [data, setUserInfoValues]);

  if (loading) {
    return <Spinner />;
  }

  if (error) {
    notFound();
  }

  return (
    <Container>
      <Row className="mb-5">{UserInfoInputForm}</Row>
      <Row>{TermsInputForm}</Row>
      <div className={styles.button_wrapper}>
        <button
          className={joinClassnames("btn btn-danger btn-lg", styles.button)}
          onClick={() => router.back()}
        >
          キャンセル
        </button>
        <button
          type="submit"
          className={joinClassnames("btn btn-primary btn-lg", styles.button)}
          onClick={onSubmit}
        >
          更新
        </button>
        <div className="text-center">
          <Link href={URL_PATH_PARTICIPANT.ACCOUNT_DELETE}>
            アカウントを削除する→
          </Link>
        </div>
      </div>
    </Container>
  );
};
