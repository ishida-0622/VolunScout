"use client";

import { useRef, useState } from "react";
import { Col, Container, Form, Row } from "react-bootstrap";
import { useForm } from "react-hook-form";

import type { CreateVolunteerRequestBody } from "@/__generated__/command";

import { TARGET_STATUSES } from "@/consts";

export type FormValues = Pick<
  CreateVolunteerRequestBody,
  | "title"
  | "message"
  | "overview"
  | "place"
  | "reward"
  | "recruited_num"
  | "start_at"
  | "finish_at"
  | "deadline_on"
  | "photos"
  | "as_group"
  | "target_status"
>;

const initialValues: FormValues = {
  title: "",
  message: "",
  overview: "",
  place: "",
  recruited_num: 1,
  start_at: "",
  finish_at: "",
  deadline_on: "",
  as_group: true,
  target_status: [],
};

type Props = {
  defaultValues?: FormValues;
  imageRef: React.RefObject<HTMLInputElement>;
};

export const useInfoForm = ({
  defaultValues = initialValues,
  imageRef,
}: Props) => {
  const { register, getValues, setValue } = useForm<FormValues>({
    defaultValues,
  });

  const [validated, setValidated] = useState(false);
  const ref = useRef<HTMLFormElement>(null);

  const [finishAtValidation, setFinishAtValidation] = useState("");
  const [deadlineValidation, setDeadlineValidation] = useState("");

  const validation = () => {
    const form = ref.current;
    if (form === null) {
      return;
    }
    setValidated(true);

    return form.checkValidity();
  };

  const InputForm = (
    <Container className="mt-3">
      <Form noValidate validated={validated} ref={ref}>
        <Form.Group as={Row} controlId="title" className="mb-3">
          <Form.Label column sm="2">
            タイトル
          </Form.Label>
          <Col>
            <Form.Control
              {...register("title")}
              placeholder="子供向けワークショップの運営ボランティア募集！"
              required
            />
            <Form.Control.Feedback type="invalid">
              入力してください
            </Form.Control.Feedback>
          </Col>
        </Form.Group>
        <Form.Group as={Row} controlId="address" className="mb-3">
          <Form.Label column sm="2">
            場所
          </Form.Label>
          <Col>
            <Form.Control
              {...register("place")}
              placeholder="東京都○○区×× 1-2-3"
              required
            />
            <Form.Control.Feedback type="invalid">
              入力してください
            </Form.Control.Feedback>
          </Col>
        </Form.Group>
        <Form.Group
          as={Row}
          controlId="datetime"
          className="align-items-center mb-3"
        >
          <Form.Label column sm="2">
            日時
          </Form.Label>
          <Col>
            <Form.Control
              type="datetime-local"
              {...register("start_at")}
              required
              min={new Date("2000-01-01").toISOString()}
              onChange={(e) => {
                setFinishAtValidation(e.target.value);
              }}
            />
          </Col>
          ～
          <Col>
            <Form.Control
              type="datetime-local"
              {...register("finish_at")}
              required
              min={finishAtValidation}
              onChange={(e) => {
                const date = new Date(e.target.value || "2000-01-01");
                setDeadlineValidation(date.toISOString().split("T")[0]);
              }}
            />
          </Col>
        </Form.Group>
        <Form.Group as={Row} controlId="reward" className="mb-3">
          <Form.Label column sm="2">
            報酬
          </Form.Label>
          <Col>
            <Form.Control
              {...register("reward")}
              placeholder="なし, 交通費支給（上限あり）, 時給1000円 など"
              required
            />
            <Form.Control.Feedback type="invalid">
              入力してください
            </Form.Control.Feedback>
          </Col>
        </Form.Group>
        <Form.Group as={Row} controlId="num" className="mb-3">
          <Form.Label column sm="2">
            募集人数
          </Form.Label>
          <Col>
            <Form.Control
              type="number"
              {...register("recruited_num")}
              required
              min={1}
              defaultValue={1}
            />
            <Form.Control.Feedback type="invalid">
              値が不正です
            </Form.Control.Feedback>
          </Col>
        </Form.Group>
        <Form.Group as={Row} controlId="target" className="mb-3">
          <Form.Label column sm="2">
            募集対象
          </Form.Label>
          <Col>
            {TARGET_STATUSES.map((status) => (
              <Form.Check
                key={status}
                id={`target-status-${status}`}
                type="checkbox"
                value={status}
                label={status}
                {...register("target_status")}
              />
            ))}
            <Form.Control.Feedback type="invalid">
              ひとつ以上選択してください
            </Form.Control.Feedback>
          </Col>
        </Form.Group>
        <Form.Group as={Row} className="mb-3 align-items-center">
          <Form.Label column sm="2">
            団体応募を許可する
          </Form.Label>
          <Col>
            <Form.Check
              {...register("as_group")}
              type="checkbox"
              label=""
              id="as-group"
            />
          </Col>
        </Form.Group>
        <Form.Group as={Row} controlId="deadline" className="mb-3">
          <Form.Label column sm="2">
            募集期限
          </Form.Label>
          <Col>
            <Form.Control
              type="date"
              {...register("deadline_on")}
              max={deadlineValidation}
              required
            />
            <Form.Control.Feedback type="invalid">
              値が不正です
            </Form.Control.Feedback>
          </Col>
        </Form.Group>
        <Form.Group as={Row} className="mb-3">
          <Form.Label column sm="2">
            写真（4枚まで）
          </Form.Label>
          <Col>
            <Form.Control
              type="file"
              accept="image/*"
              multiple
              {...register("photos", {
                required: true,
                // eslint-disable-next-line eqeqeq
                validate: (v) => v != null && v.length <= 4,
              })}
              ref={imageRef}
            />
          </Col>
        </Form.Group>
        <Form.Group as={Row} controlId="overview" className="mb-3">
          <Form.Label column sm="2">
            概要
          </Form.Label>
          <Col>
            <Form.Control
              {...register("overview")}
              as="textarea"
              rows={3}
              placeholder="○○ショッピングセンターで子供向け（幼稚園～小学校低学年）ワークショップを開催します。&#13;折り紙や紙飛行機などを作って遊びます。&#13;交通費として、一律1000円を支給します。"
              required
            />
            <Form.Control.Feedback type="invalid">
              入力してください
            </Form.Control.Feedback>
          </Col>
        </Form.Group>
        <Form.Group as={Row} controlId="message" className="mb-3">
          <Form.Label column sm="2">
            メッセージ
          </Form.Label>
          <Col>
            <Form.Control
              {...register("message")}
              as="textarea"
              rows={3}
              placeholder="子供が好きな方、運営に興味がある方を募集しています！折り紙をするため、指先が器用な方大歓迎です！"
              required
            />
            <Form.Control.Feedback type="invalid">
              入力してください
            </Form.Control.Feedback>
          </Col>
        </Form.Group>
      </Form>
    </Container>
  );
  return { InputForm, getValues, setValue, validation };
};
