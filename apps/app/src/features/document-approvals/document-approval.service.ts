import { api, apiRequest } from "@/api";

import type {
  CreateDocumentApprovalDto,
  DocumentApproval,
  ReviewDocumentApprovalDto,
} from "./types";

export const documentApprovalService = {
  listByDocument: (documentId: string) =>
    apiRequest<DocumentApproval[]>(
      api.get(`/document-approvals/document/${documentId}`),
    ),

  getById: (id: string) =>
    apiRequest<DocumentApproval>(api.get(`/document-approvals/${id}`)),

  create: (dto: CreateDocumentApprovalDto) =>
    apiRequest<DocumentApproval>(
      api.post("/document-approvals", { data: dto }),
    ),

  review: (id: string, dto: ReviewDocumentApprovalDto) =>
    apiRequest<DocumentApproval>(
      api.put(`/document-approvals/${id}/review`, { data: dto }),
    ),
};
