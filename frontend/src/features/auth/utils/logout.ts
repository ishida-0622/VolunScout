import { signOut } from "firebase/auth";
import Router from "next/router";

import { auth } from "@/firebaseConfig";

export const logout = () => {
  signOut(auth)
    .then(() => {
      Router.push("/").catch((e) => {
        console.error(e);
      });
    })
    .catch((error) => {
      console.error(error);
      alert("ログアウトに失敗しました");
    });
};
