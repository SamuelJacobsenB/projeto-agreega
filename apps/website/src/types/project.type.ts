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
  start_date: Date | null;
  estimated_end_date: Date | null;
  completed_at: Date | null;
  cover_file_id: string | null;
  created_at: Date;
  updated_at: Date;
};

type ProjectType =
  | "architecture"
  | "engineering"
  | "architecture_and_engineering"
  | "consulting"
  | "other";

type ProjectStatus =
  | "planning"
  | "in_progress"
  | "paused"
  | "completed"
  | "cancelled";
