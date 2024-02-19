"use client";

import { onAuthStateChanged, type User } from "firebase/auth";
import { createContext, useContext, useEffect, useMemo, useState } from "react";

import { auth } from "@/firebaseConfig";

export type AuthContextType = {
  user: User | null;
  initializing: boolean;
  isLogged: boolean;
};

// 初期値
const initValue: AuthContextType = {
  user: null,
  initializing: true,
  isLogged: false,
};

// コンテキストの作成
const AuthContext = createContext(initValue);

// コンテキストを使用するためのカスタムフック
export const useAuthContext = () => useContext(AuthContext);

type Props = {
  children: React.ReactNode;
};

// 認証プロバイダー
export const AuthProvider = ({ children }: Props) => {
  const [user, setUser] = useState<User | null>(null);
  const [initializing, setInitializing] = useState(true);

  // コンテキストの値
  const value: AuthContextType = useMemo(
    () => ({
      user,
      initializing,
      isLogged: !initializing && user !== null,
    }),
    [user, initializing]
  );

  // 認証状態の変更を監視
  useEffect(() => {
    const unsubscribed = onAuthStateChanged(auth, (u) => {
      setUser(u);
      setInitializing(false);
    });

    return () => unsubscribed();
  }, []);

  // プロバイダーコンポーネントのレンダリング
  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
};
