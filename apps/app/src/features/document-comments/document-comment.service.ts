import { api, apiRequest } from "@/api";

import type {
  CreateDocumentCommentDto,
  DocumentComment,
  UpdateDocumentCommentDto,
} from "./types";

export const documentCommentService = {
  listByVersion: (versionId: string) =>
    apiRequest<DocumentComment[]>(
      api.get(`/document-versions/document-version/${versionId}/comments`),
    ),

  create: (versionId: string, dto: CreateDocumentCommentDto) =>
    apiRequest<DocumentComment>(
      api.post(`/document-versions/document-version/${versionId}/comments`, {
        data: dto,
      }),
    ),

  update: (versionId: string, id: string, dto: UpdateDocumentCommentDto) =>
    apiRequest<DocumentComment>(
      api.put(
        `/document-versions/document-version/${versionId}/comments/${id}`,
        { data: dto },
      ),
    ),

  delete: (versionId: string, id: string) =>
    apiRequest<null>(
      api.delete(
        `/document-versions/document-version/${versionId}/comments/${id}`,
      ),
    ),
};
