import type { z } from "zod";

import type { createProjectSchema, updateProjectSchema } from "../schemas";

export type ProjectType =
  | "architecture"
  | "engineering"
  | "architecture_and_engineering"
  | "consulting"
  | "other";
export type ProjectStatus =
  | "planning"
  | "in_progress"
  | "paused"
  | "completed"
  | "cancelled";

export type Project = {
  id: string;
  client_id: string;
  name: string;
  description: string | null;
  project_type: ProjectType;
  status: ProjectStatus;
  address: string | null;
  city: string;
  state: string;
  start_date: string | null;
  estimated_end_date: string | null;
  completed_at: string | null;
  cover_file_id: string | null;
  created_at: string;
  updated_at: string;
};

export type CreateProjectDto = z.infer<typeof createProjectSchema>;
export type UpdateProjectDto = z.infer<typeof updateProjectSchema>;