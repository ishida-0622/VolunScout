"use client";

import { useQuery } from "@apollo/client";
import { notFound, useRouter } from "next/navigation";
import { useState } from "react";
import {
  Button,
  Col,
  Container,
  Row,
  Spinner,
  ToggleButton,
  ToggleButtonGroup,
} from "react-bootstrap";

import { Apply } from "./Apply";
import { Scout } from "./Scout";

import type { DeleteVolunteerRequestBody } from "@/__generated__/command";

import { gql } from "@/__generated__/query";
import { apiClientVolunteer } from "@/api/command";
import { URL_PATH_GROUP } from "@/consts";
import { formatDateTime } from "@/utils/formatDateTime";

type Props = {
  vid: string;
};

const GetVolunTeerDetailsQuery = gql(/* GraphQL */ `
  query GetVolunteerDetails($vid: String!) {
    volunteer: getVolunteerById(vid: $vid) {
      title
      message
      overview
      recruitedNum
      place
      startAt
      finishAt
    }
  }
`);

export const VolunteerDetails = ({ vid }: Props) => {
  const router = useRouter();

  const [page, setPage] = useState<"apply" | "scout">("apply");

  const showApply = () => {
    setPage("apply");
  };
  const showScout = () => {
    setPage("scout");
  };

  const { data, loading, error } = useQuery(GetVolunTeerDetailsQuery, {
    variables: { vid },
  });

  const deleteVolunteer = async () => {
    if (!confirm("本当に削除しますか？")) return;

    try {
      const body: DeleteVolunteerRequestBody = {
        vid,
      };
      await apiClientVolunteer.deleteVolunteer(body);
      alert("削除しました");
      router.push(URL_PATH_GROUP.HOME);
    } catch (error) {
      console.error(error);
    }
  };

  if (loading) {
    return <Spinner />;
  }

  if (error) {
    console.error(error);
    return null;
  }

  if (!data) {
    notFound();
  }

  const volunteer = data.volunteer;

  return (
    <Container>
      <Row>
        <Col>
          <h1>{volunteer.title}</h1>
        </Col>
      </Row>
      <Row>
        <Col>
          <p>場所：{volunteer.place}</p>
          <p>日時：{formatDateTime(volunteer.startAt)}</p>
        </Col>
        <Col>
          <Button variant="primary" className="mx-1">
            編集
          </Button>
          <Button variant="danger" className="mx-1" onClick={deleteVolunteer}>
            削除
          </Button>
        </Col>
      </Row>
      <Row>
        <Col>
          <h2>概要</h2>
          <p>{volunteer.overview}</p>
        </Col>
      </Row>
      <Row className="mb-3">
        <ToggleButtonGroup type="radio" name="buttons" defaultValue={"apply"}>
          <ToggleButton id="show-apply" value={"apply"} onClick={showApply}>
            応募者確認
          </ToggleButton>
          <ToggleButton id="show-scout" value={"scout"} onClick={showScout}>
            スカウト
          </ToggleButton>
        </ToggleButtonGroup>
      </Row>
      <Row>
        {page === "apply" && <Apply vid={vid} />}
        {page === "scout" && <Scout></Scout>}
      </Row>
    </Container>
  );
};
