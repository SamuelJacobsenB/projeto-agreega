import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import { clientService } from "../client.service";
import { clientQueryKeys } from "../query-keys";
import type { CreateClientDto, UpdateClientDto } from "../types";

export function useMyClient() {
  return useQuery({
    queryKey: clientQueryKeys.mine(),
    queryFn: clientService.getMine,
  });
}

export function useClients() {
  return useQuery({
    queryKey: clientQueryKeys.lists(),
    queryFn: clientService.list,
  });
}

export function useClient(id: string) {
  return useQuery({
    queryKey: clientQueryKeys.detail(id),
    queryFn: () => clientService.getById(id),
    enabled: Boolean(id),
  });
}

export function useCreateClient() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (dto: CreateClientDto) => clientService.create(dto),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: clientQueryKeys.all }),
  });
}

export function useUpdateClient() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ id, dto }: { id: string; dto: UpdateClientDto }) =>
      clientService.update(id, dto),
    onSuccess: (_response, { id }) => {
      queryClient.invalidateQueries({ queryKey: clientQueryKeys.lists() });
      queryClient.invalidateQueries({ queryKey: clientQueryKeys.detail(id) });
      queryClient.invalidateQueries({ queryKey: clientQueryKeys.mine() });
    },
  });
}

export function useDeleteClient() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: clientService.delete,
    onSuccess: () => queryClient.invalidateQueries({ queryKey: clientQueryKeys.all }),
  });
}
