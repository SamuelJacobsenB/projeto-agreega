import { useMutation } from "@tanstack/react-query";

import { tokenStorage } from "@/lib/auth";
import { authService } from "../auth.service";
import type { LoginRequestDto } from "../types";

export function useLogin() {
  const { mutateAsync, isPending, error } = useMutation({
    mutationFn: (dto: LoginRequestDto) => authService.login(dto),
    onSuccess: ({ access_token, expires_at }) =>
      tokenStorage.set(access_token, expires_at),
  });

  return { login: mutateAsync, isPending, error };
}
