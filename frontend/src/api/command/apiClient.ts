import { Configuration, ControllersApi } from "@/__generated__/command";

export const apiClient = new ControllersApi(
  new Configuration({
    basePath: process.env.NEXT_PUBLIC_WRITE_API_SERVER_BASE_URL,
  })
);
