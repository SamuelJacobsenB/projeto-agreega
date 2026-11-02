import type { z } from "zod";

import type { createDocumentVersionSchema } from "../schemas";

export type DocumentVersion = {
  id: string;
  document_id: string;
  file_id: string;
  version: number;
  uploaded_by: string;
  created_at: string;
};
export type CreateDocumentVersionDto = z.infer<
  typeof createDocumentVersionSchema
>;
