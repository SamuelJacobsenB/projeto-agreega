import { z } from "zod";

export const createMessageSchema = z.object({
  project_id: z.uuid(),
  content: z.string().trim().min(1).max(2000),
});

export const updateMessageSchema = z.object({
  content: z.string().trim().min(1).max(2000),
});
