import { Building2, CalendarDays, MapPin } from "lucide-react";
import { useState } from "react";

import { api } from "@/api";

import type { DashboardProject } from "../types";
import { DashboardEmptyState } from "./DashboardEmptyState";

type Props = {
  projects: DashboardProject[];
  isClient: boolean;
};

const projectStatusLabels: Record<DashboardProject["status"], string> = {
  planning: "Planejamento",
  in_progress: "Em andamento",
  paused: "Pausado",
  completed: "Concluído",
  cancelled: "Cancelado",
};

function formatDate(value: string) {
  return new Intl.DateTimeFormat("pt-BR", {
    day: "numeric",
    month: "short",
    year: "numeric",
    timeZone: "UTC",
  }).format(new Date(value));
}

export function DashboardProjectList({ projects, isClient }: Props) {
  if (projects.length === 0) {
    return (
      <DashboardEmptyState
        title={
          isClient
            ? "Nenhum projeto ativo ainda"
            : "Nenhum projeto em andamento"
        }
        description={
          isClient
            ? "Quando um projeto for iniciado, você poderá acompanhar o progresso por aqui."
            : "Os projetos ativos aparecerão aqui."
        }
      />
    );
  }

  return (
    <ul className="grid gap-4 p-4 sm:grid-cols-2">
      {projects.map((project) => (
        <li key={project.id}>
          <article className="h-full overflow-hidden rounded-xl border border-border bg-background">
            <ProjectCover project={project} />

            <div className="space-y-4 p-4">
              <div className="min-w-0">
                <h3 className="truncate font-semibold">{project.name}</h3>
                <p className="mt-1 flex items-center gap-1.5 text-sm text-muted-foreground">
                  <MapPin size={14} aria-hidden="true" />
                  <span className="truncate">
                    {project.city}, {project.state}
                  </span>
                </p>
              </div>

              <div>
                <div className="mb-2 flex items-center justify-between gap-3 text-xs">
                  <span className="text-muted-foreground">Progresso</span>
                  <span className="font-medium">
                    {project.progress === null
                      ? "Etapas não cadastradas"
                      : `${project.progress}%`}
                  </span>
                </div>
                <div
                  className="h-2 overflow-hidden rounded-full bg-muted"
                  role="progressbar"
                  aria-label={`Progresso de ${project.name}`}
                  aria-valuemin={0}
                  aria-valuemax={100}
                  aria-valuenow={project.progress ?? 0}
                  aria-valuetext={
                    project.progress === null
                      ? "Etapas não cadastradas"
                      : `${project.progress}% concluído`
                  }
                >
                  <div
                    className="h-full rounded-full bg-primary transition-[width]"
                    style={{
                      width: `${Math.min(100, Math.max(0, project.progress ?? 0))}%`,
                    }}
                  />
                </div>
              </div>

              {project.estimated_end_date && (
                <div className="flex items-center gap-2 border-t border-border pt-3">
                  <p className="flex items-center gap-1.5 text-xs text-muted-foreground">
                    <CalendarDays size={13} aria-hidden="true" />
                    Previsão: {formatDate(project.estimated_end_date)}
                  </p>
                </div>
              )}
            </div>
          </article>
        </li>
      ))}
    </ul>
  );
}

function ProjectCover({ project }: { project: DashboardProject }) {
  const [hasImage, setHasImage] = useState(true);
  const imageUrl = project.cover_image_url
    ? api.getUri({ url: project.cover_image_url })
    : null;

  return (
    <div className="relative aspect-video overflow-hidden bg-muted">
      {imageUrl && hasImage ? (
        <img
          src={imageUrl}
          alt={`Capa do projeto ${project.name}`}
          className="size-full object-cover"
          loading="lazy"
          onError={() => setHasImage(false)}
        />
      ) : (
        <div className="flex size-full items-center justify-center bg-linear-to-br from-primary/10 via-muted to-primary/20">
          <Building2
            size={40}
            className="text-muted-foreground/60"
            aria-hidden="true"
          />
        </div>
      )}
      <span className="absolute top-3 right-3 rounded-full bg-background/90 px-2.5 py-1 text-xs font-medium text-foreground shadow-sm backdrop-blur">
        {projectStatusLabels[project.status]}
      </span>
    </div>
  );
}
