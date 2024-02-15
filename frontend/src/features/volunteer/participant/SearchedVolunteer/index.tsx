"use client";

import { useLazyQuery, useQuery } from "@apollo/client";
import { useRouter, type ReadonlyURLSearchParams } from "next/navigation";
import { useEffect, useRef, useState } from "react";
import {
  Button,
  Col,
  Container,
  Pagination,
  Row,
  Spinner,
} from "react-bootstrap";

import { useTermsForm } from "../../useTermsForm";
import { VolunteerItem } from "../VolunteerItem";

import { gql } from "@/__generated__/query";
import { SearchBar } from "@/components/ui-parts/SearchBar";
import { TARGET_STATUSES, URL_PATH_PARTICIPANT } from "@/consts";
import { useAuthContext } from "@/contexts/AuthContext";
import { objectToUrlSearch } from "@/utils/objectToUrlSearch";

type Props = {
  params: ReadonlyURLSearchParams;
};

const SHOW_ONE_PAGE_ITEMS = 20;

const GetTargetStatus = gql(/* GraphQL */ `
  query GetTargetStatus($uid: String!) {
    targetStatus: getParticipantTargetStatus(uid: $uid) {
      name
    }
  }
`);

const SearchVolunteerQuery = gql(/* GraphQL */ `
  query SearchVolunteer(
    $word: String!
    $region: [String!]!
    $theme: [String!]!
    $requiredTheme: [String!]!
    $condition: [String!]!
    $requiredCondition: [String!]!
    $targetStatus: [String!]!
  ) {
    result: searchVolunteerByElements(
      searchWords: $word
      regions: $region
      requiredRegions: $region
      themes: $theme
      requiredThemes: $requiredTheme
      conditions: $condition
      requiredConditions: $requiredCondition
      targetStatus: $targetStatus
    ) {
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

const GetFavFromSearchedPageQuery = gql(/* GraphQL */ `
  query getFavFromSearchedPage($uid: String!) {
    fav: getFavoriteByUid(uid: $uid) {
      vid
    }
  }
`);

export const SearchedVolunteer = ({ params }: Props) => {
  const router = useRouter();

  // MEMO: word は未入力の場合は空文字列になるため, string 型なのが保証されている
  const word = params.get("word")!;
  const region =
    params
      .get("region")
      ?.split(",")
      .filter((v) => v !== "") ?? [];
  const theme =
    params
      .get("theme")
      ?.split(",")
      .filter((v) => v !== "") ?? [];
  const requiredTheme =
    params
      .get("required_theme")
      ?.split(",")
      .filter((v) => v !== "") ?? [];
  const condition =
    params
      .get("condition")
      ?.split(",")
      .filter((v) => v !== "") ?? [];
  const requiredCondition =
    params
      .get("required_condition")
      ?.split(",")
      .filter((v) => v !== "") ?? [];

  const searchWordRef = useRef(word);
  const handleChange = (value: string) => {
    searchWordRef.current = value;
  };

  const { InputForm, getValues } = useTermsForm({
    defaultValues: {
      region,
      theme,
      required_theme: requiredTheme,
      condition,
      required_condition: requiredCondition,
    },
  });

  const reSearch = () => {
    const values = getValues();
    const searchWord = searchWordRef.current;
    const query = objectToUrlSearch({ ...values, word: searchWord });
    const url = `${URL_PATH_PARTICIPANT.VOLUNTEER}?${query}`;
    router.push(url);
  };

  const { user } = useAuthContext();

  const { data, loading, refetch } = useQuery(SearchVolunteerQuery, {
    variables: {
      word,
      region,
      theme,
      requiredTheme,
      condition,
      requiredCondition,
      targetStatus: TARGET_STATUSES as unknown as string[],
    },
  });

  const [getTargetStatus] = useLazyQuery(GetTargetStatus);

  const [getFav] = useLazyQuery(GetFavFromSearchedPageQuery);
  const [favSet, setFavSet] = useState(new Set<string>());

  useEffect(() => {
    if (user) {
      getFav({ variables: { uid: user.uid } })
        .then((res) => {
          const favSet = new Set(res.data?.fav.map((v) => v.vid));
          setFavSet(favSet);
        })
        .catch(() => {});
    }
  }, [getFav, user]);

  useEffect(() => {
    if (user) {
      getTargetStatus({ variables: { uid: user.uid } })
        .then((res) => {
          if (res.data) {
            refetch({
              word,
              region,
              theme,
              requiredTheme,
              condition,
              requiredCondition,
              targetStatus: [res.data.targetStatus.name],
            }).catch(() => {});
          }
        })
        .catch(() => {});
    }
  }, [getTargetStatus, refetch, user]);

  const MIN_PAGE = 1;
  const MAX_PAGE =
    Math.floor((data?.result.length ?? 1) / SHOW_ONE_PAGE_ITEMS) + 1;

  const [page, setPage] = useState(MIN_PAGE);
  const handlePageChange = (page: number) => {
    if (page < MIN_PAGE) {
      setPage(MIN_PAGE);
    } else if (page > MAX_PAGE) {
      setPage(MAX_PAGE);
    } else {
      setPage(page);
    }
  };

  if (loading) {
    return (
      <div className="text-center">
        <Spinner />
      </div>
    );
  }

  if (data === undefined) {
    return null;
  }

  const start = (page - 1) * SHOW_ONE_PAGE_ITEMS;
  const end = Math.min(page * SHOW_ONE_PAGE_ITEMS, data.result.length);
  const showVolunteers = data.result.slice(start, end);

  return (
    <Container>
      <Row className="my-3">
        <Col>
          <SearchBar initValue={word} onChange={handleChange} />
        </Col>
        <Col sm="2">
          <Button onClick={reSearch}>再検索</Button>
        </Col>
      </Row>
      <Row className="mb-3">
        <Col>{InputForm}</Col>
      </Row>
      {data.result.length === 0 && (
        <Row className="text-center">
          <Col>
            <h2>該当するボランティアはありません</h2>
          </Col>
        </Row>
      )}
      {showVolunteers.map((volunteer) => (
        <VolunteerItem
          key={volunteer.vid}
          volunteer={volunteer}
          initIsFav={favSet.has(volunteer.vid)}
        />
      ))}

      <Row className="justify-content-md-center mt-5">
        <Col md="auto">
          <Pagination>
            <Pagination.First onClick={() => handlePageChange(MIN_PAGE)} />
            <Pagination.Prev onClick={() => handlePageChange(page - 1)} />
            {page === MAX_PAGE && page > MIN_PAGE + 1 && (
              <Pagination.Item>{page - 2}</Pagination.Item>
            )}
            {page > MIN_PAGE && <Pagination.Item>{page - 1}</Pagination.Item>}
            <Pagination.Item>{page}</Pagination.Item>
            {page < MAX_PAGE && <Pagination.Item>{page + 1}</Pagination.Item>}
            {page === MIN_PAGE && page < MAX_PAGE - 1 && (
              <Pagination.Item>{page + 2}</Pagination.Item>
            )}
            <Pagination.Next onClick={() => handlePageChange(page + 1)} />
            <Pagination.Last onClick={() => handlePageChange(MAX_PAGE)} />
          </Pagination>
        </Col>
      </Row>
    </Container>
  );
};