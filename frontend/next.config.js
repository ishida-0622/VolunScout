/** @type {import('next').NextConfig} */
const nextConfig = {
  images: {
    remotePatterns: [
      {
        protocol: "https",
        hostname: "lh3.googleusercontent.com",
      },
      {
        protocol: "https",
        hostname: "volunscout.s3.ap-northeast-1.amazonaws.com",
      },
    ],
  },
};

module.exports = nextConfig;
