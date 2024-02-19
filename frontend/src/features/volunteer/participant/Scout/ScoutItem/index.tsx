"use client";

import { Button, Col, Row } from "react-bootstrap";

import type {
  UpdateScoutDeniedRequestBody,
  UpdateScoutIsReadRequestBody,
} from "@/__generated__/command";
import type { GetScoutByUidQuery } from "@/__generated__/query/graphql";

import { apiClientScout } from "@/api/command";

type Props = {
  scout: GetScoutByUidQuery["scouts"][number];
  onPreview: () => void;
  refetch: () => Promise<unknown>;
};

export const ScoutItem = ({ scout, onPreview, refetch }: Props) => {
  // スカウトを既読にする
  const read = async () => {
    const body: UpdateScoutIsReadRequestBody = {
      sid: scout.sid,
    };
    await apiClientScout.updateScoutIsRead(body);
  };

  // スカウトを辞退する
  const refusal = async () => {
    if (!confirm("本当に辞退しますか？")) {
      return;
    }
    const body: UpdateScoutDeniedRequestBody = {
      sid: scout.sid,
    };
    try {
      await apiClientScout.updateScoutDenied(body);
      await refetch();
    } catch (error) {
      alert("辞退に失敗しました");
    }
  };

  return (
    <div
      className="border border-2 rounded-2 p-2 w-100 m-auto mb-3"
      style={{ maxWidth: "1080px" }}
    >
      <Row className="align-items-center">
        <Col
          onClick={async () => {
            await read();
            onPreview();
          }}
          role="button"
        >
          {scout.message}
        </Col>
        <Col sm="1">
          <Button variant="danger" size="sm" onClick={refusal}>
            辞退
          </Button>
        </Col>
      </Row>
    </div>
  );
};
