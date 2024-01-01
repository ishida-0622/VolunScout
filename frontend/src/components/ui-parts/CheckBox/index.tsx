import { useReducer } from "react";

import styles from "./index.module.css";

import { joinClassnames } from "@/components/@joinClassnames";

type Props = {
  label: string;
  initialState?: boolean;
  onChange?: (_: boolean) => void;
  className?: string;
  children?: React.ReactNode;
};

const noop = (_: boolean) => {};

export const CheckBox = ({
  label,
  initialState = false,
  onChange = noop,
  className,
  children,
  ...props
}: Props) => {
  const [state, toggle] = useReducer((state) => {
    // MEMO: Reducer 内部は純粋な処理の必要があるので、副作用の可能性を考慮して next tick に処理をずらす
    setTimeout(() => onChange(!state));

    return !state;
  }, initialState);

  return (
    <label className={joinClassnames(styles.base, className)}>
      <input type="checkbox" checked={state} onChange={toggle} {...props} />
      {label}
      {children}
    </label>
  );
};
