"use client";

import { useRouter } from "next/navigation";

import styles from "./index.module.css";

import type { GetVolunteerByGidQuery } from "@/__generated__/query/graphql";
import type { AccountType } from "@/features/auth/types";

import { joinClassnames } from "@/components/@joinClassnames";
import { URL_PATH_GROUP, URL_PATH_PARTICIPANT } from "@/consts";
import { formatDate } from "@/utils/formatDate";

type Props = {
  volunteer: GetVolunteerByGidQuery["volunteers"][number];
  accountType: AccountType;
};

export const VolunteerItem = ({ volunteer, accountType }: Props) => {
  const router = useRouter();
  const toVolunteer = () => {
    switch (accountType) {
      case "participant":
        router.push(URL_PATH_PARTICIPANT.VOLUNTEER_DETAIL(volunteer.vid));
        break;
      case "group":
        router.push(URL_PATH_GROUP.VOLUNTEER_DETAIL(volunteer.vid));
        break;
      default:
        throw new Error("Invalid account type");
    }
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
