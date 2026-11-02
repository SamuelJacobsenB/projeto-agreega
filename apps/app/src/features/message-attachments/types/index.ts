import type { z } from "zod";

import type { createMessageAttachmentSchema } from "../schemas";

export type MessageAttachment = {
  message_id: string;
  file_id: string;
};
export type CreateMessageAttachmentDto = z.infer<
  typeof createMessageAttachmentSchema
>;
