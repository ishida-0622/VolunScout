"use client";

import { useLazyQuery } from "@apollo/client";
import { useEffect } from "react";

import styles from "./index.module.css";

import { gql } from "@/__generated__/query";
import { CONDITIONS, REGIONS, THEMES } from "@/consts";
import { useAuthContext } from "@/contexts/AuthContext";

const GetParticipantAccountTermsQuery = gql(/* GraphQL */ `
  query GetParticipantAccountTerms($uid: String!) {
    regions: getParticipantRegions(uid: $uid) {
      name
    }
    themes: getParticipantThemes(uid: $uid) {
      name
      isRequired
    }
    conditions: getParticipantConditions(uid: $uid) {
      name
      isRequired
    }
  }
`);

export const Registration = () => {
  const { user } = useAuthContext();

  const [fetchParticipantAccountTerms, { loading, error, data }] = useLazyQuery(
    GetParticipantAccountTermsQuery,
  );

  useEffect(() => {
    if (typeof user?.uid === "string") {
      fetchParticipantAccountTerms({ variables: { uid: user.uid } }).catch(
        (e) => {
          console.error(e);
        },
      );
    }
  }, [fetchParticipantAccountTerms, user?.uid]);

  if (loading || data === undefined) {
    return null;
  }

  if (error) {
    console.error(error);
    return null;
  }

  const regionsSet = new Set(data.regions.map((region) => region.name));
  const themesMap = new Map(data.themes.map((theme) => [theme.name, theme]));
  const conditionsMap = new Map(
    data.conditions.map((condition) => [condition.name, condition]),
  );

  return (
    <div className={styles.wrapper}>
      <div className={styles.regions}>
        <details open>
          <summary>地域</summary>
          {REGIONS.map((region) => {
            if (regionsSet.has(region)) {
              return <p key={region}>{region}</p>;
            }
          })}
        </details>
      </div>
      <div className={styles.theme}>
        <details open>
          <summary>テーマ</summary>
          {THEMES.map((theme) => {
            const themeData = themesMap.get(theme);
            if (themeData === undefined) {
              return null;
            }
            if (themeData.isRequired) {
              return (
                <p key={theme}>
                  <span>※</span>
                  <span>{theme}</span>
                </p>
              );
            }
            return <p key={theme}>{theme}</p>;
          })}
        </details>
      </div>
      <div className={styles.condition}>
        <details open>
          <summary>活動希望条件</summary>
          {CONDITIONS.map((condition) => {
            const conditionData = conditionsMap.get(condition);
            if (conditionData === undefined) {
              return null;
            }
            if (conditionData.isRequired) {
              return (
                <p key={condition}>
                  <span>※</span>
                  <span>{condition}</span>
                </p>
              );
            }
            return <p key={condition}>{condition}</p>;
          })}
        </details>
      </div>
    </div>
  );
};
