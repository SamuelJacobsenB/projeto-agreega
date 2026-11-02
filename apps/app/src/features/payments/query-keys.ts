export const paymentQueryKeys = {
  all: ["payments"] as const,
  byProject: (projectId: string) =>
    [...paymentQueryKeys.all, "project", projectId] as const,
};
