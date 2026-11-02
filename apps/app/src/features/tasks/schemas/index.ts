import { z } from "zod";

export const taskStatusSchema = z.enum([
  "pending",
  "in_progress",
  "completed",
  "blocked",
  "cancelled",
]);

export const createTaskSchema = z.object({
  stage_id: z.uuid(),
  title: z.string().trim().min(1),
  description: z.string().max(1000).nullable().optional(),
  weight: z.number().positive(),
  progress: z.number().min(0).max(100),
  status: taskStatusSchema,
  assigned_to: z.uuid().nullable().optional(),
  due_date: z.iso.date().nullable().optional(),
});

export const updateTaskSchema = createTaskSchema
  .omit({ stage_id: true })
  .partial();

export const reorderTasksSchema = z.object({
  tasks: z.array(z.uuid()).min(1),
});
