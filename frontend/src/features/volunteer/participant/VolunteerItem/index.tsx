"use client";

import { useRouter } from "next/navigation";
import { Col, Row } from "react-bootstrap";

import { FavButton } from "../FavButton";

import type { GetVolunteerByGidQuery } from "@/__generated__/query/graphql";

import { URL_PATH_PARTICIPANT } from "@/consts";
import { formatDate } from "@/utils/formatDate";

type Props = {
  volunteer: GetVolunteerByGidQuery["volunteers"][number];
  initIsFav: boolean;
  fav?: () => void;
  unFav?: () => void;
};

export const VolunteerItem = ({ volunteer, initIsFav, fav, unFav }: Props) => {
  const router = useRouter();
  const toVolunteer = () => {
    router.push(URL_PATH_PARTICIPANT.VOLUNTEER_DETAIL(volunteer.vid));
  };

  return (
    <div
      className="border border-2 rounded-2 p-2 w-100 m-auto mb-2"
      style={{ maxWidth: "1080px" }}
    >
      <Row className="align-items-center">
        <Col sm={11} onClick={toVolunteer} role="button">
          <Row>
            <Col>
              <h2>{volunteer.title}</h2>
            </Col>
          </Row>
          <Row>
            <Col sm="6">
              <span>場所：</span>
              <span>{volunteer.place}</span>
            </Col>
            <Col sm="3">
              <span>日時：</span>
              <span>{formatDate(volunteer.startAt)}</span>
            </Col>
            <Col sm="3">
              <span>人数：</span>
              <span>{volunteer.recruitedNum}</span>
            </Col>
          </Row>
          <Row>
            <Col>{volunteer.overview}</Col>
          </Row>
        </Col>
        <Col sm={1}>
          <FavButton
            vid={volunteer.vid}
            initIsFav={initIsFav}
            onFavClick={fav}
            onUnFavClick={unFav}
          />
        </Col>
      </Row>
    </div>
  );
};
