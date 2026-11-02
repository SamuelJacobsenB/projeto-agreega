import type { z } from "zod";

import type {
  notificationQuerySchema,
  notificationTypeSchema,
  updateNotificationSchema,
} from "../schemas";

export type NotificationType = z.infer<typeof notificationTypeSchema>;
export type Notification = {
  id: string;
  user_id: string;
  title: string;
  message: string;
  notification_type: NotificationType;
  read_at: string | null;
  created_at: string;
};
export type NotificationQuery = z.infer<typeof notificationQuerySchema>;
export type UpdateNotificationDto = z.infer<
  typeof updateNotificationSchema
>;
