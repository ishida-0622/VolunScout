"use client";

import { signOut } from "firebase/auth";
import { useRouter } from "next/navigation";

import { URL_PATH_PARTICIPANT } from "@/consts";
import { auth } from "@/firebaseConfig";

export const useLogout = () => {
  const router = useRouter();

  const logout = () => {
    signOut(auth)
      .then(() => {
        router.push(URL_PATH_PARTICIPANT.HOME);
      })
      .catch(() => {
        alert("ログアウトに失敗しました");
      });
  };

  return { logout };
};
