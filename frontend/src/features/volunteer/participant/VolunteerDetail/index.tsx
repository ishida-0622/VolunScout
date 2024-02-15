"use client";

import { useLazyQuery, useQuery } from "@apollo/client";
import Link from "next/link";
import { notFound, useRouter } from "next/navigation";
import { useEffect } from "react";
import {
  Badge,
  Button,
  Carousel,
  Col,
  Container,
  Image,
  Row,
  Spinner,
} from "react-bootstrap";

import { useExistsApply } from "../useExistsApply";

import { gql } from "@/__generated__/query";
import { BackButton } from "@/components/ui-parts/BackButton";
import { URL_PATH_PARTICIPANT } from "@/consts";
import { formatDateTime } from "@/utils/formatDateTime";

type Props = {
  vid: string;
};

const GetVolunteerDetailQuery = gql(/* GraphQL */ `
  query getVolunteerDetail($vid: String!) {
    volunteer: getVolunteerById(vid: $vid) {
      vid
      gid
      title
      message
      overview
      recruitedNum
      place
      reward
      startAt
      finishAt
      regions
      themes
      requiredThemes
      conditions
      requiredConditions
      targetStatus
      photoUrls
    }
  }
`);

export const GetGroupNameQuery = gql(/* GraphQL */ `
  query getGroupName($gid: String!) {
    group: getGroupAccount(gid: $gid) {
      name
    }
  }
`);

export const VolunteerDetail = ({ vid }: Props) => {
  const router = useRouter();
  const toApply = () => {
    router.push(URL_PATH_PARTICIPANT.APPLY(vid));
  };

  const { existsApply, loading: existsApplyLoading } = useExistsApply(vid);

  const { data, loading, error } = useQuery(GetVolunteerDetailQuery, {
    variables: { vid },
    ssr: true,
  });

  const [getGroupAccount, { data: groupData }] =
    useLazyQuery(GetGroupNameQuery);

  useEffect(() => {
    if (data) {
      getGroupAccount({ variables: { gid: data.volunteer.gid } }).catch(
        () => {}
      );
    }
  }, [data, getGroupAccount]);

  if (loading) {
    return <Spinner />;
  }

  if (!data || error) {
    notFound();
  }

  const volunteer = data.volunteer;
  const group = groupData?.group;

  return (
    <Container>
      <Row className="align-items-center my-3">
        <Col sm="2">
          <BackButton />
        </Col>
        <Col sm="8">
          <h1>{volunteer.title}</h1>
        </Col>
        <Col sm="2">
          {existsApply ? (
            <Button variant="secondary" size="lg" disabled>
              応募済み
            </Button>
          ) : (
            <Button size="lg" onClick={toApply} disabled={existsApplyLoading}>
              応募する
            </Button>
          )}
        </Col>
      </Row>
      <Row className="mb-3">
        <Carousel>
          {volunteer.photoUrls.map((url) => (
            <Carousel.Item key={url}>
              <Image className="w-50 m-auto" src={url} alt="volunteer photo" />
            </Carousel.Item>
          ))}
        </Carousel>
      </Row>
      <Row className="mb-2">
        <Col sm="2">掲載団体</Col>
        <Col sm="10">
          <Link
            href={URL_PATH_PARTICIPANT.GROUP_ACCOUNT_DETAIL(data.volunteer.gid)}
          >
            {group?.name}
          </Link>
        </Col>
      </Row>
      <Row className="mb-2">
        <Col sm="2">開催場所</Col>
        <Col sm="10">{volunteer.place}</Col>
      </Row>
      <Row className="mb-2">
        <Col sm="2">開催日時</Col>
        <Col sm="10">
          {formatDateTime(volunteer.startAt)} ~{" "}
          {formatDateTime(volunteer.finishAt)}
        </Col>
      </Row>
      <Row className="mb-2">
        <Col sm="2">募集人数</Col>
        <Col sm="10">{volunteer.recruitedNum}人</Col>
      </Row>
      <Row className="mb-2">
        <Col sm="2">報酬</Col>
        <Col sm="10">{volunteer.reward}</Col>
      </Row>
      <Row className="mb-3">
        <Col sm="2">募集対象</Col>
        <Col sm="10">
          {volunteer.targetStatus.map((status) => (
            <Badge key={status} className="mx-2">
              {status}
            </Badge>
          ))}
        </Col>
      </Row>
      <Row className="mb-2">
        <Col sm="2">概要</Col>
        <Col sm="10">{volunteer.overview}</Col>
      </Row>
      <Row className="mb-3">
        <Col sm="2">メッセージ</Col>
        <Col sm="10">{volunteer.message}</Col>
      </Row>
      <Row className="mb-3">
        <Col>
          {volunteer.requiredThemes.concat(volunteer.themes).map((theme) => (
            <Badge key={theme} className="mx-2">
              {theme}
            </Badge>
          ))}
        </Col>
      </Row>
      <Row className="mb-5">
        <Col>
          {volunteer.requiredConditions
            .concat(volunteer.conditions)
            .map((condition) => (
              <Badge key={condition} className="mx-2">
                {condition}
              </Badge>
            ))}
        </Col>
      </Row>
      <div className="mb-3 d-flex">
        {existsApply ? (
          <Button
            variant="secondary"
            size="lg"
            className="w-50 m-auto"
            disabled
          >
            応募済み
          </Button>
        ) : (
          <Button
            size="lg"
            className="w-50 m-auto"
            onClick={toApply}
            disabled={existsApplyLoading}
          >
            応募する
          </Button>
        )}
      </div>
    </Container>
  );
};
