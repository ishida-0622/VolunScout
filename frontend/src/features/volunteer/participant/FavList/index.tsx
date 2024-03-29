"use client";

import { useLazyQuery } from "@apollo/client";
import { notFound } from "next/navigation";
import { useEffect, useState } from "react";
import { Col, Row, Spinner } from "react-bootstrap";

import { VolunteerItem } from "../VolunteerItem";

import type { GetFavQuery } from "@/__generated__/query/graphql";

import { gql } from "@/__generated__/query";
import { SearchBar } from "@/components/ui-parts/SearchBar/index";
import { useAuthContext } from "@/contexts/AuthContext";

// お気に入りを取得するクエリ
const GetFavsQuery = gql(/* GraphQL */ `
  query getFav($uid: String!) {
    volunteers: getFavoriteByUid(uid: $uid) {
      vid
      title
      overview
      recruitedNum
      place
      startAt
      finishAt
    }
  }
`);

export const FavList = () => {
  const authContext = useAuthContext();

  const [getFav, { data, loading, error }] = useLazyQuery(GetFavsQuery, {
    fetchPolicy: "cache-and-network",
  });

  const volunteers = data?.volunteers ?? [];

  // 表示するお気に入り
  const [showVolunteers, setShowVolunteers] = useState<
    GetFavQuery["volunteers"]
  >([]);

  // お気に入りのvidをセットに変換
  const [favSet, setFavSet] = useState(
    new Set(volunteers.map((volunteer) => volunteer.vid))
  );

  useEffect(() => {
    if (authContext.user) {
      getFav({ variables: { uid: authContext.user.uid } })
        .then((res) => {
          const favSet = new Set(res.data?.volunteers.map((v) => v.vid));
          setFavSet(favSet);
          setShowVolunteers(res.data?.volunteers ?? []);
        })
        .catch(() => {});
    }
  }, [authContext.user, getFav]);

  const search = (s: string) => {
    const reg = new RegExp(s, "i");
    setShowVolunteers(
      volunteers.filter(
        (v) => reg.test(v.title) || reg.test(v.overview) || reg.test(v.place)
      )
    );
  };

  if (loading) {
    return <Spinner />;
  }

  if (error) {
    notFound();
  }

  return (
    <>
      <Row
        className="items-center m-auto w-100 my-2"
        style={{ maxWidth: "1080px" }}
      >
        <Col>
          <SearchBar onChange={search} placeholder="お気に入り内を検索" />
        </Col>
      </Row>
      {volunteers.length === 0 && (
        <h2 className="m-auto w-75 mb-2 text-center">お気に入りがありません</h2>
      )}
      {showVolunteers.length === 0 && volunteers.length !== 0 && (
        <h2 className="m-auto w-75 mb-2 text-center">検索結果は0件です</h2>
      )}
      {showVolunteers.map((volunteer) => (
        <VolunteerItem
          key={volunteer.vid}
          volunteer={volunteer}
          initIsFav={favSet.has(volunteer.vid)}
        />
      ))}
    </>
  );
};
