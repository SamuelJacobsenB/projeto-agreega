import type { z } from "zod";

import type {
  createTaskSchema,
  reorderTasksSchema,
  taskStatusSchema,
  updateTaskSchema,
} from "../schemas";

export type TaskStatus = z.infer<typeof taskStatusSchema>;
export type Task = {
  id: string;
  stage_id: string;
  title: string;
  description: string | null;
  weight: number;
  progress: number;
  order: number;
  status: TaskStatus;
  assigned_to: string | null;
  due_date: string | null;
  completed_at: string | null;
  created_at: string;
  updated_at: string;
};
export type CreateTaskDto = z.infer<typeof createTaskSchema>;
export type UpdateTaskDto = z.infer<typeof updateTaskSchema>;
export type ReorderTasksDto = z.infer<typeof reorderTasksSchema>;
