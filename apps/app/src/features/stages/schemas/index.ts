import { z } from "zod";

export const stageStatusSchema = z.enum([
  "pending",
  "in_progress",
  "completed",
  "blocked",
  "cancelled",
]);

export const createStageSchema = z.object({
  project_id: z.uuid(),
  name: z.string().trim().min(1),
  description: z.string().max(1000).nullable().optional(),
  weight: z.number().positive(),
  status: stageStatusSchema,
  start_date: z.iso.date().nullable().optional(),
  end_date: z.iso.date().nullable().optional(),
});

export const updateStageSchema = createStageSchema
  .omit({ project_id: true })
  .partial();

export const reorderStagesSchema = z.object({
  stages: z.array(z.uuid()).min(1),
});
