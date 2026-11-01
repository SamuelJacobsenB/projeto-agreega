import { useMutation } from "@tanstack/react-query";

import { authService } from "../auth.service";

export function useRequestPasswordReset() {
  const { mutateAsync, isPending, error } = useMutation({
    mutationFn: (email: string) => authService.requestPasswordReset(email),
  });

  return { requestPasswordReset: mutateAsync, isPending, error };
}
