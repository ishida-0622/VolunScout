import { useForm } from "react-hook-form";

import type { UpdateParticipantAccountRequestBody } from "@/__generated__/command";

import { CheckBoxControl } from "@/components/ui-parts/CheckBoxControl";
import { ToggleSwitchControl } from "@/components/ui-parts/ToggleSwitchControl";
import { CONDITIONS, REGIONS, THEMES } from "@/consts";

type FormValues = Pick<
  UpdateParticipantAccountRequestBody,
  | "region"
  | "theme"
  | "required_theme"
  | "condition"
  | "required_condition"
  | "target_status"
>;

type Props = {
  initialValues?: FormValues;
  onSubmit?: (data: FormValues) => void;
};

const noop = () => {};

export const useTermsForm = ({ initialValues, onSubmit = noop }: Props) => {
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
        <details open>
          <summary>地域</summary>
          {REGIONS.map((region) => (
            <div key={region}>
              <label>
                <input type="checkbox" value={region} {...register("region")} />
                {region}
              </label>
            </div>
          ))}
        </details>
      </label>
      <label className="form-label">
        <details open>
          <summary>テーマ</summary>
          {THEMES.map((theme) => (
            <div key={theme} style={{ display: "flex" }}>
              <label>
                <CheckBoxControl
                  name="theme"
                  value={theme}
                  register={register}
                  label={theme}
                />
                <ToggleSwitchControl
                  name="required_theme"
                  value={theme}
                  register={register}
                  label=""
                />
              </label>
            </div>
          ))}
        </details>
      </label>
      <label className="form-label">
        <details open>
          <summary>条件</summary>
          {CONDITIONS.map((condition) => (
            <div key={condition} style={{ display: "flex" }}>
              <label>
                <CheckBoxControl
                  name="condition"
                  value={condition}
                  register={register}
                  label={condition}
                />
                <ToggleSwitchControl
                  name="required_condition"
                  value={condition}
                  register={register}
                  label=""
                />
              </label>
            </div>
          ))}
        </details>
      </label>
    </form>
  );
  return {
    getValues,
    setFormValues,
    InputForm,
  };
};
