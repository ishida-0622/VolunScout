import AWS from "aws-sdk";

// AWSリージョンを環境変数から取得
const region = process.env.NEXT_PUBLIC_AWS_REGION;
// S3バケット名を環境変数から取得
const bucketName = process.env.NEXT_PUBLIC_S3_BUCKET_NAME;
// AWS Identity PoolのIDを環境変数から取得
const identityPoolId = process.env.NEXT_PUBLIC_AWS_IDENTITY_POOL_ID;

// 環境変数が未定義の場合はエラーをスロー
if (!region) {
  throw new Error("NEXT_PUBLIC_AWS_REGION is not defined");
}
if (!bucketName) {
  throw new Error("NEXT_PUBLIC_S3_BUCKET_NAME is not defined");
}
if (!identityPoolId) {
  throw new Error("NEXT_PUBLIC_AWS_IDENTITY_POOL_ID is not defined");
}

// AWS設定を更新
AWS.config.update({
  region,
  credentials: new AWS.CognitoIdentityCredentials({
    IdentityPoolId: identityPoolId,
  }),
});

// S3クライアントを作成
export const s3 = new AWS.S3({
  region,
  params: {
    Bucket: bucketName,
  },
});
