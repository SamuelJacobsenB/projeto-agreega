import { api, apiRequest } from "@/api";

import type { DashboardResponse } from "./types";

export const dashboardService = {
  get: () => apiRequest<DashboardResponse>(api.get("/dashboard")),
};
