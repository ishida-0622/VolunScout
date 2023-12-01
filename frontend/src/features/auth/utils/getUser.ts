import { onAuthStateChanged, type User } from "firebase/auth";

import { auth } from "@/firebaseConfig";

export const getUser = (): Promise<User | null> =>
  new Promise((resolve) => {
    onAuthStateChanged(auth, (user) => {
      resolve(user);
    });
  });
