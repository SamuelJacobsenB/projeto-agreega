import { ArrowUpRight, MapPin } from "lucide-react";

import type { Project } from "../../../types";
import { projectTypeLabels } from "../constants";

type ProjectCardProps = {
  project: Project;
  cover_url: string | null;
  featured?: boolean;
  onSelect: (project: Project) => void;
};

export function ProjectCard({
  project,
  cover_url,
  featured = false,
  onSelect,
}: ProjectCardProps) {
  const statusLabel =
    project.status === "completed"
      ? null
      : project.status === "in_progress"
        ? "Em andamento"
        : project.status === "cancelled"
          ? "Cancelado"
          : "Em planejamento";

  return (
    <button
      type="button"
      onClick={() => onSelect(project)}
      aria-label={`Ver detalhes do projeto ${project.name}`}
      className={`group relative isolate flex w-full cursor-pointer overflow-hidden bg-gray-100 text-left outline-none focus-visible:ring-2 focus-visible:ring-inset focus-visible:ring-(--color-light-black) ${
        featured ? "min-h-115 md:min-h-145" : "min-h-85 md:min-h-80"
      }`}
    >
      {cover_url ? (
        <img
          src={cover_url}
          alt=""
          loading="lazy"
          decoding="async"
          className="absolute inset-0 -z-20 h-full w-full object-cover transition-transform duration-700 group-hover:scale-[1.04]"
        />
      ) : (
        <div
          aria-hidden="true"
          className="absolute inset-0 -z-20 bg-linear-to-br from-gray-200 via-gray-300 to-gray-100"
        />
      )}

      <div
        aria-hidden="true"
        className="absolute inset-0 -z-10 bg-linear-to-t from-black/85 via-black/25 to-black/5"
      />

      <div className="flex w-full flex-col justify-between gap-12 p-6 text-white md:p-8">
        <div className="flex items-start justify-between gap-4">
          <div className="flex flex-wrap gap-2">
            <span className="border border-white/20 bg-black/10 px-3 py-2 text-[10px] uppercase tracking-[0.16em] backdrop-blur-sm">
              {projectTypeLabels[project.project_type] ?? "Projeto"}
            </span>

            {statusLabel && (
              <span className="border border-white/20 bg-black/10 px-3 py-2 text-[10px] uppercase tracking-[0.16em] backdrop-blur-sm">
                {statusLabel}
              </span>
            )}
          </div>

          <span
            aria-hidden="true"
            className="flex h-10 w-10 shrink-0 items-center justify-center rounded-full border border-white/60 transition-colors duration-300 group-hover:bg-white group-hover:text-(--color-light-black)"
          >
            <ArrowUpRight size={18} strokeWidth={1.5} />
          </span>
        </div>

        <div>
          {(project.city || project.state) && (
            <div className="mb-4 flex items-center gap-2 text-xs text-white/75">
              <MapPin size={14} strokeWidth={1.5} />
              <span>
                {[project.city, project.state].filter(Boolean).join(", ")}
              </span>
            </div>
          )}

          <h3
            className={`max-w-xl font-medium leading-tight tracking-tight ${
              featured ? "text-3xl md:text-5xl" : "text-2xl md:text-3xl"
            }`}
          >
            {project.name}
          </h3>

          {project.description && (
            <p className="mt-3 max-w-lg text-sm leading-6 text-white/75">
              {project.description}
            </p>
          )}

          <div className="mt-6 flex items-center gap-3 text-xs uppercase tracking-[0.16em]">
            <span className="h-px w-8 bg-white/70 transition-all duration-300 group-hover:w-12" />
            Projeto Agreega
          </div>
        </div>
      </div>
    </button>
  );
}
