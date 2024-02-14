"use client";

import { useLazyQuery } from "@apollo/client";
import { FirebaseError } from "firebase/app";
import { GoogleAuthProvider, signInWithPopup } from "firebase/auth";
import { useRouter } from "next/navigation";
import { useRef, useState } from "react";
import { Button, Col, Form, Image, Modal, Row } from "react-bootstrap";

import { gql } from "@/__generated__/query";
import { URL_PATH_GROUP, URL_PATH_PARTICIPANT } from "@/consts";
import { auth } from "@/firebaseConfig";

const ExistsParticipantAccountQuery = gql(/* GraphQL */ `
  query ExistsParticipantAccount($uid: String!) {
    result: existsParticipantAccount(uid: $uid)
  }
`);

const ExistsGroupAccountQuery = gql(/* GraphQL */ `
  query ExistsGroupAccount($gid: String!) {
    result: existsGroupAccount(gid: $gid)
  }
`);

export const SignInButton = () => {
  const router = useRouter();
  const provider = new GoogleAuthProvider();

  const [isModalOpen, setIsModalOpen] = useState(false);
  const openModal = () => setIsModalOpen(true);
  const closeModal = () => setIsModalOpen(false);

  const isGroup = useRef(false);
  const handleChangeIsGroup = (e: React.ChangeEvent<HTMLInputElement>) => {
    const checked = e.target.checked;
    isGroup.current = checked;
  };

  const [existsParticipantAccount] = useLazyQuery(
    ExistsParticipantAccountQuery,
    {
      fetchPolicy: "network-only",
    }
  );
  const [existsGroupAccount] = useLazyQuery(ExistsGroupAccountQuery, {
    fetchPolicy: "network-only",
  });

  const handleGoogleSignIn = async () => {
    try {
      const result = await signInWithPopup(auth, provider);
      const credential = GoogleAuthProvider.credentialFromResult(result);

      if (credential === null) {
        throw new Error("credential is null");
      }

      const user = result.user;

      if (user === null) {
        throw new Error("user is null");
      }

      const uid = user.uid;

      const { data } = isGroup.current
        ? await existsGroupAccount({ variables: { gid: uid } })
        : await existsParticipantAccount({ variables: { uid } });

      if (data === undefined) {
        throw new Error("data is undefined");
      }

      // 会員登録がされていない場合はサインアップページに遷移
      if (data.result === false) {
        router.push(
          isGroup.current
            ? URL_PATH_GROUP.SIGN_UP
            : URL_PATH_PARTICIPANT.SIGN_UP
        );
      } else {
        router.push(
          isGroup.current ? URL_PATH_GROUP.HOME : URL_PATH_PARTICIPANT.HOME
        );
      }
    } catch (error) {
      if (error instanceof FirebaseError) {
        if (!(error.code === "auth/popup-closed-by-user")) {
          alert("ログインに失敗しました");
        }
      } else {
        alert("ログインに失敗しました");
      }
    }
  };

  return (
    <>
      <Button variant="none" onClick={openModal}>
        ログイン/会員登録
      </Button>
      <Modal
        show={isModalOpen}
        onHide={closeModal}
        size="lg"
        aria-labelledby="login-modal"
        className="m-auto"
        centered
      >
        <Modal.Header closeButton>
          <Modal.Title id="login-modal" className="px-3">
            <h1>Sign up / Log in</h1>
          </Modal.Title>
        </Modal.Header>
        <Modal.Body>
          <Row className="mb-3 w-50 m-auto">
            <Col>
              <Image
                src="/auth/web_neutral_sq_SI.svg"
                alt="Google Auth Logo"
                className="w-100"
                role="button"
                onClick={() => {
                  handleGoogleSignIn()
                    .then(() => closeModal())
                    .catch(() => {});
                }}
              />
            </Col>
          </Row>
          <Row className="w-75 m-auto">
            <Col>
              <Form.Switch
                label="ボランティアを募集する「団体」としてログイン・会員登録する"
                onChange={handleChangeIsGroup}
                initialState={isGroup.current}
              />
            </Col>
          </Row>
        </Modal.Body>
      </Modal>
    </>
  );
};
