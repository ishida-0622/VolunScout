import { useQuery } from "@apollo/client";
import { notFound } from "next/navigation";
import { useState } from "react";
import { Badge, Col, Row, Spinner } from "react-bootstrap";

import { Participant } from "./Participant";
import { ReviewModal } from "./ReviewModal";

import { gql } from "@/__generated__/query";

type Props = {
  vid: string;
};

const GetParticipantsQuery = gql(/* GraphQL */ `
  query GetParticipants($vid: String!) {
    participants: getPastVolunteerParticipantsByVid(vid: $vid) {
      uid
      name
      gender
      birthday
    }
  }
`);

export const Review = ({ vid }: Props) => {
  const { data, loading } = useQuery(GetParticipantsQuery, {
    variables: { vid },
  });

  const [show, setShow] = useState(false);
  const [uid, setUid] = useState("");

  const handleShow = (uid: string) => {
    setUid(uid);
    setShow(true);
  };
  const handleClose = () => setShow(false);

  if (loading) {
    return <Spinner />;
  }

  if (data === undefined) {
    notFound();
  }

  const { participants } = data;

  return (
    <div className="my-3">
      <Row className="text-center mb-3">
        <h2>
          <Badge className="w-100">参加者確認</Badge>
        </h2>
      </Row>
      <Row>
        <Col>
          {participants.map((participant, index) => (
            <Participant
              key={index}
              participant={participant}
              onReview={handleShow}
            />
          ))}
        </Col>
      </Row>
      <ReviewModal vid={vid} uid={uid} show={show} onHide={handleClose} />
    </div>
  );
};
