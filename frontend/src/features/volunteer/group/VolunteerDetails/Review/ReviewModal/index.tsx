import { useState } from "react";
import { Button, Col, Form, Modal, Row } from "react-bootstrap";

import type { ReviewToParticipantRequestBody } from "@/__generated__/command";

import { apiClientReview } from "@/api/command";

type Props = {
  vid: string;
  uid: string;
  show: boolean;
  onHide: () => void;
};

export const ReviewModal = ({ vid, uid, show, onHide }: Props) => {
  const [review, setReview] = useState<number>();
  const [comment, setComment] = useState("");

  const sendReview = async () => {
    if (review === undefined) {
      alert("評価してください");
      return;
    }
    const body: ReviewToParticipantRequestBody = {
      vid,
      uid,
      point: review,
      comment,
    };
    try {
      await apiClientReview.reviewToParticipant(body);
      alert("評価しました");
    } catch (error) {
      console.error(error);
      alert("エラーが発生しました");
    } finally {
      onHide();
      setReview(undefined);
      setComment("");
    }
  };

  return (
    <Modal show={show} onHide={onHide}>
      <Modal.Header closeButton onHide={onHide}>
        <Modal.Title>参加者の評価</Modal.Title>
      </Modal.Header>
      <Modal.Body>
        <Row className="mb-3">
          <Col sm="4">評価</Col>
          <Col>
            {[1, 2, 3, 4, 5].map((n) => (
              <span
                key={n}
                onClick={() => setReview(n)}
                className="fs-4"
                role="button"
                style={{ color: "gold" }}
              >
                {(review ?? 0) < n ? "☆" : "★"}
              </span>
            ))}
          </Col>
        </Row>
        <Row>
          <Col sm="4">コメント（任意）</Col>
          <Col>
            <Form.Control
              as="textarea"
              rows={3}
              value={comment}
              onChange={(e) => setComment(e.target.value)}
            />
          </Col>
        </Row>
        <Row>
          <Col>
            <Button onClick={sendReview}>送信</Button>
          </Col>
        </Row>
      </Modal.Body>
    </Modal>
  );
};
