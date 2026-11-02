import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import { documentService } from "../document.service";
import { documentQueryKeys } from "../query-keys";
import type { CreateDocumentDto, UpdateDocumentDto } from "../types";

export function useStageDocuments(stageId: string) {
  return useQuery({
    queryKey: documentQueryKeys.byStage(stageId),
    queryFn: () => documentService.listByStage(stageId),
    enabled: Boolean(stageId),
  });
}

export function useTaskDocuments(taskId: string) {
  return useQuery({
    queryKey: documentQueryKeys.byTask(taskId),
    queryFn: () => documentService.listByTask(taskId),
    enabled: Boolean(taskId),
  });
}

export function useDocument(id: string) {
  return useQuery({
    queryKey: documentQueryKeys.detail(id),
    queryFn: () => documentService.getById(id),
    enabled: Boolean(id),
  });
}

export function useCreateDocument() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (dto: CreateDocumentDto) => documentService.create(dto),
    onSuccess: (_response, dto) => {
      queryClient.invalidateQueries({
        queryKey: documentQueryKeys.byStage(dto.stage_id),
      });
      if (dto.task_id) {
        queryClient.invalidateQueries({
          queryKey: documentQueryKeys.byTask(dto.task_id),
        });
      }
    },
  });
}

export function useUpdateDocument() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ id, dto }: { id: string; dto: UpdateDocumentDto }) =>
      documentService.update(id, dto),
    onSuccess: () =>
      queryClient.invalidateQueries({ queryKey: documentQueryKeys.all }),
  });
}

export function useDeleteDocument() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: documentService.delete,
    onSuccess: () =>
      queryClient.invalidateQueries({ queryKey: documentQueryKeys.all }),
  });
}
