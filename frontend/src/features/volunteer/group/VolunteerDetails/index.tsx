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
import { Review } from "./Review";
import { Scout } from "./Scout";

import type { DeleteVolunteerRequestBody } from "@/__generated__/command";

import { gql } from "@/__generated__/query";
import { apiClientVolunteer } from "@/api/command";
import { URL_PATH_GROUP } from "@/consts";
import { formatDateTime } from "@/utils/formatDateTime";
import { parseDateTime } from "@/utils/parseDateTime";

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
  const isArchived = parseDateTime(volunteer.finishAt).getTime() < Date.now();

  return (
    <Container>
      <Row className="my-3">
        <Col>
          <h1>{volunteer.title}</h1>
        </Col>
      </Row>
      <Row className="mb-3">
        <Col>
          <Row>
            <Col sm="2">場所</Col>
            <Col>{volunteer.place}</Col>
          </Row>
          <Row>
            <Col sm="2">日時</Col>
            <Col>
              {formatDateTime(volunteer.startAt)} ～{" "}
              {formatDateTime(volunteer.finishAt)}
            </Col>
          </Row>
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
      <Row className="mb-3">
        <Col>
          <h2>概要</h2>
        </Col>
      </Row>
      <Row className="mb-3">
        <Col>
          <p>{volunteer.overview}</p>
        </Col>
      </Row>
      {isArchived ? (
        <>
          <Row>
            <Review vid={vid} />
          </Row>
        </>
      ) : (
        <>
          <Row className="mb-3">
            <ToggleButtonGroup
              type="radio"
              name="buttons"
              defaultValue={"apply"}
            >
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
        </>
      )}
    </Container>
  );
};
