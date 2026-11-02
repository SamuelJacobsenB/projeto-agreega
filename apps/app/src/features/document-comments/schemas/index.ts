import { z } from "zod";

export const createDocumentCommentSchema = z.object({
  content: z.string().trim().min(1).max(500),
  page: z.number().int().min(1).nullable().optional(),
  position_x: z.number().min(0).nullable().optional(),
  position_y: z.number().min(0).nullable().optional(),
});

export const updateDocumentCommentSchema =
  createDocumentCommentSchema.partial();
