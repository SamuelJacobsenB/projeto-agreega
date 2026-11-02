export const stageQueryKeys = {
  all: ["stages"] as const,
  byProject: (projectId: string) =>
    [...stageQueryKeys.all, "project", projectId] as const,
  detail: (id: string) => [...stageQueryKeys.all, "detail", id] as const,
};
