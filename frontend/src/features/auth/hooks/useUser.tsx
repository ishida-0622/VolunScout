import { onAuthStateChanged, type User } from "firebase/auth";
import { useState } from "react";

import { auth } from "@/firebaseConfig";

export const useUser = () => {
  const [user, setUser] = useState<User | null>(null);
  const [isLoading, setIsLoading] = useState<boolean>(true);

  onAuthStateChanged(auth, (user) => {
    setIsLoading(false);
    setUser(user);
  });

  return { user, isLoading };
};
