import { notFound } from "next/navigation";

import { FromParticipant } from "@/features/mypage/group/FromParticipant";

const GroupAccount = ({ params }: { params: { id: string } }) => {
  const { id } = params;
  if (typeof id !== "string") {
    notFound();
  }

  return <FromParticipant gid={id} />;
};

export default GroupAccount;
