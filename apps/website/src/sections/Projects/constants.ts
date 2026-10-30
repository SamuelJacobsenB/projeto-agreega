import type { ProjectType } from "../../types";

export type ProjectFilter = ProjectType | "all";

export const projectFilters: {
  label: string;
  value: ProjectFilter;
}[] = [
  { label: "Todos", value: "all" },
  { label: "Arquitetura", value: "architecture" },
  { label: "Engenharia", value: "engineering" },
  {
    label: "Arquitetura + Engenharia",
    value: "architecture_and_engineering",
  },
  { label: "Consultoria", value: "consulting" },
];

export const projectTypeLabels: Record<ProjectType, string> = {
  architecture: "Arquitetura",
  engineering: "Engenharia",
  architecture_and_engineering: "Arquitetura e Engenharia",
  consulting: "Consultoria",
  other: "Outros projetos",
};

export const projectStatusLabels = {
  completed: "Concluído",
  in_progress: "Em andamento",
  planned: "Em planejamento",
  cancelled: "Cancelado",
} as const;
