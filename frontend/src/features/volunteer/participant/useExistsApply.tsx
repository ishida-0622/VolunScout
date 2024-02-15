import { useLazyQuery } from "@apollo/client";
import { useEffect } from "react";

import { gql } from "@/__generated__/query";
import { useAuthContext } from "@/contexts/AuthContext";

const ExistsApplyQuery = gql(/* GraphQL */ `
  query existsApply($vid: String!, $uid: String!) {
    result: existsApply(vid: $vid, uid: $uid)
  }
`);

export const useExistsApply = (vid: string) => {
  const { user } = useAuthContext();
  const [query, { data, loading, error }] = useLazyQuery(ExistsApplyQuery);

  useEffect(() => {
    if (user) {
      query({
        variables: { vid, uid: user.uid },
        fetchPolicy: "network-only",
      }).catch(() => {});
    }
  }, [query, user, vid]);

  return { existsApply: data?.result, loading, error };
};
