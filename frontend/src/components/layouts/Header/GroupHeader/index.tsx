import Link from "next/link";

import styles from "./index.module.css";

import { joinClassnames } from "@/components/@joinClassnames";
import { URL_PATH_GROUP } from "@/consts";

type Props = {
  className?: string;
};

export const GroupHeader = ({ className }: Props) => {
  return (
    <div className={joinClassnames(styles.base, className)}>
      <div>
        <Link href={URL_PATH_GROUP.VOLUNTEER_CREATE}>
          <span>ボランティア掲載</span>
        </Link>
      </div>
      <div>
        <Link href={URL_PATH_GROUP.VOLUNTEER}>
          <span>掲載したボランティア</span>
        </Link>
      </div>
    </div>
  );
};
