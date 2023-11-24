import Image from "next/image";

import { SignInButton } from "@/features/auth/SignInButton";

export const Header = () => {
  return (
    <header>
      <div>
        <Image src={"/icon.svg"} alt="Icon" width={100} height={100} />
        <h1>VolunScout</h1>
      </div>
      <SignInButton />
    </header>
  );
};
