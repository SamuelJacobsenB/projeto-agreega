import type { AuditLogQuery } from "./types";

export const auditQueryKeys = {
  all: ["audit-logs"] as const,
  list: (params: AuditLogQuery = {}) => [...auditQueryKeys.all, params] as const,
};
