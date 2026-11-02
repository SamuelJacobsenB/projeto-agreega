import type { z } from "zod";

import type { createProjectImageSchema } from "../schemas";

export type ProjectImage = {
  id: string;
  project_id: string;
  file_id: string;
  url: string;
  order: number;
  created_at: string;
  updated_at: string;
};
export type CreateProjectImageDto = z.infer<typeof createProjectImageSchema>;
