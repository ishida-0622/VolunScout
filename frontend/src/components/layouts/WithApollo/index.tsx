"use client";

import { ApolloProvider } from "@apollo/client";

import { graphqlClient } from "@/api/query";

export const WithApollo = ({ children }: { children: React.ReactNode }) => {
  return <ApolloProvider client={graphqlClient}>{children}</ApolloProvider>;
};
