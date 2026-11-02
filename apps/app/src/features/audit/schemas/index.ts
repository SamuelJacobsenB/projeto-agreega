import { z } from "zod";

export const auditLogQuerySchema = z.object({
  user_id: z.uuid().optional(),
  entity: z.string().optional(),
  entity_id: z.uuid().optional(),
  limit: z.number().int().optional(),
  before_created_at: z.iso.datetime().optional(),
  before_id: z.uuid().optional(),
});
