export const parameterTypes = ["Int", "Bool", "Float"] as const;

export type ParameterType = typeof parameterTypes[number];

export interface Parameter {
  addr: string;
  typ: ParameterType;
}
