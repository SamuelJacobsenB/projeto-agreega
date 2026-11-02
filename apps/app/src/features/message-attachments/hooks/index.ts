import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import { messageAttachmentService } from "../message-attachment.service";
import { messageAttachmentQueryKeys } from "../query-keys";
import type { CreateMessageAttachmentDto } from "../types";

export function useMessageAttachments(messageId: string) {
  return useQuery({
    queryKey: messageAttachmentQueryKeys.byMessage(messageId),
    queryFn: () => messageAttachmentService.listByMessage(messageId),
    enabled: Boolean(messageId),
  });
}

export function useMessageAttachmentFile(messageId: string, fileId: string) {
  return useQuery({
    queryKey: messageAttachmentQueryKeys.file(messageId, fileId),
    queryFn: () => messageAttachmentService.getFile(messageId, fileId),
    enabled: Boolean(messageId && fileId),
  });
}

export function useCreateMessageAttachment() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (dto: CreateMessageAttachmentDto) =>
      messageAttachmentService.create(dto),
    onSuccess: (_response, dto) =>
      queryClient.invalidateQueries({
        queryKey: messageAttachmentQueryKeys.byMessage(dto.message_id),
      }),
  });
}

export function useDeleteMessageAttachment() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ messageId, fileId }: { messageId: string; fileId: string }) =>
      messageAttachmentService.delete(messageId, fileId),
    onSuccess: (_response, { messageId }) =>
      queryClient.invalidateQueries({
        queryKey: messageAttachmentQueryKeys.byMessage(messageId),
      }),
  });
}
