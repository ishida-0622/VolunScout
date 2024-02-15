"use client";

import { useLazyQuery, useQuery } from "@apollo/client";
import { notFound, useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import { Button, Col, Container, Form, Row, Spinner } from "react-bootstrap";

import { GetGroupNameQuery } from "../VolunteerDetail";
import { useExistsApply } from "../useExistsApply";

import type { CreateApplyRequestBody } from "@/__generated__/command";

import { gql } from "@/__generated__/query";
import { apiClientApply } from "@/api/command";
import { BackButton } from "@/components/ui-parts/BackButton";
import { URL_PATH_PARTICIPANT } from "@/consts";
import { useAuthContext } from "@/contexts/AuthContext";
import { ageCalc } from "@/utils/ageCalc";
import { formatDate } from "@/utils/formatDate";
import { formatDateTime } from "@/utils/formatDateTime";
import { formatPhone } from "@/utils/formatPhone";
import { numberToGender } from "@/utils/numberToGender";

type Props = {
  vid: string;
};

const GetVolunteerFromApplyPageQuery = gql(/* GraphQL */ `
  query getVolunteerFromApplyPage($vid: String!) {
    volunteer: getVolunteerById(vid: $vid) {
      gid
      title
      place
      startAt
      finishAt
      asGroup
      recruitedNum
    }
  }
`);

const GetParticipantAccountFromApplyPageQuery = gql(/* GraphQL */ `
  query getParticipantAccountFromApplyPage($uid: String!) {
    participant: getParticipantAccount(uid: $uid) {
      name
      gender
      birthday
      phone
    }
  }
`);

export const Apply = ({ vid }: Props) => {
  const { user } = useAuthContext();

  const router = useRouter();

  const { existsApply, loading: existsApplyLoading } = useExistsApply(vid);

  const { data, loading, error } = useQuery(GetVolunteerFromApplyPageQuery, {
    variables: { vid },
  });

  const [getGroupName, { data: groupName }] = useLazyQuery(GetGroupNameQuery);

  const [getParticipantAccount, { data: participant }] = useLazyQuery(
    GetParticipantAccountFromApplyPageQuery
  );

  useEffect(() => {
    if (data) {
      getGroupName({ variables: { gid: data.volunteer.gid } }).catch(() => {});
    }
  }, [data, getGroupName]);

  useEffect(() => {
    if (user) {
      getParticipantAccount({ variables: { uid: user.uid } }).catch(() => {});
    }
  }, [user, getParticipantAccount]);

  const [isGroup, setIsGroup] = useState(false);

  const [members, setMembers] = useState<
    {
      name: string;
      furigana: string;
      gender: number;
      age: number;
    }[]
  >([
    {
      name: "",
      furigana: "",
      gender: 0,
      age: 20,
    },
  ]);

  const addMember = () => {
    if (members.length < (data?.volunteer.recruitedNum ?? Infinity)) {
      setMembers((prev) => [
        ...prev,
        {
          name: "",
          furigana: "",
          gender: 0,
          age: 20,
        },
      ]);
    } else {
      alert("応募人数が上限に達しています");
    }
  };
  const removeMember = (index: number) => {
    if (members.length > 1) {
      setMembers((prev) => prev.filter((_, i) => i !== index));
    }
  };

  const submit = async () => {
    if (existsApply) {
      alert("すでに応募済みです");
      return;
    }
    if (!confirm("応募しますか？")) {
      return;
    }
    if (user === null) {
      return;
    }
    const body: CreateApplyRequestBody = {
      vid,
      uid: user.uid,
      members: isGroup ? members : undefined,
    };

    try {
      await apiClientApply.createApply(body);
      alert("応募が完了しました");
      router.push(URL_PATH_PARTICIPANT.APPLY_LIST);
    } catch (e) {
      alert("応募に失敗しました");
    }
  };

  if (loading) {
    return <Spinner />;
  }

  if (error) {
    notFound();
  }

  if (!(data && participant)) {
    return null;
  }

  const volunteer = data.volunteer;
  const participantInfo = participant.participant;

  return (
    <Container>
      <Row className="my-3">
        <Col sm="2">
          <BackButton />
        </Col>
        <Col sm="8" className="text-center">
          <h1>応募確認</h1>
        </Col>
        <Col sm="2"></Col>
      </Row>
      <Row className="mb-3">
        <Col>
          <h1>{volunteer.title}</h1>
        </Col>
      </Row>
      <Row className="mb-2">
        <Col sm="2">掲載団体</Col>
        <Col>{groupName?.group.name}</Col>
      </Row>
      <Row className="mb-2">
        <Col sm="2">場所</Col>
        <Col>{volunteer.place}</Col>
      </Row>
      <Row className="mb-5">
        <Col sm="2">日時</Col>
        <Col>
          {formatDateTime(volunteer.startAt)} ~{" "}
          {formatDateTime(volunteer.finishAt)}
        </Col>
      </Row>
      <Row className="mb-3">
        <Col className="text-center">
          <h2>あなたの情報</h2>
        </Col>
      </Row>
      <Row className="mb-2">
        <Col sm="2">名前</Col>
        <Col>{participantInfo.name}</Col>
      </Row>
      <Row className="mb-2">
        <Col sm="2">性別</Col>
        <Col>{numberToGender(participantInfo.gender)} </Col>
      </Row>
      <Row className="mb-2">
        <Col sm="2">生年月日</Col>
        <Col>
          {formatDate(participantInfo.birthday)}（
          {ageCalc(participantInfo.birthday)}歳）
        </Col>
      </Row>
      <Row className="mb-5">
        <Col sm="2">電話番号</Col>
        <Col>{formatPhone(participantInfo.phone)}</Col>
      </Row>
      {volunteer.asGroup && (
        <Row className="mb-3">
          <Col>
            <Form.Switch
              id="group-switch"
              label="団体で応募する"
              onChange={() => setIsGroup((prev) => !prev)}
            />
          </Col>
        </Row>
      )}
      {isGroup && (
        <>
          <Row className="text-center mb-3">
            <Col>
              <h2>あなた以外の応募者情報</h2>
            </Col>
          </Row>
          <Row>
            <Col>
              <Form.Label>名前</Form.Label>
            </Col>
            <Col>
              <Form.Label>フリガナ</Form.Label>
            </Col>
            <Col>
              <Form.Label>性別</Form.Label>
            </Col>
            <Col>
              <Form.Label>年齢</Form.Label>
            </Col>
            <Col sm="1" />
          </Row>
          {members.map((member, i) => (
            <Form.Group key={`member_${i}`} className="mb-3">
              <Row>
                <Col>
                  <Form.Control
                    type="text"
                    value={member.name}
                    onChange={(e) => {
                      const newMembers = [...members];
                      newMembers[i].name = e.target.value;
                      setMembers(newMembers);
                    }}
                    required
                  />
                </Col>
                <Col>
                  <Form.Control
                    type="text"
                    value={member.furigana}
                    onChange={(e) => {
                      const newMembers = [...members];
                      newMembers[i].furigana = e.target.value;
                      setMembers(newMembers);
                    }}
                    required
                  />
                </Col>
                <Col>
                  <Form.Select
                    value={member.gender}
                    onChange={(e) => {
                      const newMembers = [...members];
                      newMembers[i].gender = parseInt(e.target.value, 10);
                      setMembers(newMembers);
                    }}
                    required
                  >
                    <option value={-1} disabled selected>
                      選択してください
                    </option>
                    <option value={0}>男</option>
                    <option value={1}>女</option>
                    <option value={2}>その他</option>
                  </Form.Select>
                </Col>
                <Col>
                  <Form.Control
                    type="number"
                    value={member.age}
                    onChange={(e) => {
                      const newMembers = [...members];
                      newMembers[i].age = parseInt(e.target.value, 10);
                      setMembers(newMembers);
                    }}
                    required
                  />
                </Col>
                <Col sm="1">
                  <Button variant="danger" onClick={() => removeMember(i)}>
                    削除
                  </Button>
                </Col>
              </Row>
            </Form.Group>
          ))}
          <Row>
            <Col sm={{ span: 2, offset: 5 }} className="mb-5">
              <Button variant="primary" onClick={addMember} className="w-100">
                メンバーを追加
              </Button>
            </Col>
          </Row>
        </>
      )}
      <Row>
        <Col sm={{ span: 6, offset: 3 }}>
          {existsApply ? (
            <Button variant="secondary" className="w-100" disabled>
              応募済み
            </Button>
          ) : (
            <Button
              variant="primary"
              className="w-100"
              onClick={submit}
              disabled={existsApplyLoading}
            >
              応募する
            </Button>
          )}
        </Col>
      </Row>
    </Container>
  );
};
