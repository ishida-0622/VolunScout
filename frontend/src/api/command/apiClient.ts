import {
  Configuration,
  ControllersApplyApi,
  ControllersGroupApi,
  ControllersParticipantApi,
  ControllersReviewApi,
  ControllersScoutApi,
  ControllersVolunteerApi,
} from "@/__generated__/command";

// グループ用APIクライアントを作成
export const apiClientGroup = new ControllersGroupApi(
  new Configuration({
    basePath: process.env.NEXT_PUBLIC_WRITE_API_SERVER_BASE_URL,
  })
);

// 参加者用APIクライアントを作成
export const apiClientParticipant = new ControllersParticipantApi(
  new Configuration({
    basePath: process.env.NEXT_PUBLIC_WRITE_API_SERVER_BASE_URL,
  })
);

// ボランティア用APIクライアントを作成
export const apiClientVolunteer = new ControllersVolunteerApi(
  new Configuration({
    basePath: process.env.NEXT_PUBLIC_WRITE_API_SERVER_BASE_URL,
  })
);

// 応募用APIクライアントを作成
export const apiClientApply = new ControllersApplyApi(
  new Configuration({
    basePath: process.env.NEXT_PUBLIC_WRITE_API_SERVER_BASE_URL,
  })
);

// レビュー用APIクライアントを作成
export const apiClientReview = new ControllersReviewApi(
  new Configuration({
    basePath: process.env.NEXT_PUBLIC_WRITE_API_SERVER_BASE_URL,
  })
);

// スカウト用APIクライアントを作成
export const apiClientScout = new ControllersScoutApi(
  new Configuration({
    basePath: process.env.NEXT_PUBLIC_WRITE_API_SERVER_BASE_URL,
  })
);
