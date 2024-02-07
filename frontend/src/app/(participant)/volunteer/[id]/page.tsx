import { notFound } from "next/navigation";

import { VolunteerDetail } from "@/features/volunteer/participant/VolunteerDetail";

const VolunteerPage = ({ params }: { params: { id: string } }) => {
  const { id } = params;
  if (typeof id !== "string") {
    notFound();
  }

  return <VolunteerDetail vid={id} />;
};

export default VolunteerPage;
