import { Col, Form, Row } from "react-bootstrap";
import { useForm } from "react-hook-form";

import { CONDITIONS, REGIONS, THEMES } from "@/consts";

export type TermsFormValues = {
  region: string[];
  theme: string[];
  required_theme: string[];
  condition: string[];
  required_condition: string[];
};

const initialValues: TermsFormValues = {
  region: [],
  theme: [],
  required_theme: [],
  condition: [],
  required_condition: [],
};

type Props = {
  defaultValues?: TermsFormValues;
  onSubmit?: (data: TermsFormValues) => void;
};

const noop = () => {};

export const useTermsForm = ({
  defaultValues = initialValues,
  onSubmit = noop,
}: Props) => {
  const { register, handleSubmit, getValues, setValue } =
    useForm<TermsFormValues>({
      defaultValues,
    });

  const submitHandler = handleSubmit(onSubmit);

  const setFormValues = (values: TermsFormValues) => {
    Object.entries(values).forEach(([key, value]) => {
      setValue(key as keyof TermsFormValues, value);
    });
  };

  const InputForm = (
    <Form onSubmit={submitHandler}>
      <Form.Group
        as={Row}
        className="w-100 h-100 overflow-scroll"
        style={{
          maxWidth: "1140px",
          maxHeight: "60vh",
          margin: "0 auto",
        }}
      >
        <Col>
          <Form.Label>地域</Form.Label>
          <Row>
            <Col>&emsp;</Col>
          </Row>
          {REGIONS.map((region) => (
            <Form.Check
              key={region}
              id={`regin-${region}`}
              type="checkbox"
              value={region}
              label={region}
              {...register("region")}
            />
          ))}
        </Col>
        <Col>
          <Form.Label>テーマ</Form.Label>
          <Row>
            <Col></Col>
            <Col className="small">必須にする</Col>
          </Row>
          {THEMES.map((theme) => (
            <Row key={theme}>
              <Col>
                <Form.Check
                  id={`theme-${theme}`}
                  type="checkbox"
                  value={theme}
                  label={theme}
                  {...register("theme")}
                />
              </Col>
              <Col>
                <Form.Switch
                  id={`required-${theme}`}
                  type="checkbox"
                  value={theme}
                  {...register("required_theme")}
                />
              </Col>
            </Row>
          ))}
        </Col>
        <Col>
          <Form.Label>条件</Form.Label>
          <Row>
            <Col></Col>
            <Col className="small">必須にする</Col>
          </Row>
          {CONDITIONS.map((condition) => (
            <Row key={condition}>
              <Col>
                <Form.Check
                  id={`condition-${condition}`}
                  type="checkbox"
                  value={condition}
                  label={condition}
                  {...register("condition")}
                />
              </Col>
              <Col>
                <Form.Switch
                  id={`required-${condition}`}
                  type="checkbox"
                  value={condition}
                  {...register("required_condition")}
                />
              </Col>
            </Row>
          ))}
        </Col>
      </Form.Group>
    </Form>
  );
  return {
    InputForm,
    getValues,
    setFormValues,
  };
};
