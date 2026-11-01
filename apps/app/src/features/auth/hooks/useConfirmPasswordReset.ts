import { useMutation } from "@tanstack/react-query";

import { authService } from "../auth.service";
import type { ResetPasswordRequestDto } from "../types";

export function useConfirmPasswordReset() {
  const { mutateAsync, isPending, error } = useMutation({
    mutationFn: (dto: ResetPasswordRequestDto) =>
      authService.confirmPasswordReset(dto),
  });

  return { confirmPasswordReset: mutateAsync, isPending, error };
}
