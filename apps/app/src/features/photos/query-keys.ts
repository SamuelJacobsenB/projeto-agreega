export const photoQueryKeys = {
  all: ["photos"] as const,
  byStage: (stageId: string) => [...photoQueryKeys.all, "stage", stageId] as const,
  byTask: (taskId: string) => [...photoQueryKeys.all, "task", taskId] as const,
  detail: (id: string) => [...photoQueryKeys.all, "detail", id] as const,
  file: (id: string) => [...photoQueryKeys.detail(id), "file"] as const,
};
