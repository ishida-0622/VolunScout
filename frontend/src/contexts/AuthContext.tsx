"use client";

import { onAuthStateChanged, type User } from "firebase/auth";
import { createContext, useContext, useEffect, useMemo, useState } from "react";

import { auth } from "@/firebaseConfig";

export type AuthContextType = {
  user: User | null;
  initializing: boolean;
};

const initValue: AuthContextType = {
  user: null,
  initializing: true,
};

const AuthContext = createContext(initValue);

export const useAuthContext = () => useContext(AuthContext);

type Props = {
  children: React.ReactNode;
};

export const AuthProvider = ({ children }: Props) => {
  const [user, setUser] = useState<User | null>(null);
  const [initializing, setInitializing] = useState(true);

  const value: AuthContextType = useMemo(
    () => ({
      user,
      initializing,
    }),
    [user, initializing]
  );

  useEffect(() => {
    const unsubscribed = onAuthStateChanged(auth, (u) => {
      setUser(u);
      setInitializing(false);
    });

    return () => unsubscribed();
  }, []);

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
};
