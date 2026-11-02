import { api, apiRequest } from "@/api";

import type { DocumentVersion } from "./types";

export const documentVersionService = {
  listByDocument: (documentId: string) =>
    apiRequest<DocumentVersion[]>(
      api.get(`/document-versions/document/${documentId}`),
    ),

  getById: (id: string) =>
    apiRequest<DocumentVersion>(api.get(`/document-versions/${id}`)),

  getFile: async (id: string) => {
    const response = await api.get<Blob>(`/document-versions/${id}/file`, {
      responseType: "blob",
    });

    return response.data;
  },

  create: (documentId: string, file: File) => {
    const formData = new FormData();

    formData.append("data", JSON.stringify({ document_id: documentId }));
    formData.append("file", file);

    return apiRequest<DocumentVersion>(
      api.post("/document-versions", formData, {
        headers: {
          "Content-Type": "multipart/form-data",
        },
      }),
    );
  },
};
