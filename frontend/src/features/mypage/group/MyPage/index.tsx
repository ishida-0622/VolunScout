"use client";

import { useLazyQuery } from "@apollo/client";
import { notFound, useRouter } from "next/navigation";
import { useEffect } from "react";
import { Button, Col, Container, Row } from "react-bootstrap";

import { gql } from "@/__generated__/query";
import { BackButton } from "@/components/ui-parts/BackButton";
import { URL_PATH_GROUP } from "@/consts";
import { useAuthContext } from "@/contexts/AuthContext";
import { formatPhone } from "@/utils/formatPhone";
import { formatReview } from "@/utils/formatReview";

// グループアカウントを取得するクエリ
export const GetGroupAccountQuery = gql(/* GraphQL */ `
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
    reviews: getVolunteerReviewByGid(gid: $gid) {
      point
    }
  }
`);

// マイページコンポーネント
export const MyPage = () => {
  const router = useRouter();

  // 編集ページへ遷移する関数
  const toEditPage = () => {
    router.push(URL_PATH_GROUP.ACCOUNT_EDIT);
  };

  // 認証情報の取得
  const { user } = useAuthContext();

  // グループアカウントの取得クエリの実行
  const [getGroupAccount, { data, loading, error }] = useLazyQuery(
    GetGroupAccountQuery,
    {
      fetchPolicy: "cache-and-network",
    }
  );

  // 認証情報が変更された際に再取得を行う
  useEffect(() => {
    if (typeof user?.uid === "string") {
      getGroupAccount({ variables: { gid: user.uid } }).catch(() => {});
    }
  }, [getGroupAccount, user?.uid]);

  // ローディング中またはデータが未定義の場合はnullを返す
  if (loading || data === undefined) {
    return null;
  }

  // エラーが発生した場合は404ページを表示
  if (error) {
    notFound();
  }

  // ユーザー情報とレビュー情報を取得
  const { user: userInfo, reviews } = data;

  // レビューの平均点を計算
  const point =
    reviews.length > 0
      ? Math.round(
          (reviews.reduce((acc, cur) => acc + cur.point, 0) / reviews.length) *
            10
        ) / 10
      : undefined;

  // マイページの表示
  return (
    <Container className="my-3">
      <Row className="mb-3">
        <Col>
          <BackButton />
        </Col>
        <Col sm="2">
          <Button onClick={toEditPage}>編集</Button>
        </Col>
      </Row>
      <Row className="mb-3">
        <Col>
          <span>{userInfo.furigana}</span>
          <h2>{userInfo.name}</h2>
        </Col>
        <Col sm="3" className="fs-5">
          {formatReview(point)}
        </Col>
      </Row>
      <Row className="mb-1">
        <Col sm="2">代表者</Col>
        <Col>
          {userInfo.representativeName}（{userInfo.representativeFurigana}）
        </Col>
      </Row>
      <Row className="mb-1">
        <Col sm="2">所在地</Col>
        <Col>{userInfo.address}</Col>
      </Row>
      <Row className="mb-3">
        <Col sm="2">電話番号</Col>
        <Col>{formatPhone(userInfo.phone)}</Col>
      </Row>
      <Row>
        <Col>{userInfo.contents}</Col>
      </Row>
    </Container>
  );
};
