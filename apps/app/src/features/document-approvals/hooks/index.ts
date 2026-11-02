import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import { documentApprovalService } from "../document-approval.service";
import { documentApprovalQueryKeys } from "../query-keys";
import type {
  CreateDocumentApprovalDto,
  ReviewDocumentApprovalDto,
} from "../types";

export function useDocumentApprovals(documentId: string) {
  return useQuery({
    queryKey: documentApprovalQueryKeys.byDocument(documentId),
    queryFn: () => documentApprovalService.listByDocument(documentId),
    enabled: Boolean(documentId),
  });
}

export function useDocumentApproval(id: string) {
  return useQuery({
    queryKey: documentApprovalQueryKeys.detail(id),
    queryFn: () => documentApprovalService.getById(id),
    enabled: Boolean(id),
  });
}

export function useCreateDocumentApproval() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (dto: CreateDocumentApprovalDto) =>
      documentApprovalService.create(dto),
    onSuccess: (_response, dto) =>
      queryClient.invalidateQueries({
        queryKey: documentApprovalQueryKeys.byDocument(dto.document_id),
      }),
  });
}

export function useReviewDocumentApproval() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ id, dto }: { id: string; dto: ReviewDocumentApprovalDto }) =>
      documentApprovalService.review(id, dto),
    onSuccess: (_response, { id }) => {
      queryClient.invalidateQueries({
        queryKey: documentApprovalQueryKeys.all,
      });
      queryClient.invalidateQueries({
        queryKey: documentApprovalQueryKeys.detail(id),
      });
    },
  });
}
