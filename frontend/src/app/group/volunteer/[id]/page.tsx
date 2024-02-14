import { notFound } from "next/navigation";

import { VolunteerDetails } from "@/features/volunteer/group/VolunteerDetails";

const VolunteerPage = ({ params }: { params: { id: string } }) => {
  const { id } = params;
  if (typeof id !== "string") {
    notFound();
  }

  return <VolunteerDetails vid={id} />;
};

export default VolunteerPage;
