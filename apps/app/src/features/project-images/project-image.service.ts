import { api, apiRequest } from "@/api";

import type { CreateProjectImageDto, ProjectImage } from "./types";

export const projectImageService = {
  listByProject: (projectId: string) =>
    apiRequest<ProjectImage[]>(api.get(`/project-images/project/${projectId}`)),

  listCovers: () =>
    apiRequest<ProjectImage[]>(api.get("/project-images/covers")),

  getCoverByProject: (projectId: string) =>
    apiRequest<ProjectImage>(
      api.get(`/project-images/project/${projectId}/cover`),
    ),

  getById: (id: string) =>
    apiRequest<ProjectImage>(api.get(`/project-images/${id}`)),

  getFile: async (id: string) => {
    const response = await api.get<Blob>(`/project-images/${id}/file`, {
      responseType: "blob",
    });

    return response.data;
  },

  create: (dto: CreateProjectImageDto, file: File) => {
    const formData = new FormData();

    formData.append("data", JSON.stringify(dto));
    formData.append("file", file);

    return apiRequest<ProjectImage>(
      api.post("/project-images", formData, {
        headers: {
          "Content-Type": "multipart/form-data",
        },
      }),
    );
  },

  reorder: (projectId: string, images: string[]) =>
    apiRequest<null>(
      api.patch(`/project-images/project/${projectId}/reorder`, {
        images,
      }),
    ),

  delete: (id: string) => apiRequest<null>(api.delete(`/project-images/${id}`)),
};
