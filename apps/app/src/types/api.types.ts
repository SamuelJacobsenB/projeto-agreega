export type ApiResponse<T> = {
  success: boolean;
  message: string;
  data: T | null;
  fields?: Record<string, string[]> | null;
};

export interface ApiError {
  success: false;
  message: string;
  fields?: Record<string, string>;
}
