export interface Parameter {
  address: string;
  type: string;
}

export interface AvatarParameter {
  name: string;
  input?: Parameter;
  output: Parameter;
}

export type ParameterInfo = Pick<AvatarParameter, "name"> & Parameter;

export interface AvatarParameterConfig {
  id: string;
  name: string;
  parameters: AvatarParameter[];
}
