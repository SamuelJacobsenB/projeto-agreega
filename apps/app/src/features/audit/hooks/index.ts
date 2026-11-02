import { useQuery } from "@tanstack/react-query";

import { auditService } from "../audit.service";
import { auditQueryKeys } from "../query-keys";
import type { AuditLogQuery } from "../types";

export function useAuditLogs(params: AuditLogQuery = {}) {
  return useQuery({
    queryKey: auditQueryKeys.list(params),
    queryFn: () => auditService.list(params),
  });
}
