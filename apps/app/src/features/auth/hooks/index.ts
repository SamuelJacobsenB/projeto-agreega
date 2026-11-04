import { useMutation } from "@tanstack/react-query";

import { tokenStorage } from "@/lib/auth";

import { authService } from "../auth.service";
import type { LoginRequestDto, ResetPasswordRequestDto } from "../types";

export function useLogin() {
  const { mutateAsync, isPending, error } = useMutation({
    mutationFn: (dto: LoginRequestDto) => authService.login(dto),
    onSuccess: ({ data }) => {
      if (data) {
        tokenStorage.set(data.access_token, data.expires_at);
      }
    },
  });

  return { login: mutateAsync, isPending, error };
}

export function useLogout() {
  const { mutateAsync, isPending, error } = useMutation({
    mutationFn: authService.logout,
    onSuccess: () => tokenStorage.clear(),
  });

  return { logout: mutateAsync, isPending, error };
}

export function useRequestPasswordReset() {
  const { mutateAsync, isPending, error } = useMutation({
    mutationFn: (email: string) => authService.requestPasswordReset(email),
  });

  return { requestPasswordReset: mutateAsync, isPending, error };
}

export function useConfirmPasswordReset() {
  const { mutateAsync, isPending, error } = useMutation({
    mutationFn: (dto: ResetPasswordRequestDto) =>
      authService.confirmPasswordReset(dto),
  });

  return { confirmPasswordReset: mutateAsync, isPending, error };
}
