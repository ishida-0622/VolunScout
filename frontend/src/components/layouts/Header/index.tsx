import Image from "next/image";

import styles from "./index.module.css";

import { joinClassnames } from "@/components/@joinClassnames";
import { SignInButton } from "@/features/auth/SignInButton";

type Props = {
  accountType: "participant" | "group";
  className?: string;
};

export const Header = ({ accountType, className }: Props) => {
  return (
    <header
      className={joinClassnames(
        styles.base,
        accountType === "participant" ? styles.participant : styles.group,
        className
      )}
    >
      <div>
        <Image src={"/icon.svg"} alt="Icon" width={100} height={100} />
        <h1>VolunScout</h1>
      </div>
      <SignInButton />
    </header>
  );
};
