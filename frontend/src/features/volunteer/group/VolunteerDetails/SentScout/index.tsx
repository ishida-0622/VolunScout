import { useQuery } from "@apollo/client";
import { useState } from "react";
import { Badge, Button, Col, Modal, Row, Spinner } from "react-bootstrap";

import type { GetScoutByVidQuery as ST } from "@/__generated__/query/graphql";

import { gql } from "@/__generated__/query";
import { ageCalc } from "@/utils/ageCalc";
import { formatDateTime } from "@/utils/formatDateTime";
import { formatReview } from "@/utils/formatReview";
import { numberToGender } from "@/utils/numberToGender";

type Props = {
  vid: string;
};

const GetScoutByVidQuery = gql(/* GraphQL */ `
  query getScoutByVid($vid: String!) {
    scouts: getScoutByVid(vid: $vid) {
      sid
      uid
      message
      scoutedAt
      isRead
      isDenied
      name
      gender
      birthday
      point
    }
  }
`);

export const SentScout = ({ vid }: Props) => {
  const { data, loading } = useQuery(GetScoutByVidQuery, {
    variables: { vid },
  });

  const [show, setShow] = useState(false);
  const [selectedScout, setSelectedScout] = useState<ST["scouts"][number]>({
    sid: "",
    uid: "",
    message: "",
    scoutedAt: "2000-01-01T00:00:00Z",
    isRead: false,
    isDenied: false,
    name: "",
    gender: 0,
    birthday: "2000-01-01",
    point: 0,
  });
  const handleClose = () => setShow(false);
  const handleShow = (scout: ST["scouts"][number]) => {
    setSelectedScout(scout);
    setShow(true);
  };

  if (loading) {
    return <Spinner />;
  }

  if (data === undefined) {
    return null;
  }

  const { scouts } = data;

  return (
    <>
      {scouts.map((scout) => (
        <Row
          key={scout.sid}
          className="mb-3 border border-2 rounded-2 align-items-center p-2"
        >
          <Col>{scout.name}</Col>
          <Col>{ageCalc(scout.birthday)}歳</Col>
          <Col>{numberToGender(scout.gender)}</Col>
          <Col>{formatReview(scout.point)}</Col>
          <Col sm="1">
            <Badge bg={scout.isRead ? "primary" : "secondary"}>
              {scout.isRead ? "既読" : "未読"}
            </Badge>
          </Col>
          <Col sm="1">{scout.isDenied && <Badge bg="danger">拒否</Badge>}</Col>
          <Col sm="2">
            <Button
              onClick={() => {
                handleShow(scout);
              }}
            >
              確認
            </Button>
          </Col>
        </Row>
      ))}
      <Modal show={show} onHide={handleClose}>
        <Modal.Header closeButton>
          <Modal.Title>スカウト詳細</Modal.Title>
        </Modal.Header>
        <Modal.Body>
          <Row>
            <Col sm="3">名前</Col>
            <Col>{selectedScout.name}</Col>
          </Row>
          <Row>
            <Col sm="3">年齢</Col>
            <Col>{ageCalc(selectedScout.birthday)}歳</Col>
          </Row>
          <Row>
            <Col sm="3">性別</Col>
            <Col>{numberToGender(selectedScout.gender)}</Col>
          </Row>
          <Row>
            <Col sm="3">評価</Col>
            <Col>{formatReview(selectedScout.point)}</Col>
          </Row>
          <Row>
            <Col sm="3">送信日時</Col>
            <Col>{formatDateTime(selectedScout.scoutedAt)}</Col>
          </Row>
          <Row className="mt-2">
            <Col sm="3">メッセージ</Col>
            <Col>{selectedScout.message}</Col>
          </Row>
        </Modal.Body>
      </Modal>
    </>
  );
};
