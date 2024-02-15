/* eslint-disable @typescript-eslint/no-unsafe-call */
/* eslint-disable @typescript-eslint/no-unsafe-member-access */
/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-argument */
"use client";

import { useRef, useState } from "react";
import { Button, Col, Container, Form, Row } from "react-bootstrap";

import type { FormValues } from "..";

import { TARGET_STATUSES } from "@/consts";

type Personal = Pick<
  FormValues,
  "name" | "furigana" | "phone" | "birthday" | "gender" | "targetStatuses"
>;

type Props = {
  onNextPage: (values: Partial<FormValues>) => void;
  values: Partial<FormValues>;
};

export const PersonalRegistration = ({ onNextPage, values }: Props) => {
  const [validated, setValidated] = useState(false);
  const ref = useRef<HTMLFormElement>(null);
  const [invalidDateMessage, setInvalidDateMessage] =
    useState("入力してください");

  const onSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const form = e.currentTarget;

    const b = Date.parse(
      form.birthday.valueAsDate?.toISOString().split("T")[0]
    );
    if (b > Date.now() || b < Date.parse("1900-01-01")) {
      setInvalidDateMessage("不正な日付です");
    } else {
      setInvalidDateMessage("入力してください");
    }

    if (form.checkValidity() === false) {
      e.stopPropagation();
    } else {
      // MEMO: any にアクセスしているが, フィールドが存在していることは保証されている
      const personal: Personal = {
        name: form.username.value,
        furigana: form.furigana.value,
        phone: form.phone.value,
        birthday: form.birthday.valueAsDate?.toISOString().split("T")[0],
        gender: form.sex.value,
        targetStatuses: form.targetStatuses.value,
      };

      onNextPage(personal);
    }
    setValidated(true);
  };

  return (
    <Container>
      <Row>
        <Form noValidate validated={validated} onSubmit={onSubmit} ref={ref}>
          <Form.Group as={Row} controlId="username" className="mb-3">
            <Form.Label column sm={2}>
              氏名
            </Form.Label>
            <Col>
              <Form.Control
                type="text"
                placeholder="山田 太郎"
                defaultValue={values.name}
                required
              />
              <Form.Control.Feedback type="invalid">
                入力してください
              </Form.Control.Feedback>
            </Col>
          </Form.Group>
          <Form.Group as={Row} controlId="furigana" className="mb-3">
            <Form.Label column sm={2}>
              フリガナ
            </Form.Label>
            <Col>
              <Form.Control
                type="text"
                placeholder="ヤマダ タロウ"
                defaultValue={values.furigana}
                required
              />
              <Form.Control.Feedback type="invalid">
                入力してください
              </Form.Control.Feedback>
            </Col>
          </Form.Group>
          <Form.Group as={Row} controlId="birthday" className="mb-3">
            <Form.Label column sm={2}>
              生年月日
            </Form.Label>
            <Col>
              <Form.Control
                type="date"
                defaultValue={values.birthday}
                required
                min="1900-01-01"
                max={new Date().toISOString().split("T")[0]}
              />
              <Form.Control.Feedback type="invalid">
                {invalidDateMessage}
              </Form.Control.Feedback>
            </Col>
          </Form.Group>
          <Form.Group as={Row} controlId="targetStatuses" className="mb-3">
            <Form.Label column sm={2}>
              区分
            </Form.Label>
            <Col>
              <Form.Control
                as="select"
                required
                defaultValue={values.targetStatuses ?? ""}
              >
                <option value="" disabled>
                  選択してください
                </option>
                {TARGET_STATUSES.map((target) => (
                  <option key={target} value={target}>
                    {target}
                  </option>
                ))}
              </Form.Control>
              <Form.Control.Feedback type="invalid">
                選択してください
              </Form.Control.Feedback>
            </Col>
          </Form.Group>
          <Form.Group as={Row} controlId="phone" className="mb-3">
            <Form.Label column sm={2}>
              電話番号
            </Form.Label>
            <Col>
              <Form.Control
                type="tel"
                placeholder="080xxxxxxxx"
                defaultValue={values.phone}
                required
                pattern="[0-9]{10,11}"
              />
              <Form.Control.Feedback type="invalid">
                半角数字のみで入力してください
              </Form.Control.Feedback>
            </Col>
          </Form.Group>
          <Form.Group as={Row} controlId="sex" className="mb-3">
            <Form.Label column sm={2}>
              性別
            </Form.Label>
            <Col>
              <Form.Control
                as="select"
                required
                defaultValue={values.gender ?? ""}
              >
                <option value="" disabled>
                  選択してください
                </option>
                <option value={0}>男性</option>
                <option value={1}>女性</option>
                <option value={2}>その他</option>
              </Form.Control>
              <Form.Control.Feedback type="invalid">
                選択してください
              </Form.Control.Feedback>
            </Col>
          </Form.Group>
          <Row>
            <Col sm={{ span: "6", offset: "3" }}>
              <Button type="submit" variant="success" className="w-100">
                ボランティア希望の入力へ
              </Button>
            </Col>
          </Row>
        </Form>
      </Row>
    </Container>
  );
};
