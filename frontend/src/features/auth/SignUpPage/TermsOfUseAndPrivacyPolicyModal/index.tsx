import Link from "next/link";

import { URL_PATH } from "@/consts";

export const TermsOfUseAndPrivacyPolicyModal = () => {
  return (
    <>
      <span>
        <Link href={URL_PATH.TERMS_OF_SERVICE} target="_blank">
          利用規約
        </Link>
        と
        <Link href={URL_PATH.PRIVACY_POLICY} target="_blank">
          プライバシーポリシー
        </Link>
        に同意する
      </span>
    </>
  );
};
