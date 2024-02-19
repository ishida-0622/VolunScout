"use client";

import { useQuery } from "@apollo/client";
import { useRouter } from "next/navigation";
import { Button, Col, Modal, Row, Spinner } from "react-bootstrap";

import { gql } from "@/__generated__/query";
import { URL_PATH_PARTICIPANT } from "@/consts";
import { formatDateTime } from "@/utils/formatDateTime";

type Props = {
  vid: string;
  show: boolean;
  onHide: () => void;
};

// ボランティアの詳細を取得するクエリ
const GetVolunteerFromScoutPageQuery = gql(/* GraphQL */ `
  query GetVolunteerFromScoutPage($vid: String!) {
    volunteer: getVolunteerById(vid: $vid) {
      vid
      title
      overview
      recruitedNum
      place
      startAt
      finishAt
    }
  }
`);

export const ScoutModal = ({ vid, show, onHide }: Props) => {
  const router = useRouter();

  // ボランティア詳細画面に遷移
  const toVolunteerDetail = () => {
    router.push(URL_PATH_PARTICIPANT.VOLUNTEER_DETAIL(vid));
  };

  const { data, loading, error } = useQuery(GetVolunteerFromScoutPageQuery, {
    variables: { vid },
    skip: !show,
  });

  if (loading) {
    return <Spinner />;
  }

  if (error) {
    return null;
  }

  if (!data) {
    return null;
  }

  const volunteer = data.volunteer;

  return (
    <Modal show={show} onHide={onHide}>
      <Modal.Header closeButton>
        <Modal.Title>{volunteer.title}</Modal.Title>
      </Modal.Header>
      <Modal.Body>
        <Row>
          <Col>{volunteer.overview}</Col>
        </Row>
        <Row>
          <Col>場所：{volunteer.place}</Col>
        </Row>
        <Row>
          <Col>
            日時：{formatDateTime(volunteer.startAt)}～
            {formatDateTime(volunteer.finishAt)}
          </Col>
        </Row>
        <Row>
          <Col>人数：{volunteer.recruitedNum}人</Col>
        </Row>
        <Row>
          <Col sm={{ span: 6, offset: 3 }}>
            <Button className="w-100" onClick={toVolunteerDetail}>
              詳しく見る
            </Button>
          </Col>
        </Row>
      </Modal.Body>
    </Modal>
  );
};
