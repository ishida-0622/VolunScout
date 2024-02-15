import { Button, Col, Row } from "react-bootstrap";

import type { GetParticipantsQuery } from "@/__generated__/query/graphql";

import { ageCalc } from "@/utils/ageCalc";
import { numberToGender } from "@/utils/numberToGender";

type Props = {
  participant: GetParticipantsQuery["participants"][number];
  onReview: (uid: string) => void;
};

export const Participant = ({ participant, onReview }: Props) => {
  return (
    <Row className="align-items-center mb-3 border border-2 rounded-2 align-items-center p-2">
      <Col sm="4">{participant.name}</Col>
      <Col sm="1">{ageCalc(participant.birthday)}歳</Col>
      <Col sm="1">{numberToGender(participant.gender)}</Col>
      <Col>
        <Button onClick={() => onReview(participant.uid)}>評価する</Button>
      </Col>
    </Row>
  );
};
