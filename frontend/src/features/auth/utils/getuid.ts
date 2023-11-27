import { onAuthStateChanged } from "firebase/auth";

import { auth } from "@/firebaseConfig";

export const getuid = () =>
  new Promise<string | undefined>((resolve) => {
    onAuthStateChanged(auth, (user) => {
      resolve(user?.uid);
    });
  });
