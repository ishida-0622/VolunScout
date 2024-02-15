"use client";

import { useLazyQuery } from "@apollo/client";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { useEffect } from "react";
import { Container } from "react-bootstrap";
import { useForm } from "react-hook-form";

import styles from "./index.module.css";

import type { UpdateGroupAccountRequestBody } from "@/__generated__/command";

import { gql } from "@/__generated__/query";
import { apiClientGroup } from "@/api/command";
import { joinClassnames } from "@/components/@joinClassnames";
import { URL_PATH_GROUP } from "@/consts";
import { useAuthContext } from "@/contexts/AuthContext";

const GetGroupAccountQuery = gql(/* GraphQL */ `
  query GetGroupAccount($gid: String!) {
    user: getGroupAccount(gid: $gid) {
      name
      furigana
      representativeName
      representativeFurigana
      phone
      address
      contents
    }
  }
`);

export const EditMyPage = () => {
  const router = useRouter();
  const back = () => {
    router.back();
  };

  const { user } = useAuthContext();

  const [getGroupAccount, { data, loading, error }] = useLazyQuery(
    GetGroupAccountQuery,
    {
      fetchPolicy: "cache-and-network",
    }
  );

  const { register, getValues, handleSubmit, setValue } =
    useForm<UpdateGroupAccountRequestBody>({
      defaultValues: {
        ...data?.user,
        representative_name: data?.user.representativeName,
        representative_furigana: data?.user.representativeFurigana,
      },
    });

  useEffect(() => {
    if (typeof user?.uid === "string") {
      getGroupAccount({ variables: { gid: user.uid } }).catch(() => {});
    }
  }, [getGroupAccount, user?.uid]);

  useEffect(() => {
    if (data?.user) {
      setValue("name", data.user.name);
      setValue("furigana", data.user.furigana);
      setValue("address", data.user.address);
      setValue("phone", data.user.phone);
      setValue("contents", data.user.contents);
      setValue("representative_name", data.user.representativeName);
      setValue("representative_furigana", data.user.representativeFurigana);
    }
  }, [data?.user, setValue]);

  const submit = async () => {
    if (user === null) {
      return;
    }

    const body: UpdateGroupAccountRequestBody = {
      ...getValues(),
      gid: user.uid,
    };

    try {
      await apiClientGroup.updateGroupAccount(body);
    } catch (e) {
      alert("アカウント更新に失敗しました");
      return;
    }

    router.push(URL_PATH_GROUP.ACCOUNT);
  };

  if (loading || data === undefined) {
    return null;
  }

  if (error) {
    return null;
  }

  return (
    <Container className="my-3">
      <form onSubmit={handleSubmit(submit)}>
        <div className="row mb-3">
          <label htmlFor="name" className="col-sm-2 col-form-label mb-3">
            <span>団体名</span>
          </label>
          <div className="col-sm-10">
            <input
              id="name"
              className="form-control"
              type="text"
              {...register("name", { required: true })}
            />
          </div>
          <label htmlFor="furigana" className="col-sm-2 col-form-label mb-3">
            <span>団体名（フリガナ）</span>
          </label>
          <div className="col-sm-10">
            <input
              id="furigana"
              className="form-control"
              type="text"
              {...register("furigana", { required: true })}
            />
          </div>
          <label
            htmlFor="representativeName"
            className="col-sm-2 col-form-label mb-3"
          >
            <span>代表者名</span>
          </label>
          <div className="col-sm-10">
            <input
              id="representativeName"
              className="form-control"
              type="text"
              {...register("representative_name", { required: true })}
            />
          </div>
          <label
            htmlFor="representativeFurigana"
            className="col-sm-2 col-form-label mb-3"
          >
            <span>代表者名（フリガナ）</span>
          </label>
          <div className="col-sm-10">
            <input
              id="representativeFurigana"
              className="form-control"
              type="text"
              {...register("representative_furigana", { required: true })}
            />
          </div>
          <label htmlFor="address" className="col-sm-2 col-form-label mb-3">
            <span>所在地</span>
          </label>
          <div className="col-sm-10">
            <input
              id="address"
              className="form-control"
              type="text"
              {...register("address", { required: true })}
            />
          </div>
          <label htmlFor="phone" className="col-sm-2 col-form-label mb-3">
            <span>電話番号</span>
          </label>
          <div className={"col-sm-10"}>
            <input
              id="phone"
              className="form-control"
              type="text"
              {...register("phone", { required: true })}
            />
          </div>
        </div>
        <div className="text-center">
          <label className="form-label w-100">
            紹介メッセージ
            <textarea
              id="contents"
              {...register("contents", { required: true })}
              className="form-control mt-2"
            />
          </label>
        </div>
        <div className={styles.button_wrapper}>
          <button
            type="button"
            className={joinClassnames("btn btn-danger", styles.button)}
            onClick={back}
          >
            キャンセル
          </button>
          <button
            type="submit"
            className={joinClassnames("btn btn-primary", styles.button)}
          >
            更新
          </button>
        </div>
      </form>
      <div className="text-center">
        <Link href={URL_PATH_GROUP.ACCOUNT_DELETE}>アカウントを削除する→</Link>
      </div>
    </Container>
  );
};
