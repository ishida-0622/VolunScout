"use client";
import { get, set } from "idb-keyval";
import { useCallback, useEffect, useRef } from "react";
import { Button } from "react-bootstrap";

import styles from "./index.module.css";
import { useInfoForm, type FormValues } from "./useInfoForm";

import type { CreateVolunteerRequestBody } from "@/__generated__/command";

import { apiClientVolunteer, s3 } from "@/api/command";
import { joinClassnames } from "@/components/@joinClassnames";
import { useAuthContext } from "@/contexts/AuthContext";
import {
  useTermsForm,
  type TermsFormValues,
} from "@/features/volunteer/useTermsForm";

export const CreateVolunteer = () => {
  const authContext = useAuthContext();

  const imageRef = useRef<HTMLInputElement>(null);

  const {
    InputForm: InfoForm,
    getValues: getInfoValues,
    setValue: setInfoValue,
  } = useInfoForm({ imageRef });

  const {
    InputForm: TermsForm,
    getValues: getTermsValues,
    setFormValues: setTermsValue,
  } = useTermsForm({});

  const writeLocalStorage = () => {
    const infoValues = getInfoValues();
    const termsValues = getTermsValues();
    const storageInfoKey = "create-volunteer-info";
    const storageTermsKey = "create-volunteer-terms";
    set(storageInfoKey, infoValues).catch((e) => console.error(e));
    set(storageTermsKey, termsValues).catch((e) => console.error(e));
  };

  const clearLocalStorage = () => {
    const storageInfoKey = "create-volunteer-info";
    const storageTermsKey = "create-volunteer-terms";
    set(storageInfoKey, undefined).catch((e) => console.error(e));
    set(storageTermsKey, undefined).catch((e) => console.error(e));
  };

  const readLocalStorage = useCallback(() => {
    get("create-volunteer-info")
      .then((value: FormValues | undefined) => {
        if (value !== undefined) {
          const keys = Object.keys(value) as (keyof FormValues)[];
          keys.forEach((key) => {
            setInfoValue(key, value[key]);
          });
        }
      })
      .catch((e) => console.error(e));

    get("create-volunteer-terms")
      .then((value: TermsFormValues | undefined) => {
        if (value !== undefined) {
          setTermsValue(value);
        }
      })
      .catch((e) => console.error(e));
  }, [getInfoValues, setInfoValue]);

  const handleOnSave = () => {
    writeLocalStorage();
    alert("保存しました");
  };

  const handleOnSubmit = async () => {
    const gid = authContext.user?.uid;
    if (gid === undefined) {
      throw new Error("gid is undefined");
    }

    const photos = imageRef.current?.files;

    if (photos) {
      const requests = [];
      const urls = [];

      const bucket = process.env.NEXT_PUBLIC_S3_BUCKET_NAME;
      if (bucket === undefined) {
        throw new Error("S3_BUCKET_NAME is undefined");
      }

      for (let i = 0; i < photos.length; i++) {
        const photo = photos[i];
        const uuid = crypto.randomUUID();
        const key = `volunteer/${gid}/${uuid}.${photo.name.split(".").pop()}`;
        const promise = s3.putObject({
          Key: key,
          Body: photo,
          Bucket: bucket,
        });
        requests.push(promise.promise());
        urls.push(`https://${bucket}.s3.amazonaws.com/${key}`);
      }
      await Promise.all(requests);
      setInfoValue("photos", urls);
    }

    const infoValues = getInfoValues();
    const termsValues = getTermsValues();

    if (infoValues.target_status.length === 0) {
      alert("募集対象を選択してください");
      return;
    }

    const body: CreateVolunteerRequestBody = {
      gid,
      ...infoValues,
      ...termsValues,
      recruited_num: Number(infoValues.recruited_num),
      start_at: new Date(infoValues.start_at).toISOString(),
      finish_at: new Date(infoValues.finish_at).toISOString(),
      theme: termsValues.theme.flatMap((theme) =>
        termsValues.required_theme.includes(theme) ? [] : [theme]
      ),
      condition: termsValues.condition.flatMap((condition) =>
        termsValues.required_condition.includes(condition) ? [] : [condition]
      ),
    };

    try {
      await apiClientVolunteer.createVolunteer(body);
      clearLocalStorage();
    } catch (error) {
      console.error(error);
      alert("エラーが発生しました");
    }
  };

  useEffect(() => {
    readLocalStorage();
  }, [readLocalStorage]);

  return (
    <div>
      {InfoForm}
      {TermsForm}
      <div className={joinClassnames("mb-3", styles.button_wrapper)}>
        <Button variant="danger" size="lg" className="w-">
          キャンセル
        </Button>
        <Button variant="secondary" size="lg" onClick={handleOnSave}>
          一時保存（写真は保存されません）
        </Button>
        <Button variant="primary" size="lg" onClick={handleOnSubmit}>
          作成する
        </Button>
      </div>
    </div>
  );
};
