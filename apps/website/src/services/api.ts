import axios from "axios";

import type { ApiResponse } from "../types";

export const api = axios.create({
  baseURL: import.meta.env.VITE_API_URL,
  withCredentials: true,
  timeout: 5000,
  headers: {
    "Content-Type": "application/json",
  },
});

api.interceptors.response.use(
  (response) => response.data,

  (error) => {
    if (axios.isAxiosError<ApiResponse<unknown>>(error)) {
      return Promise.reject(error.response?.data ?? error);
    }

    return Promise.reject(error);
  },
);
