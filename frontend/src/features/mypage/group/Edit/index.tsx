"use client";

import { useLazyQuery } from "@apollo/client";
import Image from "next/image";
import { useRouter } from "next/navigation";
import { useEffect } from "react";
import { useForm } from "react-hook-form";

import styles from "./index.module.css";

import type { UpdateGroupAccountRequestBody } from "@/__generated__/command";

import { gql } from "@/__generated__/query";
import { apiClientGroup } from "@/api/command";
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

  const [getGroupAccount, { data, loading, error }] =
    useLazyQuery(GetGroupAccountQuery);

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
      getGroupAccount({ variables: { gid: user.uid } }).catch((e) =>
        console.error(e)
      );
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
      console.error("user is null");
      return;
    }

    const body: UpdateGroupAccountRequestBody = {
      ...getValues(),
      gid: user.uid,
    };

    try {
      await apiClientGroup.updateGroupAccount(body);
    } catch (e) {
      console.error(e);
      return;
    }

    router.push(URL_PATH_GROUP.ACCOUNT);
  };

  if (loading || data === undefined) {
    console.warn("loading or data is undefined");
    return null;
  }

  if (error) {
    console.error(error);
    return null;
  }

  return (
    <div>
      <div>
        <Image
          src={user?.photoURL ?? ""}
          alt="User icon"
          width={100}
          height={100}
          className={styles.user_icon}
        />
      </div>
      <form onSubmit={handleSubmit(submit)}>
        <div>
          <label className="form-label">
            <span>団体名</span>
            <span>：</span>
            <input
              id="name"
              className="form-control"
              type="text"
              {...register("name", { required: true })}
            />
          </label>
        </div>
        <div>
          <label className="form-label">
            <span>団体名（フリガナ）</span>
            <span>：</span>
            <input
              id="furigana"
              className="form-control"
              type="text"
              {...register("furigana", { required: true })}
            />
          </label>
        </div>
        <div>
          <label className="form-label">
            <span>代表者名</span>
            <span>：</span>
            <input
              id="representativeName"
              className="form-control"
              type="text"
              {...register("representative_name", { required: true })}
            />
          </label>
        </div>
        <div>
          <label htmlFor="representativeFurigana" className="form-label">
            <span>代表者名（フリガナ）</span>
            <span>：</span>
            <input
              id="representativeFurigana"
              className="form-control"
              type="text"
              {...register("representative_furigana", { required: true })}
            />
          </label>
        </div>
        <div>
          <label className="form-label">
            <span>所在地</span>
            <span>：</span>
            <input
              id="address"
              className="form-control"
              type="text"
              {...register("address", { required: true })}
            />
          </label>
        </div>
        <div>
          <label className="form-label">
            <span>電話番号</span>
            <span>：</span>
            <input
              id="phone"
              className="form-control"
              type="text"
              {...register("phone", { required: true })}
            />
          </label>
        </div>
        <div>
          <label className="form-label">
            紹介メッセージ
            <textarea
              id="contents"
              className="form-control"
              {...register("contents", { required: true })}
            />
          </label>
        </div>
        <div>
          <button type="button" className="btn btn-danger" onClick={back}>
            キャンセル
          </button>
          <button type="submit" className="btn btn-primary">
            更新
          </button>
        </div>
      </form>
    </div>
  );
};
