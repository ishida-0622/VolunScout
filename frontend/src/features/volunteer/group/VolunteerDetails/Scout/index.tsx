import { useLazyQuery, useQuery } from "@apollo/client";
import { useEffect, useRef, useState } from "react";
import { Button, Col, Form, Modal, Row, Spinner } from "react-bootstrap";

import type { CreateScoutRequestBody } from "@/__generated__/command";
import type {
  GetParticipantFromScoutQuery as PT,
  SearchParticipantQuery as ST,
} from "@/__generated__/query/graphql";

import { gql } from "@/__generated__/query";
import { apiClientScout } from "@/api/command";
import { ageCalc } from "@/utils/ageCalc";
import { formatReview } from "@/utils/formatReview";
import { numberToGender } from "@/utils/numberToGender";

type Props = {
  vid: string;
};

const GetVolunteerElementQuery = gql(/* GraphQL */ `
  query getVolunteerElement($vid: String!) {
    elements: getVolunteerElementsById(vid: $vid) {
      regions
      themes
      requiredThemes
      conditions
      requiredConditions
      targetStatus
    }
  }
`);

const GetParticipantFromScoutQuery = gql(/* GraphQL */ `
  query getParticipantFromScout($uid: String!) {
    participant: getParticipantAccount(uid: $uid) {
      profile
    }
    reviews: getParticipantReviewByUid(uid: $uid) {
      point
      comment
    }
  }
`);

const SearchParticipantQuery = gql(/* GraphQL */ `
  query searchParticipant(
    $vid: String!
    $regions: [String!]!
    $themes: [String!]!
    $requiredThemes: [String!]!
    $conditions: [String!]!
    $requiredConditions: [String!]!
    $targetStatus: [String!]!
  ) {
    participants: scoutParticipantByElements(
      vid: $vid
      regions: $regions
      themes: $themes
      requiredThemes: $requiredThemes
      conditions: $conditions
      requiredConditions: $requiredConditions
      targetStatus: $targetStatus
    ) {
      uid
      name
      gender
      birthday
      point
    }
  }
`);

export const Scout = ({ vid }: Props) => {
  const { data: elements, loading } = useQuery(GetVolunteerElementQuery, {
    variables: { vid },
  });

  const [getParticipant] = useLazyQuery(GetParticipantFromScoutQuery);

  const [searchParticipant, { data }] = useLazyQuery(SearchParticipantQuery);

  const [show, setShow] = useState(false);
  const [uid, setUid] = useState("");
  const [participant, setParticipant] = useState<
    PT & ST["participants"][number] & { avg: number | undefined }
  >({
    uid: "",
    name: "",
    gender: 0,
    participant: {
      profile: "",
    },
    birthday: "2000-01-01",
    reviews: [],
    avg: undefined,
  });
  const handleClose = () => setShow(false);
  const handleShow = (arg: ST["participants"][number]) => {
    setUid(uid);
    setParticipant((prev) => ({
      ...prev,
      name: arg.name,
      gender: arg.gender,
      birthday: arg.birthday,
      avg: arg.point ?? undefined,
    }));
    setShow(true);
  };

  const [selected, setSelected] = useState<string[]>([]);
  const messageRef = useRef<HTMLTextAreaElement>(null);

  const [showMessage, setShowMessage] = useState(false);
  const handleShowMessage = () => setShowMessage(true);
  const handleCloseMessage = () => setShowMessage(false);

  const handleScout = async () => {
    const message = messageRef.current?.value ?? "";

    const reqs = selected.map((uid) => {
      const body: CreateScoutRequestBody = {
        uid,
        vid,
        message,
      };
      return apiClientScout.createScout(body);
    });
    await Promise.all(reqs);
    alert("送信しました");
    handleCloseMessage();
  };

  useEffect(() => {
    if (elements) {
      const {
        regions,
        themes,
        requiredThemes,
        conditions,
        requiredConditions,
        targetStatus,
      } = elements.elements;
      searchParticipant({
        variables: {
          vid,
          regions,
          themes,
          requiredThemes,
          conditions,
          requiredConditions,
          targetStatus,
        },
      }).catch(() => {});
    }
  }, [elements, searchParticipant]);

  useEffect(() => {
    if (uid === "") {
      return;
    }
    getParticipant({ variables: { uid } })
      .then((res) => {
        if (res.data) {
          setParticipant((prev) => ({
            ...prev,
            reviews: res.data?.reviews ?? [],
            participant: { profile: res.data!.participant.profile ?? "" },
          }));
        }
      })
      .catch(() => {});
  }, [getParticipant, uid]);

  if (loading) {
    return <Spinner />;
  }

  if (elements === undefined) {
    return null;
  }

  if (data === undefined) {
    return null;
  }

  const { participants } = data;

  return (
    <>
      {participants.map((p) => (
        <Row
          key={p.uid}
          className="mb-3 border border-2 rounded-2 align-items-center p-2"
        >
          <Col>{p.name}</Col>
          <Col>{ageCalc(p.birthday)}歳</Col>
          <Col>{numberToGender(p.gender)}</Col>
          <Col>{formatReview(p.point ?? undefined)}</Col>
          <Col sm="2">
            <Button
              onClick={() => {
                handleShow(p);
              }}
            >
              詳細
            </Button>
          </Col>
          <Col>
            <Form.Check
              aria-label="option 1"
              onChange={(e) => {
                if (e.target.checked) {
                  setSelected((prev) => [...prev, p.uid]);
                } else {
                  setSelected((prev) => prev.filter((v) => v !== p.uid));
                }
              }}
            ></Form.Check>
          </Col>
        </Row>
      ))}
      <Button onClick={handleShowMessage}>一斉スカウト送信</Button>

      <Modal show={show} onHide={handleClose}>
        <Modal.Header closeButton>
          <Modal.Title>詳細</Modal.Title>
        </Modal.Header>
        {data && (
          <Modal.Body>
            <Row>
              <Col sm="3">名前</Col>
              <Col>{participant.name}</Col>
            </Row>
            <Row>
              <Col sm="3">年齢</Col>
              <Col>{ageCalc(participant.birthday)}歳</Col>
            </Row>
            <Row>
              <Col sm="3">性別</Col>
              <Col>{numberToGender(participant.gender)}</Col>
            </Row>
            <Row>
              <Col sm="3">評価</Col>
              <Col>{formatReview(participant.avg)}</Col>
            </Row>
            {participant.reviews.length > 0 && (
              <Row>
                <details style={{ maxHeight: "10rem", overflow: "auto" }}>
                  <summary>レビュー詳細</summary>
                  {participant.reviews.map((r) => (
                    <Row key={r.comment}>
                      <Col sm={3}>{formatReview(r.point)}</Col>
                      <Col sm={9}>{r.comment}</Col>
                    </Row>
                  ))}
                </details>
              </Row>
            )}
          </Modal.Body>
        )}
      </Modal>
      <Modal show={showMessage} onHide={handleCloseMessage}>
        <Modal.Header closeButton>
          <Modal.Title>一斉スカウト送信</Modal.Title>
        </Modal.Header>
        <Modal.Body>
          <Form.Group>
            <Form.Label>メッセージ</Form.Label>
            <Form.Control
              as="textarea"
              rows={3}
              ref={messageRef}
              placeholder="メッセージを入力"
            />
          </Form.Group>
        </Modal.Body>
        <Modal.Footer>
          <Button variant="secondary" onClick={handleCloseMessage}>
            キャンセル
          </Button>
          <Button variant="primary" onClick={handleScout}>
            送信
          </Button>
        </Modal.Footer>
      </Modal>
    </>
  );
};
