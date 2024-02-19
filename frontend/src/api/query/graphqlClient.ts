import { ApolloClient, InMemoryCache } from "@apollo/client";

// GraphQLクライアントを作成
export const graphqlClient = new ApolloClient({
  cache: new InMemoryCache(),
  uri: process.env.NEXT_PUBLIC_READ_API_SERVER_BASE_URL,
});
