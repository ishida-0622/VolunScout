"use client";

import { useLazyQuery } from "@apollo/client";
import { useRouter } from "next/navigation";
import { useEffect } from "react";
import { Button, Col, Container, Row } from "react-bootstrap";

import styles from "./index.module.css";

import { gql } from "@/__generated__/query";
import { joinClassnames } from "@/components/@joinClassnames";
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
      fetchParticipantAccount({ variables: { uid: user.uid } }).catch((e) => {
        console.error(e);
      });
    }
  }, [fetchParticipantAccount, user?.uid]);

  if (loading || data === undefined) {
    return null;
  }

  const userInfo = data.user;
  const targetStatus = data.targetStatus;
  const reviews = data.reviews.map((review: { point: number }) => review.point);
  const point = reviews.length
    ? reviews.reduce((acc, cur) => acc + cur.point, 0) / reviews.length
    : undefined;

  if (error) {
    console.error(error);
    return null;
  }

  return (
    <Container className={joinClassnames("my-3", styles.main_contents)}>
      <Row>
        <Col>
          <Row className="mb-3">
            <Col>
              <p>{userInfo.furigana}</p>
              <h2>{userInfo.name}</h2>
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
        </Col>
        <Col sm="3">
          <Row className="mb-3">
            <Col>
              <Button onClick={toEditPage}>編集</Button>
            </Col>
          </Row>
          <Row className="mb-3">
            <Col>
              <span>{formatReview(point)}</span>
            </Col>
          </Row>
        </Col>
      </Row>
      <Row>
        <Col>{userInfo.profile}</Col>
      </Row>
    </Container>
  );
};
