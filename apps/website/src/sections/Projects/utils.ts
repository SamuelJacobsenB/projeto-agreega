import type { Project, ProjectImage } from "../../types";

export type OrganizedProject = {
  project: Project;
  cover_url: string | null;
};

export function organizeProjects(
  projects: Project[],
  images: ProjectImage[],
): OrganizedProject[] {
  return projects.map((project) => {
    const cover = images.find(
      (image) => image.project_id === project.id && image.order === 0,
    );

    return {
      project,
      cover_url: cover?.url ?? null,
    };
  });
}

export function formatProjectDate(
  value: Date | string | null | undefined,
): string | null {
  if (!value) return null;

  const date = value instanceof Date ? value : new Date(value);

  if (Number.isNaN(date.getTime())) return null;

  return new Intl.DateTimeFormat("pt-BR", {
    day: "2-digit",
    month: "long",
    year: "numeric",
    timeZone: "UTC",
  }).format(date);
}

export function getStatusInformation(status: Project["status"]) {
  switch (status) {
    case "completed":
      return {
        label: "Concluído",
        color: "bg-emerald-600",
      };

    case "in_progress":
      return {
        label: "Em andamento",
        color: "bg-amber-500",
      };

    case "cancelled":
      return {
        label: "Cancelado",
        color: "bg-red-500",
      };

    default:
      return {
        label: "Em planejamento",
        color: "bg-gray-400",
      };
  }
}
