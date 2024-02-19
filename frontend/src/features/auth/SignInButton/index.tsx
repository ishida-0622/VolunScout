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

// 参加者アカウントが存在するか確認するクエリ
const ExistsParticipantAccountQuery = gql(/* GraphQL */ `
  query ExistsParticipantAccount($uid: String!) {
    result: existsParticipantAccount(uid: $uid)
  }
`);

// 団体アカウントが存在するか確認するクエリ
const ExistsGroupAccountQuery = gql(/* GraphQL */ `
  query ExistsGroupAccount($gid: String!) {
    result: existsGroupAccount(gid: $gid)
  }
`);

// サインインボタンコンポーネント
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

  // 参加者アカウントが存在するかを確認するクエリの実行フック
  const [existsParticipantAccount] = useLazyQuery(
    ExistsParticipantAccountQuery,
    {
      fetchPolicy: "network-only",
    }
  );

  // 団体アカウントが存在するかを確認するクエリの実行フック
  const [existsGroupAccount] = useLazyQuery(ExistsGroupAccountQuery, {
    fetchPolicy: "network-only",
  });

  // Googleサインイン処理
  const handleGoogleSignIn = async () => {
    try {
      // Googleサインインを実行
      const result = await signInWithPopup(auth, provider);
      // サインイン結果からクレデンシャルを取得
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

      // アカウントが存在しない場合はサインアップページに遷移
      if (data.result === false) {
        router.push(
          isGroup.current
            ? URL_PATH_GROUP.SIGN_UP
            : URL_PATH_PARTICIPANT.SIGN_UP
        );
      } else {
        // アカウントが存在する場合はホームページに遷移
        router.push(
          isGroup.current ? URL_PATH_GROUP.HOME : URL_PATH_PARTICIPANT.HOME
        );
      }
    } catch (error) {
      if (error instanceof FirebaseError) {
        if (!(error.code === "auth/popup-closed-by-user")) {
          // サインインエラーが発生した場合はアラートを表示
          alert("ログインに失敗しました");
        }
      } else {
        alert("ログインに失敗しました");
      }
    }
  };

  return (
    <>
      {/* ログイン/会員登録ボタン */}
      <Button variant="none" onClick={openModal}>
        ログイン/会員登録
      </Button>
      {/* ログイン/会員登録モーダル */}
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
          {/* Googleサインインボタン */}
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
          {/* 団体アカウントかどうかを選択するスイッチ */}
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
