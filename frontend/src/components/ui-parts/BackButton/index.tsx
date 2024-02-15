"use client";

import { useRouter } from "next/navigation";
import { Button } from "react-bootstrap";
import { IoCaretBack } from "react-icons/io5";

type Props = {
  className?: string;
};

/**
 * 戻るボタン
 */
export const BackButton = ({ className }: Props) => {
  const router = useRouter();
  const handleOnClick = () => router.back();

  return (
    <Button variant="secondary" className={className} onClick={handleOnClick}>
      <IoCaretBack />
      戻る
    </Button>
  );
};
