"use client";

import { useRouter } from "next/navigation";
import { Button, Col, Container, Row } from "react-bootstrap";

import type { FormValues } from ".";

type Props = {
  values: FormValues;
  onPrevPage: () => void;
};

export const Confirmation = ({ values, onPrevPage }: Props) => {
  const router = useRouter();
  const handleOnClick = () => {
    // TODO: 開発者への送信処理
    // 恐らくLambda関数を呼び出すことになる
    alert("送信しました");
    router.back();
  };

  return (
    <Container>
      <h1>お問い合わせ内容確認</h1>
      <Button variant="secondary" onClick={onPrevPage}>
        戻る
      </Button>
      <Row className="mb-3">
        <Col sm="2">氏名</Col>
        <Col>{values.name}</Col>
      </Row>
      <Row className="mb-3">
        <Col sm="2">フリガナ</Col>
        <Col>{values.furigana}</Col>
      </Row>
      <Row className="mb-3">
        <Col sm="2">メールアドレス</Col>
        <Col>{values.email}</Col>
      </Row>
      <Row className="mb-3">
        <Col sm="2">電話番号</Col>
        <Col>{values.phone}</Col>
      </Row>
      <Row className="mb-3">
        <Col sm="2">お問い合わせ内容</Col>
        <Col>{values.title}</Col>
      </Row>
      <Row>
        <Col sm="2">本文</Col>
        <Col>{values.text}</Col>
      </Row>
      <Button onClick={handleOnClick}>送信</Button>
    </Container>
  );
};
