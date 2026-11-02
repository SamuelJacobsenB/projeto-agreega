import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import { documentCommentService } from "../document-comment.service";
import { documentCommentQueryKeys } from "../query-keys";
import type {
  CreateDocumentCommentDto,
  UpdateDocumentCommentDto,
} from "../types";

export function useDocumentComments(versionId: string) {
  return useQuery({
    queryKey: documentCommentQueryKeys.byVersion(versionId),
    queryFn: () => documentCommentService.listByVersion(versionId),
    enabled: Boolean(versionId),
  });
}

export function useCreateDocumentComment() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({
      versionId,
      dto,
    }: {
      versionId: string;
      dto: CreateDocumentCommentDto;
    }) => documentCommentService.create(versionId, dto),
    onSuccess: (_response, { versionId }) =>
      queryClient.invalidateQueries({
        queryKey: documentCommentQueryKeys.byVersion(versionId),
      }),
  });
}

export function useUpdateDocumentComment() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({
      versionId,
      id,
      dto,
    }: {
      versionId: string;
      id: string;
      dto: UpdateDocumentCommentDto;
    }) => documentCommentService.update(versionId, id, dto),
    onSuccess: (_response, { versionId }) =>
      queryClient.invalidateQueries({
        queryKey: documentCommentQueryKeys.byVersion(versionId),
      }),
  });
}

export function useDeleteDocumentComment() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ versionId, id }: { versionId: string; id: string }) =>
      documentCommentService.delete(versionId, id),
    onSuccess: (_response, { versionId }) =>
      queryClient.invalidateQueries({
        queryKey: documentCommentQueryKeys.byVersion(versionId),
      }),
  });
}
