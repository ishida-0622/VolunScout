"use client";

import { useLazyQuery } from "@apollo/client";
import { useCallback, useEffect, useState } from "react";
import {
  Container,
  Row,
  Spinner,
  ToggleButton,
  ToggleButtonGroup,
} from "react-bootstrap";

import { VolunteerItem } from "../VolunteerItem";

import { gql } from "@/__generated__/query";
import { SearchBar } from "@/components/ui-parts/SearchBar/index";
import { useAuthContext } from "@/contexts/AuthContext";

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
    favs: getFavoriteByUid(uid: $uid) {
      vid
    }
  }
`);

export const ApplyList = () => {
  const { user } = useAuthContext();

  const [getApply, { data, loading, error }] = useLazyQuery(
    GetApplyFromApplyListQuery
  );

  const [favs, setFavs] = useState<Set<string>>(new Set());

  const [searchedActivities, setSearchedActivities] = useState(
    data?.activities ?? []
  );
  const [searchedScheduledActivities, setSearchedScheduledActivities] =
    useState(data?.scheduledActivities ?? []);

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
    },
    [data]
  );

  useEffect(() => {
    if (user) {
      (async () => {
        try {
          const res = await getApply({ variables: { uid: user.uid } });
          if (res.data) {
            setFavs(new Set(res.data.favs.map((v) => v.vid)));
            setSearchedActivities(res.data.activities);
            setSearchedScheduledActivities(res.data.scheduledActivities);
          }
        } catch (e) {
          console.error(e);
        }
      })().catch(console.error);
    }
  }, [getApply, user]);

  const [page, setPage] = useState<"activities" | "scheduledActivities">(
    "activities"
  );
  const showActivities = () => {
    setPage("activities");
  };
  const showScheduledActivities = () => {
    setPage("scheduledActivities");
  };

  if (loading) {
    return <Spinner />;
  }

  if (error) {
    console.error(error);
    return null;
  }

  if (!data) {
    return null;
  }

  return (
    <Container>
      <SearchBar onChange={search} className="my-2" />
      <Row className="my-3">
        <ToggleButtonGroup type="radio" name="buttons" defaultValue={"apply"}>
          <ToggleButton
            id="show-activities"
            value={"apply"}
            onClick={showActivities}
          >
            今後の活動予定
          </ToggleButton>
          <ToggleButton
            id="show-scheduled-activities"
            value={"scout"}
            onClick={showScheduledActivities}
          >
            過去の活動
          </ToggleButton>
        </ToggleButtonGroup>
      </Row>
      {page === "activities" &&
        searchedActivities.map((volunteer) => (
          <VolunteerItem
            key={volunteer.vid}
            volunteer={volunteer}
            initIsFav={favs.has(volunteer.vid)}
            isCalendar
          />
        ))}
      {page === "scheduledActivities" &&
        searchedScheduledActivities.map((volunteer) => (
          <VolunteerItem
            key={volunteer.vid}
            volunteer={volunteer}
            initIsFav={favs.has(volunteer.vid)}
          />
        ))}
    </Container>
  );
};
