import { api, apiRequest } from "@/api";

import type {
  AuthToken,
  LoginRequestDto,
  ResetPasswordRequestDto,
} from "./types";

export const authService = {
  login: (dto: LoginRequestDto) =>
    apiRequest<AuthToken>(api.post("/auth/login", { data: dto })),

  refresh: () => apiRequest<AuthToken>(api.post("/auth/refresh")),

  logout: () => apiRequest<void>(api.post("/auth/logout")),

  requestPasswordReset: (email: string) =>
    apiRequest<void>(
      api.post("/auth/password-reset/request", {
        data: { email },
      }),
    ),

  confirmPasswordReset: (dto: ResetPasswordRequestDto) =>
    apiRequest<void>(api.post("/auth/password-reset", { data: dto })),
};
