import type { z } from "zod";

import type { createPhotoSchema } from "../schemas";

export type Photo = {
  id: string;
  project_id: string;
  stage_id: string;
  task_id: string | null;
  file_id: string;
  description: string | null;
  url: string;
  uploaded_by: string;
  created_at: string;
};
export type CreatePhotoDto = z.infer<typeof createPhotoSchema>;
