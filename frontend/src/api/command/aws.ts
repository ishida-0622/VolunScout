import AWS from "aws-sdk";

const region = process.env.NEXT_PUBLIC_AWS_REGION;
const bucketName = process.env.NEXT_PUBLIC_S3_BUCKET_NAME;
const identityPoolId = process.env.NEXT_PUBLIC_AWS_IDENTITY_POOL_ID;

if (!region) throw new Error("NEXT_PUBLIC_AWS_REGION is not defined");
if (!bucketName) throw new Error("NEXT_PUBLIC_S3_BUCKET_NAME is not defined");
if (!identityPoolId)
  throw new Error("NEXT_PUBLIC_AWS_IDENTITY_POOL_ID is not defined");

AWS.config.update({
  region,
  credentials: new AWS.CognitoIdentityCredentials({
    IdentityPoolId: identityPoolId,
  }),
});

export const s3 = new AWS.S3({
  region,
  params: {
    Bucket: bucketName,
  },
});
