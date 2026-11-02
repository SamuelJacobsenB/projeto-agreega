import { api, apiRequest } from "@/api";

import type { CreateMessageAttachmentDto, MessageAttachment } from "./types";

export const messageAttachmentService = {
  create: (dto: CreateMessageAttachmentDto) =>
    apiRequest<MessageAttachment>(
      api.post("/message-attachments", { data: dto }),
    ),

  listByMessage: (messageId: string) =>
    apiRequest<MessageAttachment[]>(
      api.get(`/message-attachments/message/${messageId}`),
    ),

  getFile: async (messageId: string, fileId: string) => {
    const response = await api.get<Blob>(
      `/message-attachments/message/${messageId}/file/${fileId}`,
      { responseType: "blob" },
    );

    return response.data;
  },

  delete: (messageId: string, fileId: string) =>
    apiRequest<null>(
      api.delete(`/message-attachments/message/${messageId}/file/${fileId}`),
    ),
};
