"use client";

import { useLazyQuery } from "@apollo/client";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import { Button, Spinner } from "react-bootstrap";

import { VolunteerItem } from "../../VolunteerItem";

import styles from "./index.module.css";

import type { GetVolunteerByGidQuery } from "@/__generated__/query/graphql";

import { gql } from "@/__generated__/query";
import { SearchBar } from "@/components/ui-parts/SearchBar/index";
import { URL_PATH_GROUP } from "@/consts";
import { useAuthContext } from "@/contexts/AuthContext";

const GetVolunteersQuery = gql(/* GraphQL */ `
  query GetVolunteerByGid($gid: String!) {
    volunteers: getVolunteerByGid(gid: $gid) {
      vid
      title
      message
      overview
      recruitedNum
      place
      startAt
      finishAt
      isDeleted
    }
  }
`);

export const VolunteerList = () => {
  const authContext = useAuthContext();
  const router = useRouter();

  const toCreatePage = () => {
    router.push(URL_PATH_GROUP.VOLUNTEER_CREATE);
  };

  const [getVolunteers, { data, loading, error }] = useLazyQuery(
    GetVolunteersQuery,
    {
      fetchPolicy: "cache-and-network",
    }
  );

  const volunteers = data?.volunteers ?? [];

  const [showVolunteers, setShowVolunteers] = useState<
    GetVolunteerByGidQuery["volunteers"]
  >([]);

  useEffect(() => {
    if (authContext.user?.uid) {
      getVolunteers({ variables: { gid: authContext.user.uid } })
        .then((res) => {
          if (res.data) {
            setShowVolunteers(res.data.volunteers);
          }
        })
        .catch((e) => console.error(e));
    }
  }, [getVolunteers, authContext.user?.uid]);

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
    console.error(error);
    return null;
  }

  return (
    <div>
      <div className={styles.search_bar_wrapper}>
        <SearchBar onChange={search} className={styles.search_bar} />
        <Button
          variant="primary"
          size="lg"
          className={styles.create_button}
          onClick={toCreatePage}
        >
          新規掲載
        </Button>
      </div>
      {loading && <Spinner />}
      {showVolunteers.map((volunteer) => (
        <VolunteerItem
          key={volunteer.vid}
          volunteer={volunteer}
          accountType="group"
        />
      ))}
    </div>
  );
};
