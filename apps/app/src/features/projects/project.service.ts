import { api, apiRequest } from "@/api";

import type { CreateProjectDto, Project, UpdateProjectDto } from "./types";

export const projectService = {
  list: () => apiRequest<Project[]>(api.get("/projects")),

  getById: (id: string) => apiRequest<Project>(api.get(`/projects/${id}`)),

  create: (dto: CreateProjectDto) =>
    apiRequest<Project>(api.post("/projects", dto)),

  update: (id: string, dto: UpdateProjectDto) =>
    apiRequest<Project>(api.put(`/projects/${id}`, dto)),

  delete: (id: string) => apiRequest<null>(api.delete(`/projects/${id}`)),
};
