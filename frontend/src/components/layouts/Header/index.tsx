"use client";

import { usePathname, useRouter } from "next/navigation";
import { Container, Image, Navbar } from "react-bootstrap";

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

  const router = useRouter();
  const toHome = () =>
    router.push(
      accountType === "group" ? URL_PATH_GROUP.HOME : URL_PATH_PARTICIPANT.HOME
    );

  const isNoLink = !isLogged || isNoHeaderLink(pathname);
  const isNoIcon = isNoHeaderIcon(pathname);

  return (
    <header className={joinClassnames(styles.header_container, className)}>
      <Navbar expand="lg" className={joinClassnames(className)}>
        <Container>
          <div
            onClick={toHome}
            className="d-flex align-items-center cursor-pointer me-2 w-25"
            role="button"
          >
            <Image
              src={"/icons/banner_color.png"}
              alt="Icon"
              className="d-inline-block align-top h-100 w-auto"
              fluid
            />
          </div>

          {!isNoLink && accountType === "group" && (
            <GroupHeader className="me-2" />
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
