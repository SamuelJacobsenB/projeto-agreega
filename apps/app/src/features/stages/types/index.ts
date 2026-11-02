import type { z } from "zod";

import type {
  createStageSchema,
  reorderStagesSchema,
  stageStatusSchema,
  updateStageSchema,
} from "../schemas";

export type StageStatus = z.infer<typeof stageStatusSchema>;
export type Stage = {
  id: string;
  project_id: string;
  name: string;
  description: string | null;
  weight: number;
  order: number;
  status: StageStatus;
  start_date: string | null;
  end_date: string | null;
  created_at: string;
  updated_at: string;
};
export type CreateStageDto = z.infer<typeof createStageSchema>;
export type UpdateStageDto = z.infer<typeof updateStageSchema>;
export type ReorderStagesDto = z.infer<typeof reorderStagesSchema>;
