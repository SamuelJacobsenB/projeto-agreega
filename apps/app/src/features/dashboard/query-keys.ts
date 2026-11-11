export const dashboardQueryKeys = {
  all: ["dashboard"] as const,
  staff: (userId: string) => [...dashboardQueryKeys.all, "staff", userId] as const,
  client: (userId: string) => [...dashboardQueryKeys.all, "client", userId] as const,
};
