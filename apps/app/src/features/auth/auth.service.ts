import { api } from "@/api";
import type {
  AuthToken,
  LoginRequestDto,
  ResetPasswordRequestDto,
} from "./types";

export const authService = {
  login: (dto: LoginRequestDto) =>
    api.post<unknown, AuthToken>("/auth/login", { data: dto }),
  logout: () => api.post("/auth/logout"),

  requestPasswordReset: (email: string) =>
    api.post("/auth/password-reset/request", { data: { email } }),
  confirmPasswordReset: (dto: ResetPasswordRequestDto) =>
    api.post("/auth/password-reset", { data: dto }),
};
