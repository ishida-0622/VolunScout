import { onAuthStateChanged } from "firebase/auth";

import { auth } from "@/firebaseConfig";

export const getUid = () =>
  new Promise<string | undefined>((resolve) => {
    onAuthStateChanged(auth, (user) => {
      resolve(user?.uid);
    });
  });
