"use client";

import { IconContext } from "react-icons";

export const IconConfig = ({ children }: { children: React.ReactNode }) => {
  return (
    <IconContext.Provider
      value={{ size: "24px", style: { verticalAlign: "top" } }}
    >
      {children}
    </IconContext.Provider>
  );
};
