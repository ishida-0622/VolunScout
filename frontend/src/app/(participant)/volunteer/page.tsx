"use client";

import { useSearchParams } from "next/navigation";

import { SearchedVolunteer } from "@/features/volunteer/participant/SearchedVolunteer";

const SearchedVolunteerPage = () => {
  const params = useSearchParams();

  return <SearchedVolunteer params={params} />;
};

export default SearchedVolunteerPage;
