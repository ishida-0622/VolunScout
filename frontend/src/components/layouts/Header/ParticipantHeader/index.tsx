import Link from "next/link";

import styles from "./index.module.css";

import { joinClassnames } from "@/components/@joinClassnames";
import { URL_PATH_PARTICIPANT } from "@/consts";

type Props = {
  className?: string;
};

export const ParticipantHeader = ({ className }: Props) => {
  return (
    <div className={joinClassnames(styles.base, className)}>
      <div>
        <Link href={URL_PATH_PARTICIPANT.FAVORITE}>
          <span>お気に入り</span>
        </Link>
      </div>
      <div>
        <Link href={URL_PATH_PARTICIPANT.APPLY_LIST}>
          <span>応募したボランティア</span>
        </Link>
      </div>
      <div>
        <Link href={URL_PATH_PARTICIPANT.SCOUT}>
          <span>スカウト</span>
        </Link>
      </div>
    </div>
  );
};
