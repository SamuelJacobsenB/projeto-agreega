import type { z } from "zod";

import type { createMessageSchema, updateMessageSchema } from "../schemas";

export type Message = {
  id: string;
  project_id: string;
  sender_id: string;
  content: string;
  created_at: string;
  edited_at: string | null;
};
export type CreateMessageDto = z.infer<typeof createMessageSchema>;
export type UpdateMessageDto = z.infer<typeof updateMessageSchema>;
