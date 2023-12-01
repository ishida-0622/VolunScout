"use client";

import { UserIcon } from "../UserIcon";

import type { AccountType } from "@/features/auth/types";

import { SignInButton } from "@/features/auth/SignInButton";
import { useUser } from "@/features/auth/hooks/useUser";

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
  const { user, isLoading } = useUser();

  if (isLoading) return null;

  return user ? <UserIcon accountType={accountType} /> : <SignInButton />;
};
