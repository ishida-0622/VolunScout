import { useQuery } from "@apollo/client";
import { Button, Col, Modal, Row, Spinner } from "react-bootstrap";

import { gql } from "@/__generated__/query";
import { formatDate } from "@/utils/formatDate";
import { formatReview } from "@/utils/formatReview";
import { numberToGender } from "@/utils/numberToGender";

type Props = {
  uid: string;
  show: boolean;
  onHide: () => void;
  asGroup: boolean;
  aid: string;
};

const GetParticipantQuery = gql(/* GraphQL */ `
  query getParticipantForApplyModal($uid: String!) {
    info: getParticipantAccount(uid: $uid) {
      name
      furigana
      birthday
      gender
      phone
      profile
    }
    review: getParticipantReviewByUid(uid: $uid) {
      point
      comment
    }
  }
`);

const GetMembersQuery = gql(/* GraphQL */ `
  query getMembers($aid: String!) {
    members: getGroupParticipants(aid: $aid) {
      name
      furigana
      gender
      age
    }
  }
`);

export const ParticipantModal = ({
  uid,
  show,
  onHide,
  asGroup,
  aid,
}: Props) => {
  const { data, loading, error } = useQuery(GetParticipantQuery, {
    variables: { uid },
    skip: !show,
  });

  const { data: membersData } = useQuery(GetMembersQuery, {
    variables: { aid },
    skip: !show || !asGroup,
  });

  if (loading) return <Spinner />;

  if (error) {
    console.error(error);
    return null;
  }

  if (!data) return null;

  const info = data.info;
  const review = data.review;

  if (!info || !review) return null;

  return (
    <Modal
      show={show}
      onHide={onHide}
      size="lg"
      aria-labelledby="participant-info-modal"
      centered
    >
      <Modal.Header closeButton>
        <Modal.Title id="participant-info-modal" className="w-75 m-auto">
          <p>{info.furigana}</p>
          <h2>{info.name}</h2>
        </Modal.Title>
      </Modal.Header>
      <Modal.Body className="w-75 m-auto">
        <Row>
          <Col sm={3}>生年月日</Col>
          <Col sm={9}>{formatDate(info.birthday)}</Col>
        </Row>
        <Row>
          <Col sm={3}>性別</Col>
          <Col sm={9}>{numberToGender(info.gender)}</Col>
        </Row>
        <Row>
          <Col sm={3}>電話番号</Col>
          <Col sm={9}>{info.phone}</Col>
        </Row>
        <Row>
          <Col sm={3}>プロフィール</Col>
          <Col sm={9}>{info.profile}</Col>
        </Row>
        <Row>
          <Col sm={3}>レビュー</Col>
          <Col sm={9}>
            {formatReview(
              review.map((r) => r.point).reduce((a, b) => a + b) /
                review.length,
            )}
          </Col>
        </Row>
        {membersData && (
          <Row>
            <details>
              <summary>団体メンバー</summary>
              {membersData.members.map((m) => (
                <Row key={`${m.name}-${m.gender}-${m.age}`}>
                  <Col sm={8}>
                    {m.name}（{m.furigana}）
                  </Col>
                  <Col sm={2}>{numberToGender(m.gender)}</Col>
                  <Col sm={2}>{m.age}歳</Col>
                </Row>
              ))}
            </details>
          </Row>
        )}
        <Row>
          <details style={{ maxHeight: "10rem", overflow: "auto" }}>
            <summary>レビュー詳細</summary>
            {review.map((r) => (
              <Row key={r.comment}>
                <Col sm={3}>{formatReview(r.point)}</Col>
                <Col sm={9}>{r.comment}</Col>
              </Row>
            ))}
          </details>
        </Row>
      </Modal.Body>
      <Modal.Footer>
        <Button onClick={onHide}>閉じる</Button>
      </Modal.Footer>
    </Modal>
  );
};
