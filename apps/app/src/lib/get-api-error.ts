import axios from "axios";

import type { ApiError } from "@/types";

export function getApiError(error: unknown): ApiError {
  if (axios.isAxiosError<ApiError>(error)) {
    return (
      error.response?.data ?? {
        success: false,
        message: "Não foi possível conectar ao servidor.",
      }
    );
  }

  return {
    success: false,
    message: "Ocorreu um erro inesperado.",
  };
}
