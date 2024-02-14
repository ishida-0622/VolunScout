"use client";

import { notFound } from "next/navigation";

import { useAuthContext } from "@/contexts/AuthContext";
import { UpdateVolunteer } from "@/features/volunteer/group/UpdateVolunteer";

const EditVolunteerPage = ({ params }: { params: { id: string } }) => {
  const { id: vid } = params;
  const { user } = useAuthContext();

  if (typeof vid !== "string") {
    notFound();
  }

  if (!user) {
    return null;
  }

  return <UpdateVolunteer vid={vid} gid={user.uid} />;
};

export default EditVolunteerPage;
