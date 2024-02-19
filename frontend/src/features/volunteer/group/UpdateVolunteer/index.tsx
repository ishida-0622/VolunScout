"use client";

import { useQuery } from "@apollo/client";
import { notFound, useRouter } from "next/navigation";
import { useEffect, useRef } from "react";
import { Button, Col, Container, Row, Spinner } from "react-bootstrap";

import { useTermsForm } from "../../useTermsForm";
import { useInfoForm, type FormValues } from "../CreateVolunteer/useInfoForm";

import type { UpdateVolunteerRequestBody } from "@/__generated__/command";

import { gql } from "@/__generated__/query";
import { apiClientVolunteer, s3 } from "@/api/command";
import { utcToJst } from "@/utils/utcToJst";

type Props = { vid: string; gid: string };

const GetVolunteerFromUpdatePageQuery = gql(/* GraphQL */ `
  query getVolunteerFromUpdatePage($vid: String!) {
    volunteer: getVolunteerById(vid: $vid) {
      vid
      gid
      title
      message
      overview
      reward
      recruitedNum
      place
      startAt
      finishAt
      asGroup
      deadlineOn
      regions
      themes
      requiredThemes
      conditions
      requiredConditions
      targetStatus
      photoUrls
    }
  }
`);

// ボランティアの更新ページコンポーネント
export const UpdateVolunteer = ({ vid, gid }: Props) => {
  const router = useRouter();

  // ボランティア情報のクエリを実行
  const { data, loading, error } = useQuery(GetVolunteerFromUpdatePageQuery, {
    variables: { vid },
    fetchPolicy: "cache-and-network",
  });

  const imageRef = useRef<HTMLInputElement>(null); // 画像の参照を作成

  // 情報フォームのカスタムフックを使用
  const { InputForm, setValue, getValues, validation } = useInfoForm({
    imageRef,
  });

  // テーマフォームのカスタムフックを使用
  const {
    InputForm: TermsForm,
    getValues: getTermsValues,
    setFormValues,
  } = useTermsForm({ isOpen: true });

  // ボランティア情報をフォームにセットする副作用を処理
  useEffect(() => {
    if (data) {
      // フォームにボランティア情報をセット
      setFormValues({
        theme: data.volunteer.themes,
        required_theme: data.volunteer.requiredThemes,
        condition: data.volunteer.conditions,
        required_condition: data.volunteer.requiredConditions,
        region: data.volunteer.regions,
      });

      // キーのスネークケースをキャメルケースに変換して値をセット
      const f = (s: string) => {
        return s
          .split("")
          .map((c) => (c === c.toUpperCase() ? `_${c.toLowerCase()}` : c))
          .join("")
          .replace(/^_/, "");
      };

      // 日時情報をJSTに変換してセット
      const volunteer: typeof data.volunteer = {
        ...data.volunteer,
        startAt: utcToJst(data.volunteer.startAt),
        finishAt: utcToJst(data.volunteer.finishAt),
      };

      Object.entries(volunteer).forEach(([key, value]) => {
        setValue(f(key) as keyof FormValues, value);
      });
    }
  }, [data, setFormValues, setValue]);

  // フォームの送信処理
  const handleSubmit = async () => {
    if (!validation()) {
      return;
    }

    const photos = imageRef.current?.files;

    // 写真が4枚以上の場合はアラートを表示して処理を中断
    if (photos && photos.length > 4) {
      alert("写真は4枚までです");
      return;
    }

    // 写真のアップロード処理
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
      setValue("photos", urls);
    }

    // フォームの値を取得
    const infoValues = getValues();
    const termsValues = getTermsValues();

    // 必須項目のチェック
    if (infoValues.target_status.length === 0) {
      alert("募集対象を選択してください");
      return;
    }

    if (termsValues.region.length === 0) {
      alert("活動地域を選択してください");
      return;
    }

    if (termsValues.theme.length + termsValues.required_theme.length === 0) {
      alert("テーマを選択してください");
      return;
    }

    // ボランティア情報のリクエストボディを作成
    const values: UpdateVolunteerRequestBody = {
      vid,
      gid,
      ...getValues(),
      ...getTermsValues(),
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
      // ボランティア情報を更新
      await apiClientVolunteer.updateVolunteer(values);
      alert("更新しました");
      router.back();
    } catch (e) {
      console.error(e);
      alert("エラーが発生しました");
    }
  };

  // ローディング中はスピナーを表示
  if (loading) {
    return <Spinner />;
  }
  // エラーがある場合はnullを返す
  if (error) {
    return null;
  }
  // データが未定義の場合や取得したデータのグループIDが異なる場合は404エラーを表示
  if (data === undefined || data.volunteer.gid !== gid) {
    notFound();
  }

  // 更新フォームをレンダリング
  return (
    <Container>
      {InputForm}
      {TermsForm}
      <Row className="text-center my-3">
        <span className="text-danger">
          ※写真は再度アップロードする必要があります
        </span>
      </Row>
      <Row>
        <Col sm={{ span: 6, offset: 3 }}>
          <Button onClick={handleSubmit} className="w-100">
            更新
          </Button>
        </Col>
      </Row>
    </Container>
  );
};
