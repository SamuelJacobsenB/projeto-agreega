import { api, apiRequest } from "@/api";

import type { CreateDocumentDto, Document, UpdateDocumentDto } from "./types";

export const documentService = {
  listByStage: (stageId: string) =>
    apiRequest<Document[]>(api.get(`/documents/stage/${stageId}`)),

  listByTask: (taskId: string) =>
    apiRequest<Document[]>(api.get(`/documents/task/${taskId}`)),

  getById: (id: string) => apiRequest<Document>(api.get(`/documents/${id}`)),

  create: (dto: CreateDocumentDto) =>
    apiRequest<Document>(api.post("/documents", { data: dto })),

  update: (id: string, dto: UpdateDocumentDto) =>
    apiRequest<Document>(api.put(`/documents/${id}`, { data: dto })),

  delete: (id: string) => apiRequest<null>(api.delete(`/documents/${id}`)),
};
