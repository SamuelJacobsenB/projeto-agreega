import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import { messageService } from "../message.service";
import { messageQueryKeys } from "../query-keys";
import type { CreateMessageDto, UpdateMessageDto } from "../types";

export function useProjectMessages(projectId: string) {
  return useQuery({
    queryKey: messageQueryKeys.byProject(projectId),
    queryFn: () => messageService.listByProject(projectId),
    enabled: Boolean(projectId),
  });
}

export function useCreateMessage() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (dto: CreateMessageDto) => messageService.create(dto),
    onSuccess: (_response, dto) =>
      queryClient.invalidateQueries({
        queryKey: messageQueryKeys.byProject(dto.project_id),
      }),
  });
}

export function useUpdateMessage() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ id, dto }: { id: string; dto: UpdateMessageDto }) =>
      messageService.update(id, dto),
    onSuccess: () =>
      queryClient.invalidateQueries({ queryKey: messageQueryKeys.all }),
  });
}

export function useDeleteMessage() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: messageService.delete,
    onSuccess: () =>
      queryClient.invalidateQueries({ queryKey: messageQueryKeys.all }),
  });
}
