import { onAuthStateChanged, type User } from "firebase/auth";
import { useRef, useState } from "react";

import { auth } from "@/firebaseConfig";

/**
 * ユーザー情報を取得する
 * @returns user: ユーザー情報
 * @returns isLoading: ユーザー情報が取得中ならtrue, 取得済みならfalse
 */
export const useUser = () => {
  const [user, setUser] = useState<User | null>(null);
  const [isLoading, setIsLoading] = useState<boolean>(true);
  const isFetched = useRef(false);

  onAuthStateChanged(auth, (user) => {
    if (isFetched.current) return;
    setIsLoading(false);
    setUser(user);
    isFetched.current = true;
  });

  return { user, isLoading };
};
