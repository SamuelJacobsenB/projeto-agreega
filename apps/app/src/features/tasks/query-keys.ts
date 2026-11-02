export const taskQueryKeys = {
  all: ["tasks"] as const,
  byStage: (stageId: string) => [...taskQueryKeys.all, "stage", stageId] as const,
  detail: (id: string) => [...taskQueryKeys.all, "detail", id] as const,
};
