"use client";

import { Button, Col, Container, Form, Row } from "react-bootstrap";
import { useForm } from "react-hook-form";

import type { FormValues } from ".";

import { BackButton } from "@/components/ui-parts/BackButton";

type Props = {
  onNextPage: (values: FormValues) => void;
};

const TITLES = {
  QUESTION: "質問",
  TROUBLE: "トラブル",
  FEEDBACK: "フィードバック",
  BUG: "システムの不具合",
  OTHER: "その他",
} as const;

export const InputContact = ({ onNextPage }: Props) => {
  const { register, handleSubmit } = useForm<FormValues>();
  const onSubmit = (data: FormValues) => {
    onNextPage(data);
  };

  const submit = handleSubmit(onSubmit);

  return (
    <Container>
      <Row className="mb-3">
        <Col>
          <BackButton />
        </Col>
      </Row>
      <h1 className="mb-3">お問い合わせ</h1>
      <Form onSubmit={submit}>
        <Form.Group className="mb-3">
          <Form.Label>名前</Form.Label>
          <Form.Control type="text" {...register("name")} />
        </Form.Group>
        <Form.Group className="mb-3">
          <Form.Label>フリガナ</Form.Label>
          <Form.Control type="text" {...register("furigana")} />
        </Form.Group>
        <Form.Group className="mb-3">
          <Form.Label>メールアドレス</Form.Label>
          <Form.Control type="email" {...register("email")} required />
        </Form.Group>
        <Form.Group className="mb-3">
          <Form.Label>電話番号</Form.Label>
          <Form.Control type="tel" {...register("phone")} />
        </Form.Group>
        <Form.Group className="mb-3">
          <Form.Label>お問い合わせ内容</Form.Label>
          <Form.Control
            as="select"
            {...register("title")}
            defaultValue=""
            required
          >
            <option value="" disabled>
              選択してください
            </option>
            <option value={TITLES.QUESTION}>質問</option>
            <option value={TITLES.TROUBLE}>トラブル</option>
            <option value={TITLES.FEEDBACK}>フィードバック</option>
            <option value={TITLES.BUG}>システムの不具合</option>
            <option value={TITLES.OTHER}>その他</option>
          </Form.Control>
        </Form.Group>
        <Form.Group className="mb-3">
          <Form.Label>自由記入欄</Form.Label>
          <Form.Control as="textarea" {...register("text")} required />
        </Form.Group>
        <Button type="submit">確認画面へ</Button>
      </Form>
    </Container>
  );
};
