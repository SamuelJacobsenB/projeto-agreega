import axios from "axios";

import { tokenStorage } from "@/lib/auth";

export const api = axios.create({
  baseURL: import.meta.env.VITE_API_URL,
  withCredentials: true,
  timeout: 5000,
  headers: {
    "Content-Type": "application/json",
  },
});

api.interceptors.response.use(
  (response) => response,

  async (error) => {
    const originalRequest = error.config;

    if (error.response?.status !== 401 || originalRequest._retry) {
      return Promise.reject(error);
    }

    originalRequest._retry = true;

    try {
      const { data } = await api.post("/auth/refresh");

      tokenStorage.set(data.access_token, data.expires_at);

      originalRequest.headers.Authorization = `Bearer ${data.access_token}`;

      return api(originalRequest);
    } catch (refreshError) {
      tokenStorage.clear();

      return Promise.reject(refreshError);
    }
  },
);
