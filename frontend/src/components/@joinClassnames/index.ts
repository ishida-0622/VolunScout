export const joinClassnames = (...classNames: (string | undefined)[]) =>
  classNames.filter(Boolean).join(" ");
