import { z } from "zod";

export const createPhotoSchema = z.object({
  project_id: z.uuid(),
  stage_id: z.uuid(),
  task_id: z.uuid().nullable().optional(),
  description: z.string().max(255).nullable().optional(),
});

export const updatePhotoDescriptionSchema = z.object({
  description: z.string().max(255),
});
