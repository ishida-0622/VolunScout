"use client";

import { useLazyQuery } from "@apollo/client";
import { useRouter } from "next/navigation";
import { useEffect, useRef, useState } from "react";
import { Button, Col, Container, Row, Spinner } from "react-bootstrap";

import { useTermsForm } from "../../useTermsForm";

import { gql } from "@/__generated__/query";
import { SearchBar } from "@/components/ui-parts/SearchBar/index";
import { URL_PATH_PARTICIPANT } from "@/consts";
import { useAuthContext } from "@/contexts/AuthContext";
import { objectToUrlSearch } from "@/utils/objectToUrlSearch";

const GetParticipantElementsQuery = gql(/* GraphQL */ `
  query GetParticipantElements($uid: String!) {
    regions: getParticipantRegions(uid: $uid) {
      name
    }
    themes: getParticipantThemes(uid: $uid) {
      name
      isRequired
    }
    conditions: getParticipantConditions(uid: $uid) {
      name
      isRequired
    }
  }
`);

export const SearchVolunteer = () => {
  const router = useRouter();

  const { user } = useAuthContext();

  const [getParticipantElements] = useLazyQuery(GetParticipantElementsQuery);

  const [isLoading, setIsLoading] = useState(false);

  const { InputForm, getValues, setFormValues } = useTermsForm({
    isOpen: true,
  });

  const searchWord = useRef("");
  const handleChange = (value: string) => {
    searchWord.current = value;
  };

  const handleSubmit = () => {
    const values = getValues();
    const query = objectToUrlSearch({ ...values, word: searchWord.current });
    const url = `${URL_PATH_PARTICIPANT.VOLUNTEER}?${query}`;
    router.push(url);
    setIsLoading(true);
  };

  useEffect(() => {
    if (user) {
      getParticipantElements({ variables: { uid: user.uid } })
        .then((res) => {
          if (res.data) {
            const { regions, themes, conditions } = res.data;
            setFormValues({
              region: regions.map((v) => v.name),
              theme: themes.filter((v) => !v.isRequired).map((v) => v.name),
              required_theme: themes
                .filter((v) => v.isRequired)
                .map((v) => v.name),
              condition: conditions
                .filter((v) => !v.isRequired)
                .map((v) => v.name),
              required_condition: conditions
                .filter((v) => v.isRequired)
                .map((v) => v.name),
            });
          }
        })
        .catch(() => {});
    }
  }, [getParticipantElements, user]);

  return (
    <Container>
      <Row className="my-3">
        <Col>
          <SearchBar onChange={handleChange} placeholder="ボランティアを検索" />
        </Col>
        <Col sm="2">
          {isLoading ? (
            <Button className="w-100" disabled>
              <Spinner
                as="span"
                animation="grow"
                size="sm"
                role="status"
                aria-hidden="true"
              />
              検索中...
            </Button>
          ) : (
            <Button onClick={handleSubmit} className="w-100">
              検索
            </Button>
          )}
        </Col>
      </Row>
      {InputForm}
    </Container>
  );
};
