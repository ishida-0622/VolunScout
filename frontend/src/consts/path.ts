/**
 * URLパス（共通）
 */
export const URL_PATH = {
  CONTACT: "/contact",
  DONATE: "/donate",
  PRIVACY_POLICY: "/privacy-policy",
  TERMS_OF_SERVICE: "/terms-of-service",
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

/**
 * URLパスが参加者のものかどうかを判定する
 *
 * @param path URLパス
 *
 * @returns 参加者のものであればtrue
 */
export const isParticipantPath = (path: string) => {
  const paths = Object.values(URL_PATH_PARTICIPANT).map((p) => {
    if (typeof p === "function") {
      switch (p) {
        case URL_PATH_PARTICIPANT.ACCOUNT_DETAIL:
          // uid(Firebase Auth)は28桁
          return new RegExp(`^${p(".{28}")}$`);
        case URL_PATH_PARTICIPANT.SCOUT_DETAIL:
        case URL_PATH_PARTICIPANT.VOLUNTEER_DETAIL:
        case URL_PATH_PARTICIPANT.APPLY:
          // ulidは26桁
          return new RegExp(`^${p(".{26}")}$`);
        default:
          throw new Error("invalid path");
      }
    }
    return new RegExp(`^${p}$`);
  });
  return paths.some((p) => p.test(path));
};

/**
 * URLパスが団体のものかどうかを判定する
 *
 * @param path URLパス
 *
 * @returns 団体のものであればtrue
 */
export const isGroupPath = (path: string) => {
  const paths = Object.values(URL_PATH_GROUP).map((p) => {
    if (typeof p === "function") {
      switch (p) {
        case URL_PATH_GROUP.ACCOUNT_DETAIL:
          // uid(Firebase Auth)は28桁
          return new RegExp(`^${p(".{28}")}$`);
        case URL_PATH_GROUP.VOLUNTEER_DETAIL:
        case URL_PATH_GROUP.VOLUNTEER_EDIT:
          // ulidは26桁
          return new RegExp(`^${p(".{26}")}$`);
        default:
          throw new Error("invalid path");
      }
    }
    return new RegExp(`^${p}$`);
  });
  return paths.some((p) => p.test(path));
};

/**
 * URLパスが共通のものかどうかを判定する
 *
 * @param path URLパス
 *
 * @returns 共通のものであればtrue
 */
export const isCommonPath = (path: string) => {
  const paths = Object.values(URL_PATH).map((p) => new RegExp(`^${p}$`));
  return paths.some((p) => p.test(path));
};

/**
 * URLパスがヘッダーのリンクが非表示のものかどうかを判定する
 *
 * @param path URLパス
 *
 * @returns ヘッダーのリンクが非表示のものであればtrue
 */
export const isNoHeaderLink = (path: string) => {
  switch (path) {
    case URL_PATH_PARTICIPANT.SIGN_UP:
    case URL_PATH_GROUP.SIGN_UP:
      return true;
    default:
      return false;
  }
};

/**
 * URLパスがヘッダーのアイコンが非表示のものかどうかを判定する
 * @param path URLパス
 * @returns ヘッダーのアイコンが非表示のものであればtrue
 */
export const isNoHeaderIcon = (path: string) => {
  switch (path) {
    case URL_PATH_PARTICIPANT.SIGN_UP:
    case URL_PATH_GROUP.SIGN_UP:
      return true;
    default:
      return false;
  }
};
