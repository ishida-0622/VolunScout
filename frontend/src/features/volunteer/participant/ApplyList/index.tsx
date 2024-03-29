"use client";

import { useLazyQuery } from "@apollo/client";
import { notFound } from "next/navigation";
import { useCallback, useEffect, useState } from "react";
import {
  Container,
  Row,
  Spinner,
  ToggleButton,
  ToggleButtonGroup,
} from "react-bootstrap";

import { VolunteerItem } from "../VolunteerItem";

import { ReviewModal } from "./ReviewModal";

import { gql } from "@/__generated__/query";
import { SearchBar } from "@/components/ui-parts/SearchBar/index";
import { useAuthContext } from "@/contexts/AuthContext";

// 応募済みボランティア一覧取得
const GetApplyFromApplyListQuery = gql(/* GraphQL */ `
  query getApplyFromApplyList($uid: String!) {
    activities: getActivitiesByUid(uid: $uid) {
      vid
      title
      overview
      recruitedNum
      place
      startAt
      finishAt
    }
    scheduledActivities: getScheduledActivitiesByUid(uid: $uid) {
      vid
      title
      overview
      recruitedNum
      place
      startAt
      finishAt
    }
    notAllowed: getNotAllowedActivitiesByUid(uid: $uid) {
      vid
      title
      overview
      recruitedNum
      place
      startAt
      finishAt
    }
    favs: getFavoriteByUid(uid: $uid) {
      vid
    }
  }
`);

export const ApplyList = () => {
  const { user } = useAuthContext();

  const [getApply, { data, loading, error }] = useLazyQuery(
    GetApplyFromApplyListQuery,
    {
      fetchPolicy: "cache-and-network",
    }
  );

  // お気に入りボランティア
  const [favs, setFavs] = useState<Set<string>>(new Set());

  const [searchedActivities, setSearchedActivities] = useState(
    data?.activities ?? []
  );
  const [searchedScheduledActivities, setSearchedScheduledActivities] =
    useState(data?.scheduledActivities ?? []);
  const [searchedNotAllowed, setSearchedNotAllowed] = useState(
    data?.notAllowed ?? []
  );

  // 検索
  const search = useCallback(
    (s: string) => {
      if (data === undefined) {
        return;
      }
      const reg = new RegExp(s, "i");
      setSearchedActivities(
        data.activities.filter(
          (v) => reg.test(v.title) || reg.test(v.overview) || reg.test(v.place)
        )
      );
      setSearchedScheduledActivities(
        data.scheduledActivities.filter(
          (v) => reg.test(v.title) || reg.test(v.overview) || reg.test(v.place)
        )
      );
      setSearchedNotAllowed(
        data.notAllowed.filter(
          (v) => reg.test(v.title) || reg.test(v.overview) || reg.test(v.place)
        )
      );
    },
    [data]
  );

  // uidが変更されたら取得
  useEffect(() => {
    if (user) {
      (async () => {
        const res = await getApply({ variables: { uid: user.uid } });
        if (res.data) {
          setFavs(new Set(res.data.favs.map((v) => v.vid)));
          setSearchedActivities(res.data.activities);
          setSearchedScheduledActivities(res.data.scheduledActivities);
          setSearchedNotAllowed(res.data.notAllowed);
        }
      })().catch(() => {});
    }
  }, [getApply, user]);

  const [page, setPage] = useState<
    "activities" | "scheduledActivities" | "notAllowed"
  >("scheduledActivities");
  const showActivities = () => {
    setPage("activities");
  };
  const showScheduledActivities = () => {
    setPage("scheduledActivities");
  };
  const showNotAllowedActivities = () => {
    setPage("notAllowed");
  };

  const [show, setShow] = useState(false);
  const [vid, setVid] = useState("");

  const handleShow = (vid: string) => {
    setVid(vid);
    setShow(true);
  };

  const handleClose = () => setShow(false);

  if (loading) {
    return <Spinner />;
  }

  if (error) {
    notFound();
  }

  if (!data || !user) {
    return null;
  }

  return (
    <Container>
      <SearchBar
        onChange={search}
        className="my-2"
        placeholder="応募済みボランティアを検索"
      />
      <Row className="my-3">
        <ToggleButtonGroup
          type="radio"
          name="buttons"
          defaultValue={"scheduled"}
        >
          <ToggleButton
            id="show-scheduled-activities"
            value={"scheduled"}
            onClick={showScheduledActivities}
          >
            今後の活動予定
          </ToggleButton>
          <ToggleButton
            id="show-not-allowed-activities"
            value={"notAllowed"}
            onClick={showNotAllowedActivities}
          >
            応募済み（未承認）
          </ToggleButton>
          <ToggleButton
            id="show-activities"
            value={"apply"}
            onClick={showActivities}
          >
            過去の活動
          </ToggleButton>
        </ToggleButtonGroup>
      </Row>
      {page === "scheduledActivities" &&
        searchedScheduledActivities.map((volunteer) => (
          <VolunteerItem
            key={volunteer.vid}
            volunteer={volunteer}
            initIsFav={favs.has(volunteer.vid)}
            isCalendar
          />
        ))}
      {page === "notAllowed" &&
        searchedNotAllowed.map((volunteer) => (
          <VolunteerItem
            key={volunteer.vid}
            volunteer={volunteer}
            initIsFav={favs.has(volunteer.vid)}
          />
        ))}
      {page === "activities" &&
        searchedActivities.map((volunteer) => (
          <VolunteerItem
            key={volunteer.vid}
            volunteer={volunteer}
            initIsFav={favs.has(volunteer.vid)}
            isReview
            onReviewClick={() => handleShow(volunteer.vid)}
          />
        ))}
      <ReviewModal show={show} vid={vid} uid={user.uid} onHide={handleClose} />
    </Container>
  );
};
