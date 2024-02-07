import { notFound } from "next/navigation";

import { Apply } from "@/features/volunteer/participant/Apply";

const VolunteerPage = ({ params }: { params: { id: string } }) => {
  const { id } = params;
  if (typeof id !== "string") {
    notFound();
  }

  return <Apply vid={id} />;
};

export default VolunteerPage;
