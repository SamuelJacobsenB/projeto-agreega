import { api, apiRequest } from "@/api";

import type { Notification, NotificationQuery } from "./types";

export const notificationService = {
  list: (query: NotificationQuery = {}) =>
    apiRequest<Notification[]>(
      api.get("/notifications", {
        params: query,
      }),
    ),

  updateReadState: (id: string, read: boolean) =>
    apiRequest<Notification>(
      api.patch(`/notifications/${id}`, { read }),
    ),

  delete: (id: string) => apiRequest<null>(api.delete(`/notifications/${id}`)),
};
