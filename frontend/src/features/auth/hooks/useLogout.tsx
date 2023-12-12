import { signOut } from "firebase/auth";
import { useRouter } from "next/navigation";

import { auth } from "@/firebaseConfig";

export const useLogout = () => {
  const router = useRouter();

  const logout = () => {
    signOut(auth)
      .then(() => {
        router.push("/");
      })
      .catch((error) => {
        console.error(error);
        alert("ログアウトに失敗しました");
      });
  };

  return { logout };
};
