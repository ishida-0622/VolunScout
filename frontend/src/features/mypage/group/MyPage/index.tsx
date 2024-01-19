"use client";

import { useLazyQuery } from "@apollo/client";
import Image from "next/image";
import { useRouter } from "next/navigation";
import { useEffect } from "react";

import styles from "./index.module.css";

import { gql } from "@/__generated__/query";
import { joinClassnames } from "@/components/@joinClassnames";
import { URL_PATH_GROUP } from "@/consts";
import { useAuthContext } from "@/contexts/AuthContext";

const GetGroupAccountQuery = gql(/* GraphQL */ `
  query GetGroupAccountAndReview($gid: String!) {
    user: getGroupAccount(gid: $gid) {
      name
      furigana
      representativeName
      representativeFurigana
      phone
      address
      contents
    }
    # TODO: レビューの取得
  }
`);

export const MyPage = () => {
  const router = useRouter();
  const toEditPage = () => {
    router.push(URL_PATH_GROUP.ACCOUNT_EDIT);
  };

  const { user } = useAuthContext();
  const [getGroupAccount, { data, loading, error }] = useLazyQuery(
    GetGroupAccountQuery,
    {
      fetchPolicy: "cache-and-network",
    }
  );

  useEffect(() => {
    if (typeof user?.uid === "string") {
      getGroupAccount({ variables: { gid: user.uid } }).catch((e) =>
        console.error(e)
      );
    }
  }, [getGroupAccount, user?.uid]);

  if (loading || data === undefined) {
    console.warn("loading or data is undefined");
    return null;
  }

  if (error) {
    console.error(error);
    return null;
  }

  const { __typename, ...userInfo } = data.user;

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
            <span>代表者</span>
            <span>：</span>
            <span>{userInfo.representativeName}</span>
            <span>（{userInfo.representativeFurigana}）</span>
          </p>
          <p>
            <span>所在地</span>
            <span>：</span>
            <span>{userInfo.address}</span>
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
        <p>{userInfo.contents}</p>
      </div>
    </div>
  );
};
