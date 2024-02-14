/**
 * レビューを整形する
 * @param review レビュー
 * @returns 整形されたレビュー
 */
export const formatReview = (review: number | undefined) => {
  if (review === undefined) {
    return "まだレビューがありません";
  }
  if (review < 0 || review > 5) {
    throw new Error("invalid review");
  }
  let formattedReview = "";
  const roundedReview = Math.round(review);
  for (let i = 0; i < roundedReview; i++) {
    formattedReview += "★";
  }
  for (let i = 0; i < 5 - roundedReview; i++) {
    formattedReview += "☆";
  }
  return `${formattedReview} ${review}`;
};
