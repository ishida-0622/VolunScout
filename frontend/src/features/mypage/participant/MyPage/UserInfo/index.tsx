"use client";

import { useLazyQuery } from "@apollo/client";
import { notFound, useRouter } from "next/navigation";
import { useEffect } from "react";
import { Button, Col, Container, Row } from "react-bootstrap";

import { gql } from "@/__generated__/query";
import { BackButton } from "@/components/ui-parts/BackButton";
import { URL_PATH_PARTICIPANT } from "@/consts";
import { useAuthContext } from "@/contexts/AuthContext";
import { formatDate } from "@/utils/formatDate";
import { formatPhone } from "@/utils/formatPhone";
import { formatReview } from "@/utils/formatReview";
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
    targetStatus: getParticipantTargetStatus(uid: $uid) {
      name
    }
    reviews: getParticipantReviewByUid(uid: $uid) {
      point
    }
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
      fetchParticipantAccount({ variables: { uid: user.uid } }).catch(() => {});
    }
  }, [fetchParticipantAccount, user?.uid]);

  if (loading || data === undefined) {
    return null;
  }

  const userInfo = data.user;
  const targetStatus = data.targetStatus;
  const reviews = data.reviews.map((review: { point: number }) => review.point);
  const point = reviews.length
    ? reviews.reduce((acc, cur) => acc + cur, 0) / reviews.length
    : undefined;

  if (error) {
    notFound();
  }

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
      <Row className="mb-3">
        <Col sm="2">
          <span>生年月日</span>
        </Col>
        <Col>
          <span>{formatDate(userInfo.birthday)}</span>
        </Col>
      </Row>
      <Row className="mb-3">
        <Col sm="2">
          <span>区分</span>
        </Col>
        <Col>
          <span>{targetStatus.name}</span>
        </Col>
      </Row>
      <Row className="mb-3">
        <Col sm="2">
          <span>性別</span>
        </Col>
        <Col>
          <span>{numberToGender(userInfo.gender)}</span>
        </Col>
      </Row>
      <Row className="mb-3">
        <Col sm="2">
          <span>電話番号</span>
        </Col>
        <Col>
          <span>{formatPhone(userInfo.phone)}</span>
        </Col>
      </Row>
      <Row>
        <Col>{userInfo.profile}</Col>
      </Row>
    </Container>
  );
};
