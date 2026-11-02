export const documentQueryKeys = {
  all: ["documents"] as const,
  byStage: (stageId: string) =>
    [...documentQueryKeys.all, "stage", stageId] as const,
  byTask: (taskId: string) =>
    [...documentQueryKeys.all, "task", taskId] as const,
  detail: (id: string) => [...documentQueryKeys.all, "detail", id] as const,
};
