import type { AccountType } from "../types";

import { isGroupPath } from "@/consts";

/**
 * 現在のパスからアカウントタイプを取得する
 *
 * 参加者とグループどちらからもアクセス可能なページの場合は参加者として扱う
 * @param path 現在のパス
 * @returns アカウントタイプ
 */
export const getAccountTypeFromPath = (path: string): AccountType => {
  if (isGroupPath(path)) {
    return "group";
  }
  // MEMO: 参加者とグループどちらからもアクセス可能なページの場合は参加者として扱う
  return "participant";
};
