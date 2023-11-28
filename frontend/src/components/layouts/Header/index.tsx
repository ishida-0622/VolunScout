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
        className,
      )}
    >
      {/* Flexboxを使用して横並びにする */}
      <div className={styles.headerContent}>
        <div>
          <Image src={"/icon.svg"} alt="Icon" width={100} height={100} />
        </div>
        <div>
          <h1>VolunScout</h1>
        </div>
        <div>
          <SignInButton />
        </div>
      </div>
    </header>
  );
};
