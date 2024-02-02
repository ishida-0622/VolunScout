"use client";

import Image from "next/image";
import Link from "next/link";
import { ListGroup, OverlayTrigger, Popover } from "react-bootstrap";
import { AiOutlineQuestionCircle } from "react-icons/ai";
import { HiOutlineCurrencyYen } from "react-icons/hi2";
import { IoDocumentTextOutline, IoLogOutOutline } from "react-icons/io5";

import styles from "./index.module.css";

import type { AccountType } from "@/features/auth/types";

import { IconConfig } from "@/components/layouts/IconConfig";
import { URL_PATH, URL_PATH_GROUP, URL_PATH_PARTICIPANT } from "@/consts";
import { useAuthContext } from "@/contexts/AuthContext";
import { useLogout } from "@/features/auth/hooks/useLogout";

type Props = {
  accountType: AccountType;
};

export const UserIcon = ({ accountType }: Props) => {
  const { user, initializing } = useAuthContext();
  const { logout } = useLogout();

  const toMyPage = () => {
    if (accountType === "group") {
      return URL_PATH_GROUP.ACCOUNT;
    } else if (accountType === "participant") {
      return URL_PATH_PARTICIPANT.ACCOUNT;
    } else {
      throw new Error("accountType is invalid");
    }
  };

  if (initializing) return null;

  return (
    <IconConfig>
      <div>
        <OverlayTrigger
          trigger={"click"}
          placement="bottom"
          overlay={
            <Popover>
              <ListGroup>
                <ListGroup.Item>
                  <Link href={toMyPage()}>
                    <IoDocumentTextOutline />
                    マイページ
                  </Link>
                </ListGroup.Item>
                <ListGroup.Item>
                  <Link href={URL_PATH.CONTACT}>
                    <AiOutlineQuestionCircle />
                    お問い合わせ
                  </Link>
                </ListGroup.Item>
                <ListGroup.Item>
                  <Link href={URL_PATH.DONATE}>
                    <HiOutlineCurrencyYen />
                    運営への寄付
                  </Link>
                </ListGroup.Item>
                <ListGroup.Item>
                  <span
                    onClick={logout}
                    className={styles.logout}
                    role="button"
                  >
                    <IoLogOutOutline />
                    ログアウト
                  </span>
                </ListGroup.Item>
              </ListGroup>
            </Popover>
          }
        >
          <Image
            src={user?.photoURL ?? "/icon.svg"}
            alt="user icon"
            // TODO: 画像のサイズを調整する
            width={80}
            height={80}
            className={styles.base}
          />
        </OverlayTrigger>
      </div>
    </IconConfig>
  );
};
