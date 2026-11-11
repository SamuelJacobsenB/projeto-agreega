import { useQuery } from "@tanstack/react-query";

import { useAuth } from "@/hooks";

import { dashboardService } from "../dashboard.service";
import { dashboardQueryKeys } from "../query-keys";

export function useDashboard() {
  const { session } = useAuth();

  const role = session?.user?.role;
  const userId = session?.user?.id;
  const isClient = role === "client";

  return useQuery({
    queryKey: isClient
      ? dashboardQueryKeys.client(userId ?? "")
      : dashboardQueryKeys.staff(userId ?? ""),

    queryFn: dashboardService.get,

    enabled: !!role && !!userId,
  });
}
