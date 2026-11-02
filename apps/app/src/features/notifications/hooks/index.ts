import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import { notificationService } from "../notification.service";
import { notificationQueryKeys } from "../query-keys";
import type { NotificationQuery } from "../types";

export function useNotifications(params: NotificationQuery = {}) {
  return useQuery({
    queryKey: notificationQueryKeys.list(params),
    queryFn: () => notificationService.list(params),
  });
}

export function useUpdateNotificationReadState() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ id, read }: { id: string; read: boolean }) =>
      notificationService.updateReadState(id, read),
    onSuccess: () =>
      queryClient.invalidateQueries({ queryKey: notificationQueryKeys.all }),
  });
}

export function useDeleteNotification() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: notificationService.delete,
    onSuccess: () =>
      queryClient.invalidateQueries({ queryKey: notificationQueryKeys.all }),
  });
}
