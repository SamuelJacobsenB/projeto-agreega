import type { ApiResponse, ProjectImage } from "../types";
import { api } from "./api";

export const projectImageService = {
  getCovers: () =>
    api.get<unknown, ApiResponse<ProjectImage[]>>("/project-images/covers"),
  getByProjectId: (project_id: string) =>
    api.get<unknown, ApiResponse<ProjectImage[]>>(
      `/project-images/project/${project_id}`,
    ),
};
