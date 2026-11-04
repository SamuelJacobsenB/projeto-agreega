import { api, apiRequest } from "@/api";

import type { CreatePhotoDto, Photo } from "./types";

export const photoService = {
  listByStage: (stageId: string) =>
    apiRequest<Photo[]>(api.get(`/photos/stage/${stageId}`)),

  listByTask: (taskId: string) =>
    apiRequest<Photo[]>(api.get(`/photos/task/${taskId}`)),

  getById: (id: string) => apiRequest<Photo>(api.get(`/photos/${id}`)),

  getFile: async (id: string) => {
    const response = await api.get<Blob>(`/photos/${id}/file`, {
      responseType: "blob",
    });

    return response.data;
  },

  create: (dto: CreatePhotoDto, file: File) => {
    const formData = new FormData();

    formData.append("data", JSON.stringify(dto));
    formData.append("file", file);

    return apiRequest<Photo>(
      api.post("/photos", formData, {
        headers: {
          "Content-Type": "multipart/form-data",
        },
      }),
    );
  },

  updateDescription: (id: string, description: string) =>
    apiRequest<Photo>(api.patch(`/photos/${id}`, { description })),

  delete: (id: string) => apiRequest<null>(api.delete(`/photos/${id}`)),
};
