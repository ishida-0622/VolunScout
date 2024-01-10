import Link from "next/link";
import { useState } from "react";
import Modal from "react-modal";

import { PrivacyPolicyPage } from "@/features/legal/PrivacyPolicyPage";
import { TermsOfServicePage } from "@/features/legal/TermsOfServicePage";

export const TermsOfUseAndPrivacyPolicyModal = () => {
  const [isTermsOfUseModalOpen, setIsTermsOfUseModalOpen] = useState(false);
  const openTermsOfUseModal = () => setIsTermsOfUseModalOpen(true);
  const closeTermsOfUseModal = () => setIsTermsOfUseModalOpen(false);

  const [isPrivacyPolicyModalOpen, setIsPrivacyPolicyModalOpen] =
    useState(false);
  const openPrivacyPolicyModal = () => setIsPrivacyPolicyModalOpen(true);
  const closePrivacyPolicyModal = () => setIsPrivacyPolicyModalOpen(false);

  return (
    <>
      <span>
        <Link href={{}} onClick={openTermsOfUseModal}>
          利用規約
        </Link>
        と
        <Link href={{}} onClick={openPrivacyPolicyModal}>
          プライバシーポリシー
        </Link>
        に同意する
      </span>
      <Modal
        isOpen={isTermsOfUseModalOpen}
        onRequestClose={closeTermsOfUseModal}
      >
        <TermsOfServicePage />
        <button onClick={closeTermsOfUseModal}>閉じる</button>
      </Modal>
      <Modal
        isOpen={isPrivacyPolicyModalOpen}
        onRequestClose={closePrivacyPolicyModal}
      >
        <PrivacyPolicyPage />
        <button onClick={closePrivacyPolicyModal}>閉じる</button>
      </Modal>
    </>
  );
};
