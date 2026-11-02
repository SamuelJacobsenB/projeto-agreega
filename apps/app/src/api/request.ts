import type { AxiosResponse } from "axios";

import type { ApiResponse } from "@/types";

export async function apiRequest<T>(
  request: Promise<AxiosResponse<ApiResponse<T>>>,
): Promise<ApiResponse<T>> {
  const { data } = await request;
  return data;
}
