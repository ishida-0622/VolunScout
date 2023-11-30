/**
 * URLパス（共通）
 */
export const URL_PATH = {
  CONTACT: "/contact",
} as const;

/**
 * URLパス（参加者）
 */
export const URL_PATH_PARTICIPANT = {
  HOME: "/",
  SIGN_UP: "/signup",
  ACCOUNT: "/account",
  ACCOUNT_DETAIL: (id: string) => `/account/${id}`,
  ACCOUNT_EDIT: "/account/edit",
  ACCOUNT_DELETE: "/account/delete",
  FAVORITE: "/favorite",
  SCOUT: "/scout",
  SCOUT_DETAIL: (id: string) => `/scout/${id}`,
  VOLUNTEER: "/volunteer",
  VOLUNTEER_DETAIL: (id: string) => `/volunteer/${id}`,
  APPLY: (id: string) => `/volunteer/${id}/apply`,
  APPLY_LIST: "/apply",
} as const;

/**
 * URLパス（団体）
 */
export const URL_PATH_GROUP = {
  HOME: "/group",
  SIGN_UP: "/group/signup",
  ACCOUNT: "/group/account",
  ACCOUNT_DETAIL: (id: string) => `/group/account/${id}`,
  ACCOUNT_EDIT: "/group/account/edit",
  ACCOUNT_DELETE: "/group/account/delete",
  VOLUNTEER: "/group/volunteer",
  VOLUNTEER_DETAIL: (id: string) => `/group/volunteer/${id}`,
  VOLUNTEER_CREATE: "/group/volunteer/create",
  VOLUNTEER_EDIT: (id: string) => `/group/volunteer/${id}/edit`,
} as const;
