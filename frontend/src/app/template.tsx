"use client";

import { usePathname } from "next/navigation";

import type { AccountType } from "@/features/auth/types";

import { Header } from "@/components/layouts/Header";
import {
  URL_PATH_GROUP,
  URL_PATH_PARTICIPANT,
  isGroupPath,
  isNoHeaderLink,
} from "@/consts";
import { useUser } from "@/features/auth/hooks/useUser";

const Template = ({ children }: { children: React.ReactNode }) => {
  const pathname = usePathname();
  const { user, isLoading } = useUser();

  const getAccountType = (): AccountType =>
    isGroupPath(pathname) ? "group" : "participant";

  const isNoLink = () => {
    if (isLoading) return true;
    if (user === null) return true;
    isNoHeaderLink(pathname);
  };

  const isNoIcon = () => {
    if (isLoading) return true;
    if (user === null) return true;

    switch (pathname) {
      case URL_PATH_PARTICIPANT.SIGN_UP:
      case URL_PATH_GROUP.SIGN_UP:
        return true;
      default:
        return false;
    }
  };

  return (
    <>
      <Header
        accountType={getAccountType()}
        isNoLink={isNoLink()}
        isNoIcon={isNoIcon()}
      />
      {children}
    </>
  );
};

export default Template;
