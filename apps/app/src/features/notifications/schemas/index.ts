import { z } from "zod";

export const notificationTypeSchema = z.enum([
  "general",
  "project",
  "document",
  "approval",
  "message",
  "task",
  "payment",
  "system",
]);

export const notificationQuerySchema = z.object({
  limit: z.number().int().optional(),
  offset: z.number().int().optional(),
});

export const updateNotificationSchema = z.object({
  read: z.boolean(),
});
