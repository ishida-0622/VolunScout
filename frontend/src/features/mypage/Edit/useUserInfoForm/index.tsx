import { useForm } from "react-hook-form";

import type { UpdateParticipantAccountRequestBody } from "@/__generated__/command";

import { joinClassnames } from "@/components/@joinClassnames";
import { numberToGender } from "@/utils/numberToGender";

type FormValues = Pick<
  UpdateParticipantAccountRequestBody,
  "name" | "furigana" | "phone" | "gender" | "birthday" | "profile"
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
      <label className="form-label">
        <span>氏名</span>
        <span>：</span>
        <input
          type="text"
          {...register("name", { required: true })}
          className={joinClassnames("form-control")}
          placeholder="山田 太郎"
        />
      </label>
      <label className="form-label">
        <span>フリガナ</span>
        <span>：</span>
        <input
          type="text"
          {...register("furigana", { required: true })}
          className={joinClassnames("form-control")}
          placeholder="ヤマダ タロウ"
        />
      </label>
      <label className="form-label">
        <span>生年月日</span>
        <span>：</span>
        <input
          type="date"
          {...register("birthday", { required: true })}
          className={joinClassnames("form-control")}
        />
      </label>
      <label className="form-label">
        <span>性別</span>
        <span>：</span>
        <select
          {...register("gender", { required: true })}
          className="form-select"
        >
          <option value="0">{numberToGender(0)}</option>
          <option value="1">{numberToGender(1)}</option>
          <option value="2">{numberToGender(2)}</option>
        </select>
      </label>
      <label className="form-label">
        <span>電話番号</span>
        <span>：</span>
        <input
          type="text"
          {...register("phone", { required: true })}
          className={joinClassnames("form-control")}
          placeholder="09012345678"
        />
      </label>
      <label className="form-label">
        <span>プロフィール</span>
        <span>：</span>
        <textarea
          {...register("profile")}
          className={joinClassnames("form-control")}
        />
      </label>
    </form>
  );
  return { InputForm, getValues, setFormValues };
};
