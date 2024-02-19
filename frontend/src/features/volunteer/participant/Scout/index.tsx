"use client";

import { useLazyQuery } from "@apollo/client";
import { useEffect, useState } from "react";
import { Col, Container, Row, Spinner } from "react-bootstrap";

import { ScoutItem } from "./ScoutItem";
import { ScoutModal } from "./ScoutModal";

import { gql } from "@/__generated__/query";
import { useAuthContext } from "@/contexts/AuthContext";

// スカウトを取得するクエリ
const GetScoutQuery = gql(/* GraphQL */ `
  query GetScoutByUid($uid: String!) {
    scouts: getScoutByUid(uid: $uid) {
      sid
      vid
      message
    }
  }
`);

export const Scout = () => {
  const { user } = useAuthContext();
  const [getScout, { data, loading, error, refetch }] = useLazyQuery(
    GetScoutQuery,
    {
      fetchPolicy: "cache-and-network",
    }
  );

  // モーダルの表示状態
  const [showModal, setShowModal] = useState(false);
  const [vid, setVid] = useState("");

  const openModal = (vid: string) => {
    setVid(vid);
    setShowModal(true);
  };
  const closeModal = () => {
    setShowModal(false);
  };

  useEffect(() => {
    if (user) {
      getScout({ variables: { uid: user.uid } }).catch(() => {});
    }
  }, [getScout, user]);

  if (loading) {
    return <Spinner />;
  }

  if (error) {
    return null;
  }

  if (!data) {
    return null;
  }

  return (
    <Container className="my-5">
      {data.scouts.length === 0 && (
        <Row className="text-center">
          <Col>
            <h2>スカウトはありません</h2>
          </Col>
        </Row>
      )}
      {data.scouts.map((scout) => (
        <ScoutItem
          key={scout.sid}
          scout={scout}
          onPreview={() => {
            openModal(scout.vid);
          }}
          refetch={refetch}
        />
      ))}
      <ScoutModal vid={vid} show={showModal} onHide={closeModal} />
    </Container>
  );
};
