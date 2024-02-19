"use client";

import { useRouter } from "next/navigation";

import styles from "./index.module.css";

import type { GetAllVolunteerByGidQuery } from "@/__generated__/query/graphql";

import { joinClassnames } from "@/components/@joinClassnames";
import { URL_PATH_GROUP } from "@/consts";
import { formatDate } from "@/utils/formatDate";

type Props = {
  volunteer: GetAllVolunteerByGidQuery["volunteers"][number];
};

export const VolunteerItem = ({ volunteer }: Props) => {
  const router = useRouter();

  // ボランティア詳細画面に遷移
  const toVolunteer = () => {
    router.push(URL_PATH_GROUP.VOLUNTEER_DETAIL(volunteer.vid));
  };

  return (
    <div
      className={joinClassnames("border border-2 rounded-2", styles.wrapper)}
      onClick={toVolunteer}
    >
      <h2>{volunteer.title}</h2>
      <p>
        <span>場所：</span>
        <span>{volunteer.place}</span>
        &emsp;
        <span>日時：</span>
        <span>{formatDate(volunteer.startAt)}</span>
        &emsp;
        <span>人数：</span>
        <span>{volunteer.recruitedNum}</span>
      </p>
      <p>{volunteer.overview}</p>
    </div>
  );
};
