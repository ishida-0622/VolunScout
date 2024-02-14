import { FaSearch } from "react-icons/fa";

import styles from "./index.module.css";

import { joinClassnames } from "@/components/@joinClassnames";

type Props = {
  initValue?: string;
  placeholder?: string;
  onChange?: (value: string) => void;
  className?: string;
};

const noop = () => {};

export const SearchBar = ({
  initValue,
  placeholder,
  onChange = noop,
  className,
}: Props) => {
  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    onChange(e.target.value);
  };

  return (
    <label className={joinClassnames(styles.search_bar, className)}>
      <FaSearch />
      <input
        type="text"
        defaultValue={initValue}
        placeholder={placeholder}
        onChange={handleChange}
        className="w-100"
      />
    </label>
  );
};
