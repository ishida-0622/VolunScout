import Image from "next/image";
import Link from "next/link";

import { GroupHeader } from "./GroupHeader";
import { ParticipantHeader } from "./ParticipantHeader";
import { UserIconOrSignInButton } from "./UserIconOrSignInButton";
import styles from "./index.module.css";

import type { AccountType } from "@/features/auth/types";

import { joinClassnames } from "@/components/@joinClassnames";
import { URL_PATH_GROUP, URL_PATH_PARTICIPANT } from "@/consts";

type Props = {
  accountType: AccountType;
  isNoLink?: boolean;
  isNoIcon?: boolean;
  className?: string;
};

export const Header = ({
  accountType,
  className,
  isNoLink = false,
  isNoIcon = false,
}: Props) => {
  return (
    <header
      className={joinClassnames(
        styles.base,
        accountType === "participant" ? styles.participant : styles.group,
        className
      )}
    >
      <div>
        <Link
          href={
            accountType === "group"
              ? URL_PATH_GROUP.HOME
              : URL_PATH_PARTICIPANT.HOME
          }
        >
          <Image src={"/icon.svg"} alt="Icon" width={100} height={100} />
        </Link>
      </div>
      <div>
        <h1>VolunScout</h1>
      </div>
      {isNoLink ? null : (
        <>
          {accountType === "group" && <GroupHeader />}
          {accountType === "participant" && <ParticipantHeader />}
        </>
      )}
      {isNoIcon ? null : (
        <div>
          <UserIconOrSignInButton accountType={accountType} />
        </div>
      )}
    </header>
  );
};
