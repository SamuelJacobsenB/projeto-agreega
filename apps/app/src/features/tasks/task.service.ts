import { api, apiRequest } from "@/api";

import type { CreateTaskDto, Task } from "./types";

export const taskService = {
  listByStage: (stageId: string) =>
    apiRequest<Task[]>(api.get(`/tasks/stage/${stageId}`)),

  getById: (id: string) => apiRequest<Task>(api.get(`/tasks/${id}`)),

  create: (dto: CreateTaskDto) =>
    apiRequest<Task>(api.post("/tasks", { data: dto })),

  reorder: (stageId: string, tasks: string[]) =>
    apiRequest<null>(
      api.patch(`/tasks/stage/${stageId}/reorder`, {
        data: { tasks },
      }),
    ),

  delete: (id: string) => apiRequest<null>(api.delete(`/tasks/${id}`)),
};
