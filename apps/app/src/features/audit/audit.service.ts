import { api, apiRequest } from "@/api";

import type { AuditLog, AuditLogQuery } from "./types";

export const auditService = {
  list: (query: AuditLogQuery = {}) =>
    apiRequest<AuditLog[]>(api.get("/audit-logs", { params: query })),
};
