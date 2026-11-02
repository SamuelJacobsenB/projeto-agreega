import type { z } from "zod";

import type {
  createDocumentSchema,
  documentStatusSchema,
  documentTypeSchema,
  updateDocumentSchema,
} from "../schemas";

export type DocumentType = z.infer<typeof documentTypeSchema>;
export type DocumentStatus = z.infer<typeof documentStatusSchema>;
export type Document = {
  id: string;
  project_id: string;
  stage_id: string;
  task_id: string | null;
  name: string;
  document_type: DocumentType;
  status: DocumentStatus;
  current_version: number;
  created_by: string;
  created_at: string;
  updated_at: string;
};
export type CreateDocumentDto = z.infer<typeof createDocumentSchema>;
export type UpdateDocumentDto = z.infer<typeof updateDocumentSchema>;
