import type { z } from "zod";

import type {
  createDocumentCommentSchema,
  updateDocumentCommentSchema,
} from "../schemas";

export type DocumentComment = {
  id: string;
  document_version_id: string;
  user_id: string;
  content: string;
  page: number | null;
  position_x: number | null;
  position_y: number | null;
  created_at: string;
  updated_at: string | null;
};
export type CreateDocumentCommentDto = z.infer<
  typeof createDocumentCommentSchema
>;
export type UpdateDocumentCommentDto = z.infer<
  typeof updateDocumentCommentSchema
>;
