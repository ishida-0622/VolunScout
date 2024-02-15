"use client";

import { useRouter } from "next/navigation";
import { Button, Col, Row } from "react-bootstrap";

import { FavButton } from "../FavButton";

import type { GetFavQuery } from "@/__generated__/query/graphql";

import { URL_PATH_PARTICIPANT } from "@/consts";
import { dateTimeToGoogleCalendarFormat } from "@/utils/dateTimeToGoogleCalendarFormat";
import { formatDate } from "@/utils/formatDate";

type Props = {
  volunteer: GetFavQuery["volunteers"][number];
  initIsFav: boolean;
  fav?: () => void;
  unFav?: () => void;
  isCalendar?: boolean;
  isReview?: boolean;
  onReviewClick?: () => void;
};

export const VolunteerItem = ({
  volunteer,
  initIsFav,
  fav,
  unFav,
  isCalendar,
  isReview,
  onReviewClick,
}: Props) => {
  const router = useRouter();
  const toVolunteer = () => {
    router.push(URL_PATH_PARTICIPANT.VOLUNTEER_DETAIL(volunteer.vid));
  };

  const addGoogleCalendar = () => {
    // 別タブでGoogleカレンダーに追加する
    const url = `https://www.google.com/calendar/render?action=TEMPLATE&text=${
      volunteer.title
    }&details=${volunteer.overview}&location=${
      volunteer.place
    }&dates=${dateTimeToGoogleCalendarFormat(
      volunteer.startAt
    )}/${dateTimeToGoogleCalendarFormat(volunteer.finishAt)}`;
    window.open(url, "_blank");
  };

  return (
    <div
      className="border border-2 rounded-2 p-2 w-100 m-auto mb-2"
      style={{ maxWidth: "1080px" }}
    >
      <Row className="align-items-center">
        <Col sm={10} onClick={toVolunteer} role="button">
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
            <Col sm="4">
              <span>日時：</span>
              <span>{formatDate(volunteer.startAt)}</span>
              <span>～</span>
              <span>{formatDate(volunteer.finishAt)}</span>
            </Col>
            <Col sm="2">
              <span>人数：</span>
              <span>{volunteer.recruitedNum}</span>
            </Col>
          </Row>
          <Row>
            <Col>{volunteer.overview}</Col>
          </Row>
        </Col>
        <Col sm={2}>
          <Row>
            <Col className="text-center">
              <FavButton
                vid={volunteer.vid}
                initIsFav={initIsFav}
                onFavClick={fav}
                onUnFavClick={unFav}
              />
            </Col>
          </Row>
          <Row>
            {isCalendar && (
              <Col>
                <Button size="sm" className="w-100" onClick={addGoogleCalendar}>
                  <span>
                    Googleカレンダー
                    <br />
                    に追加
                  </span>
                </Button>
              </Col>
            )}
            {isReview && (
              <Col>
                <Button size="sm" className="w-100" onClick={onReviewClick}>
                  評価する
                </Button>
              </Col>
            )}
          </Row>
        </Col>
      </Row>
    </div>
  );
};
