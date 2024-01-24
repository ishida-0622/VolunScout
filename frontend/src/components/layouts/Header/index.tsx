// "use client" 以下のコード
"use client";

import Image from "next/image";
import Link from "next/link";
import { usePathname } from "next/navigation";
import { Container, Navbar } from "react-bootstrap";

import { GroupHeader } from "./GroupHeader";
import { ParticipantHeader } from "./ParticipantHeader";
import { UserIconOrSignInButton } from "./UserIconOrSignInButton";
import styles from "./index.module.css";

import type { AccountType } from "@/features/auth/types";

import { joinClassnames } from "@/components/@joinClassnames";
import {
  URL_PATH_GROUP,
  URL_PATH_PARTICIPANT,
  isNoHeaderIcon,
  isNoHeaderLink,
} from "@/consts";
import { useAuthContext } from "@/contexts/AuthContext";
import { getAccountTypeFromPath } from "@/features/auth/utils/getAccountType";

type Props = {
  className?: string;
};

export const Header = ({ className }: Props) => {
  const pathname = usePathname();
  const { isLogged } = useAuthContext();

  const accountType: AccountType = getAccountTypeFromPath(pathname);

  const isNoLink = !isLogged || isNoHeaderLink(pathname);
  const isNoIcon = isNoHeaderIcon(pathname);

  const linkHref =
    accountType === "group" ? URL_PATH_GROUP.HOME : URL_PATH_PARTICIPANT.HOME;

  return (
    <header className={joinClassnames(styles.header_container, className)}>
      <Navbar expand="lg" className={joinClassnames(className)}>
        <Container>
          {/* Icon and Text */}
          <div
            onClick={() => {
              window.location.href = linkHref;
            }}
            className={joinClassnames(
              "headerItem",
              "d-flex align-items-center cursor-pointer me-2",
            )}
          >
            <Image
              src={"/icon.svg"}
              alt="Icon"
              width={80} // Adjust the width as needed for a larger icon
              height={80} // Adjust the height as needed for a larger icon
              className="d-inline-block align-top"
            />
            <span className="ms-2 fs-1">VolunScout</span>
          </div>

          {!isNoLink && accountType === "group" && (
            <GroupHeader className="headerItem me-2" />
          )}
          {!isNoLink && accountType === "participant" && (
            <ParticipantHeader className="me-2" />
          )}
          {!isNoIcon && <UserIconOrSignInButton accountType={accountType} />}
        </Container>
      </Navbar>
    </header>
  );
};
