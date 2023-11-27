import styles from "./index.module.css";

import type { FieldPath, FieldValues, UseFormRegister } from "react-hook-form";

import { joinClassnames } from "@/components/@joinClassnames";

type Props<T extends FieldValues> = {
  name: FieldPath<T>;
  value: string;
  register: UseFormRegister<T>;
  label: string;
  className?: string;
};

export const CheckBoxControl = <T extends FieldValues>({
  name,
  value,
  register,
  label,
  className,
}: Props<T>) => {
  return (
    <label className={joinClassnames(styles.base, className)}>
      <input type="checkbox" value={value} {...register(name)} />
      {label}
    </label>
  );
};
