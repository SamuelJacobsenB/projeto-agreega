import type { ApiResponse, Project } from "../types";
import { api } from "./api";

export const projectService = {
  getAll: () => api.get<unknown, ApiResponse<Project[]>>("/projects"),
};
