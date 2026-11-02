import type { z } from "zod";

import type { auditLogQuerySchema } from "../schemas";

export type AuditLog = {
  id: string;
  user_id: string | null;
  action: string;
  entity: string;
  entity_id: string;
  metadata: unknown;
  created_at: string;
};
export type AuditLogQuery = z.infer<typeof auditLogQuerySchema>;
