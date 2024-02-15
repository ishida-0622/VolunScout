"use client";

import { useLazyQuery } from "@apollo/client";
import { notFound, useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import { Button, Spinner } from "react-bootstrap";

import { VolunteerItem } from "../VolunteerItem";

import styles from "./index.module.css";

import type { GetAllVolunteerByGidQuery } from "@/__generated__/query/graphql";

import { gql } from "@/__generated__/query";
import { SearchBar } from "@/components/ui-parts/SearchBar/index";
import { URL_PATH_GROUP } from "@/consts";
import { useAuthContext } from "@/contexts/AuthContext";

type Props = {
  type: "all" | "active" | "scheduled";
};

const GetAllVolunteersQuery = gql(/* GraphQL */ `
  query GetAllVolunteerByGid($gid: String!) {
    volunteers: getVolunteerByGid(gid: $gid) {
      vid
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

const GetActiveVolunteersQuery = gql(/* GraphQL */ `
  query GetActiveVolunteerByGid($gid: String!) {
    volunteers: getActivitiesByGid(gid: $gid) {
      vid
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

const GetScheduledVolunteersQuery = gql(/* GraphQL */ `
  query GetScheduledVolunteerByGid($gid: String!) {
    volunteers: getScheduledActivitiesByGid(gid: $gid) {
      vid
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

export const VolunteerList = ({ type }: Props) => {
  const { user } = useAuthContext();
  const router = useRouter();

  const [isDeActive, setIsDeActive] = useState(false);

  const toCreatePage = () => {
    setIsDeActive(true);
    router.push(URL_PATH_GROUP.VOLUNTEER_CREATE);
    setIsDeActive(false);
  };

  const [getVolunteers, { data, loading, error }] = useLazyQuery(
    type === "all"
      ? GetAllVolunteersQuery
      : type === "active"
      ? GetActiveVolunteersQuery
      : GetScheduledVolunteersQuery,
    {
      fetchPolicy: "cache-and-network",
    }
  );

  const volunteers = data?.volunteers ?? [];

  const [showVolunteers, setShowVolunteers] = useState<
    GetAllVolunteerByGidQuery["volunteers"]
  >([]);

  useEffect(() => {
    if (user?.uid) {
      getVolunteers({ variables: { gid: user.uid } })
        .then((res) => {
          if (res.data) {
            setShowVolunteers(res.data.volunteers);
          }
        })
        .catch(() => {});
    }
  }, [getVolunteers, user?.uid]);

  const search = (s: string) => {
    const reg = new RegExp(s, "i");
    setShowVolunteers(
      volunteers.filter(
        (v) =>
          reg.test(v.title) ||
          reg.test(v.message) ||
          reg.test(v.overview) ||
          reg.test(v.place)
      )
    );
  };

  if (error) {
    notFound();
  }

  return (
    <div>
      <div className={styles.search_bar_wrapper}>
        <SearchBar
          onChange={search}
          placeholder={
            type === "all"
              ? "ボランティアを検索"
              : type === "active"
              ? "活動履歴を検索"
              : "活動予定を検索"
          }
          className={styles.search_bar}
        />
        {isDeActive ? (
          <Button size="lg" disabled>
            <Spinner
              as="span"
              animation="grow"
              size="sm"
              role="status"
              aria-hidden="true"
            />
          </Button>
        ) : (
          <Button
            variant="primary"
            size="lg"
            className={styles.create_button}
            onClick={toCreatePage}
          >
            新規掲載
          </Button>
        )}
      </div>
      {loading && <Spinner />}
      {showVolunteers.map((volunteer) => (
        <VolunteerItem key={volunteer.vid} volunteer={volunteer} />
      ))}
    </div>
  );
};
