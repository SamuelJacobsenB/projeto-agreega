import { useMutation } from "@tanstack/react-query";

import { tokenStorage } from "@/lib/auth";
import { authService } from "../auth.service";

export function useLogout() {
  const { mutateAsync, isPending, error } = useMutation({
    mutationFn: authService.logout,
    onSuccess: () => tokenStorage.clear(),
  });

  return { logout: mutateAsync, isPending, error };
}
