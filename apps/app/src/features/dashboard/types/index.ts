export type DashboardResponse = {
  statistics: DashboardStatistics;
  projects: DashboardProject[];
  tasks: DashboardTask[];
};

export type DashboardProject = {
  id: string;
  name: string;
  status: "planning" | "in_progress" | "paused" | "completed" | "cancelled";
  city: string;
  state: string;
  estimated_end_date: string | null;
  progress: number | null;
  cover_image_url: string | null;
};

export type DashboardTask = {
  id: string;
  title: string;
  status: "pending" | "in_progress" | "blocked";
  due_date: string | null;
  is_overdue: boolean;
  stage_name: string;
  project_name: string;
};

export type DashboardStatistics = {
  active_projects: number;
  pending_tasks: number;
  overdue_tasks: number;
  completed_projects: number;
};

export type StatisticCard = {
  title: string;
  value: number;
  description: string;
  icon: React.ElementType;
  alert?: boolean;
};
