import { api, apiRequest } from "@/api";

import type {
  AuthToken,
  LoginRequestDto,
  ResetPasswordRequestDto,
} from "./types";

export const authService = {
  login: (dto: LoginRequestDto) =>
    apiRequest<AuthToken>(api.post("/auth/login", dto)),

  refresh: () => apiRequest<AuthToken>(api.post("/auth/refresh")),

  logout: () => apiRequest<void>(api.post("/auth/logout")),

  requestPasswordReset: (email: string) =>
    apiRequest<void>(api.post("/auth/password-reset/request", { email })),

  confirmPasswordReset: (dto: ResetPasswordRequestDto) =>
    apiRequest<void>(api.post("/auth/password-reset", dto)),
};
