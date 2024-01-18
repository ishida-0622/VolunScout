"use client";

import { useLazyQuery } from "@apollo/client";
import Image from "next/image";
import { useRouter } from "next/navigation";
import { useEffect } from "react";

import styles from "./index.module.css";

import { gql } from "@/__generated__/query";
import { joinClassnames } from "@/components/@joinClassnames";
import { URL_PATH_PARTICIPANT } from "@/consts";
import { useAuthContext } from "@/contexts/AuthContext";
import { formatDate } from "@/utils/formatDate";
import { numberToGender } from "@/utils/numberToGender";

const GetParticipantAccountInfoQuery = gql(/* GraphQL */ `
  query GetParticipantAccountInfo($uid: String!) {
    user: getParticipantAccount(uid: $uid) {
      name
      furigana
      phone
      gender
      birthday
      profile
    }
    # TODO: レビューの取得
  }
`);

export const UserInfo = () => {
  const router = useRouter();
  const toEditPage = () => {
    router.push(URL_PATH_PARTICIPANT.ACCOUNT_EDIT);
  };

  const { user } = useAuthContext();

  const [fetchParticipantAccount, { loading, error, data }] = useLazyQuery(
    GetParticipantAccountInfoQuery,
    {
      fetchPolicy: "cache-and-network",
    }
  );

  useEffect(() => {
    if (typeof user?.uid === "string") {
      fetchParticipantAccount({ variables: { uid: user.uid } }).catch((e) => {
        console.error(e);
      });
    }
  }, [fetchParticipantAccount, user?.uid]);

  const userInfo = data?.user;

  if (loading || userInfo === undefined) {
    return null;
  }

  if (error) {
    console.error(error);
    return null;
  }

  return (
    <div>
      <div>
        <div>
          <Image
            src={user?.photoURL ?? ""}
            alt="User icon"
            width={100}
            height={100}
            className={styles.user_icon}
          />
        </div>
        <div>
          <p>{userInfo.furigana}</p>
          <h2>{userInfo.name}</h2>
          <p>
            <span>生年月日</span>
            <span>：</span>
            <span>{formatDate(userInfo.birthday)}</span>
          </p>
          <p>
            <span>性別</span>
            <span>：</span>
            <span>{numberToGender(userInfo.gender)}</span>
          </p>
          <p>
            <span>電話番号</span>
            <span>：</span>
            <span>{userInfo.phone}</span>
          </p>
        </div>
        <div>
          {/* TODO:レビュー */}
          <p>★★★★☆</p>
        </div>
        <button className={joinClassnames("btn btn-info")} onClick={toEditPage}>
          編集
        </button>
      </div>
      <div>
        <p>{userInfo.profile}</p>
      </div>
    </div>
  );
};
