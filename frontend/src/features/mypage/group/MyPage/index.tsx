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
      getGroupAccount({ variables: { gid: user.uid } }).catch(() => {});
    }
  }, [getGroupAccount, user?.uid]);

  if (loading || data === undefined) {
    return null;
  }

  if (error) {
    notFound();
  }

  const { user: userInfo, reviews } = data;
  const point =
    reviews.length > 0
      ? Math.round(
          (reviews.reduce((acc, cur) => acc + cur.point, 0) / reviews.length) *
            10
        ) / 10
      : undefined;

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
