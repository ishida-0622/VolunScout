import { useLazyQuery, useQuery } from "@apollo/client";
import { notFound } from "next/navigation";
import { useEffect, useState } from "react";
import { Badge, Button, Col, Row, Spinner } from "react-bootstrap";

import { getAllowedStatus } from "../../../utils/getAllowedStatus/index";

import { ParticipantModal } from "./ParticipantModal";

import type { UpdateApplyAllowedStatusRequestBody } from "@/__generated__/command";

import { gql } from "@/__generated__/query";
import { apiClientApply } from "@/api/command/apiClient";
import { ALLOWED_STATUS } from "@/consts";
import { ageCalc } from "@/utils/ageCalc";
import { formatReview } from "@/utils/formatReview";
import { numberToGender } from "@/utils/numberToGender";

type Props = {
  vid: string;
};

type ApplyAndParticipantDataType = {
  __typename: "ApplyAndParticipantData";
  aid: string;
  uid: string;
  asGroup: boolean;
  allowedStatus: number;
  name: string;
  birthday: string;
  gender: number;
  point?: number;
};

const GetApplyQuery = gql(/* GraphQL */ `
  query GetApply($vid: String!) {
    apply: getApplyByVid(vid: $vid) {
      aid
      uid
      asGroup
      allowedStatus
    }
  }
`);

const GetParticipantAccountsQuery = gql(/* GraphQL */ `
  query GetParticipantAccounts($uids: [String!]!) {
    participantAccounts: getParticipantAccounts(uids: $uids) {
      uid
      name
      birthday
      gender
    }
    participantReviews: getParticipantReviewAverageByUids(uids: $uids) {
      uid
      point
    }
  }
`);

export const Apply = ({ vid }: Props) => {
  const [modalValues, setModalValues] = useState({
    uid: "",
    asGroup: false,
    aid: "",
  });
  const [showParticipantModal, setShowParticipantModal] = useState(false);
  const openModal = (uid: string, asGroup: boolean, aid: string) => {
    setModalValues({
      uid,
      asGroup,
      aid,
    });
    setShowParticipantModal(true);
  };
  const closeModal = () => setShowParticipantModal(false);

  const {
    data: applyData,
    loading: applyLoading,
    error: applyError,
    refetch,
  } = useQuery(GetApplyQuery, {
    variables: { vid },
    fetchPolicy: "cache-and-network",
  });

  const accept = async (aid: string) => {
    if (!confirm("承認しますか？")) return;

    const body: UpdateApplyAllowedStatusRequestBody = {
      aid,
      allowed_status: ALLOWED_STATUS.ACCEPT,
    };

    try {
      await apiClientApply.updateApplyAllowedStatus(body);
      alert("承認しました");
      await refetch();
    } catch (error) {
      alert("承認に失敗しました");
    }
  };

  const reject = async (aid: string) => {
    if (!confirm("棄却しますか？")) return;

    const body: UpdateApplyAllowedStatusRequestBody = {
      aid,
      allowed_status: ALLOWED_STATUS.REJECT,
    };

    try {
      await apiClientApply.updateApplyAllowedStatus(body);
      alert("棄却しました");
      await refetch();
    } catch (error) {
      alert("棄却に失敗しました");
    }
  };

  const [applyAndParticipantDataMap, setState] = useState(
    new Map<string, ApplyAndParticipantDataType>()
  );

  const [getParticipantAccounts, { data: participantData }] = useLazyQuery(
    GetParticipantAccountsQuery
  );

  useEffect(() => {
    if (applyData) {
      const uids = applyData.apply.map((apply) => apply.uid);
      getParticipantAccounts({ variables: { uids } }).catch(() => {});
    }
  }, [applyData, getParticipantAccounts]);

  useEffect(() => {
    if (applyData && participantData) {
      const applys = applyData.apply;
      const participantAccounts = participantData.participantAccounts;
      const participantReviews = participantData.participantReviews;
      const participantDataMap = new Map<string, ApplyAndParticipantDataType>();
      for (const apply of applys) {
        const participantAccount = participantAccounts.find(
          (account) => account.uid === apply.uid
        );
        const participantReview = participantReviews.find(
          (review) => review.uid === apply.uid
        );
        if (!participantAccount) {
          continue;
        }
        participantDataMap.set(apply.uid, {
          ...apply,
          ...participantReview,
          ...participantAccount,
          __typename: "ApplyAndParticipantData",
        });
      }
      setState(participantDataMap);
    }
  }, [applyData, participantData]);

  if (applyLoading) {
    return <Spinner />;
  }

  if (applyError) {
    notFound();
  }

  if (applyData === undefined) {
    return notFound();
  }

  return (
    <div>
      {Array.from(applyAndParticipantDataMap).map(([uid, data]) =>
        data.allowedStatus === ALLOWED_STATUS.REJECT ? null : (
          <Row
            key={uid}
            className="mb-3 border border-2 rounded-2 align-items-center p-2"
          >
            <Col>{data.name}</Col>
            <Col>{ageCalc(data.birthday)}歳</Col>
            <Col>{numberToGender(data.gender)}</Col>
            <Col>{formatReview(data.point)}</Col>
            <Col>
              <Button onClick={() => openModal(uid, data.asGroup, data.aid)}>
                詳細
              </Button>
            </Col>
            <Col>{data.asGroup && <Badge>団体応募</Badge>}</Col>
            <Col>
              {data.allowedStatus === ALLOWED_STATUS.ACCEPT && (
                <Badge>{getAllowedStatus(data.allowedStatus)}</Badge>
              )}
              {data.allowedStatus === ALLOWED_STATUS.PENDING && (
                <>
                  <Button
                    variant="primary"
                    className="mx-2"
                    onClick={() => accept(data.aid)}
                  >
                    承認
                  </Button>
                  <Button variant="danger" onClick={() => reject(data.aid)}>
                    棄却
                  </Button>
                </>
              )}
            </Col>
          </Row>
        )
      )}
      <ParticipantModal
        show={showParticipantModal}
        onHide={closeModal}
        {...modalValues}
      />
    </div>
  );
};
