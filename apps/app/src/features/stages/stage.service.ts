import { api, apiRequest } from "@/api";

import type { CreateStageDto, Stage } from "./types";

export const stageService = {
  listByProject: (projectId: string) =>
    apiRequest<Stage[]>(api.get(`/stages/project/${projectId}`)),

  getById: (id: string) => apiRequest<Stage>(api.get(`/stages/${id}`)),

  create: (dto: CreateStageDto) =>
    apiRequest<Stage>(api.post("/stages", { data: dto })),

  reorder: (projectId: string, stages: string[]) =>
    apiRequest<null>(
      api.patch(`/stages/project/${projectId}/reorder`, {
        data: { stages },
      }),
    ),

  delete: (id: string) => apiRequest<null>(api.delete(`/stages/${id}`)),
};
