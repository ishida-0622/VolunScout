import type { CodegenConfig } from "@graphql-codegen/cli";

const config: CodegenConfig = {
  schema: "./schemas/schema.graphql",
  documents: ["src/**/*.tsx", "src/**/*.ts"],
  generates: {
    "./src/__generated__/query/": {
      preset: "client",
      plugins: [],
      presetConfig: {
        gqlTagName: "gql",
      },
      config: {
        strictScalars: true,
        scalars: {
          NaiveDateTime: "Date",
          NaiveDate: "Date",
        },
      },
    },
  },
  ignoreNoDocuments: true,
};

export default config;
