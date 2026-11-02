import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import { documentQueryKeys } from "@/features/documents/query-keys";
import { documentVersionService } from "../document-version.service";
import { documentVersionQueryKeys } from "../query-keys";

export function useDocumentVersions(documentId: string) {
  return useQuery({
    queryKey: documentVersionQueryKeys.byDocument(documentId),
    queryFn: () => documentVersionService.listByDocument(documentId),
    enabled: Boolean(documentId),
  });
}

export function useDocumentVersion(id: string) {
  return useQuery({
    queryKey: documentVersionQueryKeys.detail(id),
    queryFn: () => documentVersionService.getById(id),
    enabled: Boolean(id),
  });
}

export function useDocumentVersionFile(id: string) {
  return useQuery({
    queryKey: [...documentVersionQueryKeys.detail(id), "file"],
    queryFn: () => documentVersionService.getFile(id),
    enabled: Boolean(id),
  });
}

export function useCreateDocumentVersion() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ documentId, file }: { documentId: string; file: File }) =>
      documentVersionService.create(documentId, file),
    onSuccess: (_response, { documentId }) => {
      queryClient.invalidateQueries({
        queryKey: documentVersionQueryKeys.byDocument(documentId),
      });
      queryClient.invalidateQueries({ queryKey: documentQueryKeys.all });
    },
  });
}
