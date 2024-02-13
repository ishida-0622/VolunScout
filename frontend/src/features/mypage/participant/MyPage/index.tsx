import Link from "next/link";

import { Registration } from "./Registration";
import { UserInfo } from "./UserInfo";

import { URL_PATH_PARTICIPANT } from "@/consts";

export const MyPage = () => {
  return (
    <div>
      <UserInfo />
      <Registration />
      <div className="text-center">
        <Link href={URL_PATH_PARTICIPANT.ACCOUNT_DELETE}>
          アカウントを削除する→
        </Link>
      </div>
    </div>
  );
};
