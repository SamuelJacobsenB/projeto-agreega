import { ArrowDownRight, MapPin, X } from "lucide-react";

import { Modal } from "../../../components/ui/Modal";
import type { Project } from "../../../types";
import { projectTypeLabels } from "../constants";
import { ProjectGallery } from "./ProjectGallery";
import { ProjectInformation } from "./ProjectInformation";

type ProjectModalProps = {
  project: Project | null;
  onClose: () => void;
};

export function ProjectModal({ project, onClose }: ProjectModalProps) {
  if (!project) return null;

  const typeLabel = projectTypeLabels[project.project_type] ?? "Projeto";

  const statusLabel =
    project.status === "in_progress"
      ? "Em andamento"
      : project.status === "completed"
        ? "Concluído"
        : project.status === "cancelled"
          ? "Cancelado"
          : "Em planejamento";

  return (
    <Modal
      isOpen={Boolean(project)}
      onClose={onClose}
      panelClassName="relative mx-auto min-h-screen w-full max-w-6xl bg-white text-(--color-light-black) shadow-2xl sm:min-h-0"
    >
      <button
        onClick={onClose}
        className="absolute p-2 right-10 top-10 border border-gray-200 rounded-sm duration-100 hover:bg-gray-100"
      >
        <X size={22} strokeWidth={1.5} />
      </button>

      <div className="p-5 sm:p-8 lg:p-12">
        <nav
          aria-label="Localização do projeto"
          className="mb-8 flex flex-wrap items-center gap-2 text-xs uppercase tracking-[0.14em] text-gray-500"
        >
          <span>Projetos</span>
          <span aria-hidden="true">/</span>
          <span>{typeLabel}</span>

          {project.status !== "completed" && (
            <>
              <span aria-hidden="true">/</span>
              <span>{statusLabel}</span>
            </>
          )}
        </nav>

        <div className="grid gap-8 lg:grid-cols-[minmax(0,1fr)_260px] lg:items-end">
          <div>
            <h2
              id="project-detail-title"
              className="max-w-4xl text-3xl font-light leading-tight tracking-tight sm:text-4xl lg:text-5xl"
            >
              {project.name}
            </h2>

            {(project.city || project.state) && (
              <p className="mt-4 flex items-center gap-2 text-sm text-gray-500">
                <MapPin size={16} strokeWidth={1.5} />

                {[project.city, project.state].filter(Boolean).join(", ")}
              </p>
            )}
          </div>

          <p className="text-sm leading-6 text-gray-500 lg:text-right">
            {typeLabel}
          </p>
        </div>

        <ProjectGallery projectId={project.id} projectName={project.name} />

        <div className="mt-10 grid gap-10 lg:grid-cols-[minmax(0,1fr)_300px] lg:gap-16">
          <section aria-labelledby="project-description-title">
            <h3
              id="project-description-title"
              className="text-xs font-medium uppercase tracking-[0.18em] text-gray-500"
            >
              Sobre o projeto
            </h3>

            <p className="mt-4 whitespace-pre-line text-base leading-8 text-gray-700">
              {project.description ||
                "Nenhuma descrição foi adicionada a este projeto."}
            </p>
          </section>

          <ProjectInformation project={project} />
        </div>

        <footer className="mt-12 flex flex-col justify-between gap-4 border-t border-gray-200 pt-6 sm:flex-row sm:items-center">
          <p className="text-sm text-gray-500">
            Quer conversar sobre um projeto semelhante?
          </p>

          <a
            href="#contato"
            onClick={onClose}
            className="inline-flex items-center justify-center gap-3 bg-(--color-light-black) px-5 py-3 text-sm text-white transition hover:opacity-85"
          >
            Vamos conversar
            <ArrowDownRight size={16} strokeWidth={1.5} />
          </a>
        </footer>
      </div>
    </Modal>
  );
}
