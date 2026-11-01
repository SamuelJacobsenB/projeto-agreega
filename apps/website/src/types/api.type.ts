export type ApiResponse<T> = {
  success: boolean;
  message: string;
  data: T | null;
  fields: Record<string, string[]> | null;
};
