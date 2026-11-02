import { api, apiRequest } from "@/api";

import type { CreateMessageDto, Message, UpdateMessageDto } from "./types";

export const messageService = {
  listByProject: (projectId: string) =>
    apiRequest<Message[]>(api.get(`/messages/project/${projectId}`)),

  create: (dto: CreateMessageDto) =>
    apiRequest<Message>(api.post("/messages", { data: dto })),

  update: (id: string, dto: UpdateMessageDto) =>
    apiRequest<Message>(api.patch(`/messages/${id}`, { data: dto })),

  delete: (id: string) => apiRequest<null>(api.delete(`/messages/${id}`)),
};
