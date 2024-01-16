"use client";

import { useLazyQuery } from "@apollo/client";
import Image from "next/image";
import { useEffect } from "react";

import { gql } from "@/__generated__/query";
import { useAuthContext } from "@/contexts/AuthContext";
import { formatDate } from "@/utils/formatDate";
import { numberToGender } from "@/utils/numberToGender";

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
    # TODO: レビューの取得
  }
`);

export const UserInfo = () => {
  const { user } = useAuthContext();

  const [fetchParticipantAccount, { loading, error, data }] = useLazyQuery(
    GetParticipantAccountQuery
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
        <button>編集</button>
      </div>
      <div>
        <p>{userInfo.profile}</p>
      </div>
    </div>
  );
};
