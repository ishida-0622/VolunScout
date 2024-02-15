export const objectToUrlSearch = (object: object) => {
  const searchParams = new URLSearchParams();
  Object.entries(object).forEach(([key, value]) => {
    if (Array.isArray(value)) {
      searchParams.append(key, value.join(","));
    } else {
      searchParams.append(key, String(value));
    }
  });
  return searchParams.toString();
};
