import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import { userService } from "../user.service";
import { userQueryKeys } from "../query-keys";
import type { UpdateUserDto } from "../types";

export function useMyUser() {
  return useQuery({ queryKey: userQueryKeys.mine(), queryFn: userService.getMine });
}

export function useUsers() {
  return useQuery({ queryKey: userQueryKeys.lists(), queryFn: userService.list });
}

export function useUser(id: string) {
  return useQuery({
    queryKey: userQueryKeys.detail(id),
    queryFn: () => userService.getById(id),
    enabled: Boolean(id),
  });
}

export function useUpdateUser() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ id, dto }: { id: string; dto: UpdateUserDto }) =>
      userService.update(id, dto),
    onSuccess: (_response, { id }) => {
      queryClient.invalidateQueries({ queryKey: userQueryKeys.lists() });
      queryClient.invalidateQueries({ queryKey: userQueryKeys.detail(id) });
      queryClient.invalidateQueries({ queryKey: userQueryKeys.mine() });
    },
  });
}

export function useDeleteUser() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: userService.delete,
    onSuccess: () => queryClient.invalidateQueries({ queryKey: userQueryKeys.all }),
  });
}
