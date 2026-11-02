import { z } from "zod";

export const createDocumentVersionSchema = z.object({
  document_id: z.uuid(),
});
