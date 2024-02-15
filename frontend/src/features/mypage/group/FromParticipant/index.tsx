"use client";

import { useQuery } from "@apollo/client";
import { notFound } from "next/navigation";
import { Col, Container, Row } from "react-bootstrap";

import { GetGroupAccountQuery } from "../MyPage";

import { BackButton } from "@/components/ui-parts/BackButton";
import { formatPhone } from "@/utils/formatPhone";
import { formatReview } from "@/utils/formatReview";

type Props = {
  gid: string;
};

export const FromParticipant = ({ gid }: Props) => {
  const { data, loading } = useQuery(GetGroupAccountQuery, {
    variables: { gid },
  });

  if (loading) {
    return null;
  }

  if (data === undefined) {
    notFound();
  }

  const { user, reviews } = data;
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
      </Row>
      <Row className="mb-3">
        <Col>
          <span>{user.furigana}</span>
          <h2>{user.name}</h2>
        </Col>
        <Col sm="3" className="fs-5">
          {formatReview(point)}
        </Col>
      </Row>
      <Row className="mb-1">
        <Col sm="2">代表者</Col>
        <Col>
          {user.representativeName}（{user.representativeFurigana}）
        </Col>
      </Row>
      <Row className="mb-1">
        <Col sm="2">所在地</Col>
        <Col>{user.address}</Col>
      </Row>
      <Row className="mb-3">
        <Col sm="2">電話番号</Col>
        <Col>{formatPhone(user.phone)}</Col>
      </Row>
      <Row>
        <Col>{user.contents}</Col>
      </Row>
    </Container>
  );
};
