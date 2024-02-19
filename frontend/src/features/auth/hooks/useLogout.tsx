"use client";

import { signOut } from "firebase/auth";
import { useRouter } from "next/navigation";

import { URL_PATH_PARTICIPANT } from "@/consts";
import { auth } from "@/firebaseConfig";

// ログアウトフック
export const useLogout = () => {
  const router = useRouter();

  /**
   * ログアウト処理を実行し, 成功時には参加者用ホームページへリダイレクト, 失敗時にはアラートを表示する.
   */
  const logout = () => {
    // Firebaseでログアウト処理を実行
    signOut(auth)
      .then(() => {
        // ログアウト成功時には参加者用ホームページへリダイレクト
        router.push(URL_PATH_PARTICIPANT.HOME);
      })
      .catch(() => {
        // ログアウト失敗時にはアラートを表示
        alert("ログアウトに失敗しました");
      });
  };

  return { logout };
};
