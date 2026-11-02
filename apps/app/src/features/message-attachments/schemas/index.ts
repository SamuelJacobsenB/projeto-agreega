import { z } from "zod";

export const createMessageAttachmentSchema = z.object({
  message_id: z.uuid(),
  file_id: z.uuid(),
});
