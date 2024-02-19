"use client";

import { UserIcon } from "../UserIcon";

import type { AccountType } from "@/features/auth/types";

import { useAuthContext } from "@/contexts/AuthContext";
import { SignInButton } from "@/features/auth/SignInButton";

type Props = {
  accountType: AccountType;
};

/**
 * ユーザーのログイン状態に応じて, ユーザーアイコンまたはサインインボタンを表示する
 *
 * @param accountType アカウントの種類
 * @returns ユーザーアイコンまたはサインインボタン
 */
export const UserIconOrSignInButton = ({ accountType }: Props) => {
  const { user, initializing } = useAuthContext();

  if (initializing) {
    return null;
  }

  return user ? <UserIcon accountType={accountType} /> : <SignInButton />;
};
