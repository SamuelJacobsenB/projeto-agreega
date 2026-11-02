import { z } from "zod";

export const createProjectImageSchema = z.object({
  project_id: z.uuid(),
});

export const reorderProjectImagesSchema = z.object({
  images: z.array(z.uuid()).min(1),
});
