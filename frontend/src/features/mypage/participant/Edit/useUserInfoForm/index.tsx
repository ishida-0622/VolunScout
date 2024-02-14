import { useForm } from "react-hook-form";

import styles from "./index.module.css";

import type { UpdateParticipantAccountRequestBody } from "@/__generated__/command";

import { joinClassnames } from "@/components/@joinClassnames";
import { TARGET_STATUSES } from "@/consts";
import { numberToGender } from "@/utils/numberToGender";

type FormValues = Pick<
  UpdateParticipantAccountRequestBody,
  | "name"
  | "furigana"
  | "phone"
  | "gender"
  | "birthday"
  | "profile"
  | "target_status"
>;

type Props = {
  initialValues?: FormValues;
  onSubmit?: (data: FormValues) => void;
};

const noop = () => {};

export const useUserInfoForm = ({ initialValues, onSubmit = noop }: Props) => {
  const { register, handleSubmit, getValues, setValue } = useForm<FormValues>({
    defaultValues: initialValues,
  });

  const setFormValues = (values: FormValues) => {
    Object.entries(values).forEach(([key, value]) => {
      setValue(key as keyof FormValues, value);
    });
  };

  const InputForm = (
    <form onSubmit={handleSubmit(onSubmit)}>
      <div className={joinClassnames(styles.main)}>
        <div className="row my-3">
          <label htmlFor="name" className="col-sm-2 col-form-label">
            <span>氏名</span>
          </label>
          <div className="col-sm-10">
            <input
              type="text"
              id="name"
              {...register("name", { required: true })}
              className={joinClassnames("form-control")}
              placeholder="山田 太郎"
            />
          </div>
        </div>
        <div className="row mb-3">
          <label htmlFor="furigana" className="col-sm-2 col-form-label">
            <span>フリガナ</span>
          </label>
          <div className="col-sm-10">
            <input
              type="text"
              id="furigana"
              {...register("furigana", { required: true })}
              className={joinClassnames("form-control")}
              placeholder="ヤマダ タロウ"
            />
          </div>
        </div>
        <div className="row mb-3">
          <label htmlFor="birth" className="col-sm-2 col-form-label">
            <span>生年月日</span>
          </label>
          <div className="col-sm-10">
            <input
              type="date"
              {...register("birthday", { required: true })}
              className={joinClassnames("form-control")}
            />
          </div>
        </div>
        <div className="row mb-3">
          <label htmlFor="target" className="col-sm-2 col-form-label">
            <span>区分</span>
          </label>
          <div className="col-sm-10">
            <select
              {...register("target_status", { required: true })}
              className="form-select"
            >
              {TARGET_STATUSES.map((target) => (
                <option key={target} value={target}>
                  {target}
                </option>
              ))}
            </select>
          </div>
        </div>
        <div className="row mb-3">
          <label htmlFor="gender" className="col-sm-2 col-form-label">
            <span>性別</span>
          </label>
          <div className="col-sm-10">
            <select
              {...register("gender", { required: true })}
              className="form-select"
            >
              <option value="0">{numberToGender(0)}</option>
              <option value="1">{numberToGender(1)}</option>
              <option value="2">{numberToGender(2)}</option>
            </select>
          </div>
        </div>
        <div className="row mb-3">
          <label htmlFor="phone" className="col-sm-2 col-form-label">
            <span>電話番号</span>
          </label>
          <div className="col-sm-10">
            <input
              type="text"
              {...register("phone", { required: true })}
              className={joinClassnames("form-control")}
              placeholder="09012345678"
            />
          </div>
        </div>
      </div>
      <div className={styles.profile}>
        <label className="form-label">
          <span>プロフィール</span>
          <textarea
            {...register("profile")}
            className={joinClassnames("form-control", styles.textarea)}
          />
        </label>
      </div>
    </form>
  );
  return { InputForm, getValues, setFormValues };
};
