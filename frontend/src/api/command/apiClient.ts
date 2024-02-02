import {
  Configuration,
  ControllersApplyApi,
  ControllersGroupApi,
  ControllersParticipantApi,
  ControllersReviewApi,
  ControllersVolunteerApi,
} from "@/__generated__/command";

export const apiClientGroup = new ControllersGroupApi(
  new Configuration({
    basePath: process.env.NEXT_PUBLIC_WRITE_API_SERVER_BASE_URL,
  })
);

export const apiClientParticipant = new ControllersParticipantApi(
  new Configuration({
    basePath: process.env.NEXT_PUBLIC_WRITE_API_SERVER_BASE_URL,
  })
);

export const apiClientVolunteer = new ControllersVolunteerApi(
  new Configuration({
    basePath: process.env.NEXT_PUBLIC_WRITE_API_SERVER_BASE_URL,
  })
);

export const apiClientApply = new ControllersApplyApi(
  new Configuration({
    basePath: process.env.NEXT_PUBLIC_WRITE_API_SERVER_BASE_URL,
  })
);

export const apiClientReview = new ControllersReviewApi(
  new Configuration({
    basePath: process.env.NEXT_PUBLIC_WRITE_API_SERVER_BASE_URL,
  })
);
